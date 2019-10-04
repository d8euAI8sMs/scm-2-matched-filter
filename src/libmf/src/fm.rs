extern crate rand;
extern crate num;

mod reg;

use num::Zero;
use num::complex::Complex64;

#[derive(Clone)]
pub struct Pt {
    pub x: f64,
    pub y: f64,
}

pub struct Params {
    pub n: u32,
    pub sample_rate: f64,
    pub bit_rate: f64,
    pub snr: f64,
}

pub struct Demo {
    pub i: Vec<Pt>,
    pub q: Vec<Pt>,
    pub f: [Vec<Pt>; 4],
    pub e: f64,
}

impl Demo {
    fn new() -> Demo {
        Demo {
            i: Vec::new(),
            q: Vec::new(),
            f: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            e: 0.0
        }
    }
}

pub fn demo(p: Params) -> Demo {
    let mut r = Demo::new();

    let symbs = gen_symbs((p.n / 2) as usize);

    let mut iq = gen_iq(&p, &symbs);
    noisify_complex(&mut iq, p.snr);
    split_complex(&p, &iq, &mut r.i, &mut r.q);
    let res = apply_filter(&p, &iq);
    for (i, f) in r.f.iter_mut().enumerate() {
        timify(&p, &res[i], f);
    }

    let dec = decode(&p, &res);

    r.e = compare(&symbs, &dec);

    r
}

fn compare(v1: &Vec<u8>, v2: &Vec<u8>) -> f64 {
    let n = v1.len().min(v2.len());
    let n0 = v1.iter()
        .zip(v2.iter())
        .filter(|(a, b)| a == b)
        .count();
    (n0 as f64) / (n as f64)
}

fn noisify_complex(v: &mut Vec<Complex64>, snr: f64) {
    let mut n = Vec::new();
    n.resize_with(v.len(), rand_std_complex);

    let es2en = (snr / 10.).exp();

    let en0: f64 = n.iter().map(|x| x.norm_sqr()).sum();
    let es: f64 = v.iter().map(|p| p.norm_sqr()).sum();

    let en = es / es2en;

    for (i, p) in v.iter_mut().enumerate() {
        *p += (en / en0) * n[i];
    }
}

fn rand_std() -> f64 {
    (0..12).map(|_| rand::random::<f64>()).map(|x| x - 0.5).sum()
}

fn rand_std_complex() -> Complex64 {
    Complex64::new(rand_std(), rand_std())
}

fn gen_iq(p: &Params, symbs: &Vec<u8>) -> Vec<Complex64> {
    let gs = make_gold_seqs();

    let data = symbs.iter()
        .flat_map(|s| &gs[*s as usize])
        .map(|&s| s)
        .collect();

    qpsk(p, &data)
}

fn split_complex(p: &Params, v: &Vec<Complex64>, i: &mut Vec<Pt>, q: &mut Vec<Pt>) {
    i.reserve(v.len());
    q.reserve(v.len());

    for (j, c) in v.iter().enumerate() {
        let t = (j as f64) / p.sample_rate;
        i.push(Pt { x: t, y: c.re });
        q.push(Pt { x: t, y: c.im });
    }
}

fn timify(p: &Params, v: &Vec<f64>, r: &mut Vec<Pt>) {
    r.extend(v.iter().enumerate()
        .map(|(i, y)| Pt { x: (i as f64) / p.sample_rate, y: *y })
    )
}

fn gen_symbs(n: usize) -> Vec<u8> {
    let mut v = Vec::new();
    v.resize_with(n, rand_symb);
    v
}

fn rand_symb() -> u8 {
    let a = rand::random::<bool>() as u8;
    let b = rand::random::<bool>() as u8;
    (a << 1) | b
}

fn make_gold_seqs() -> Vec<Vec<bool>> {
    let (_, seq1) = reg::Reg31::new(0b10100).collect();
    let (_, seq2) = reg::Reg31::new(0b11110).collect();

    (0..4)
        .map(|i| seq1 ^ reg::Reg31::rot(seq2, i))
        .map(|s| bits32(s))
        .collect()
}

fn make_gold_filters(p: &Params, s: &Vec<Vec<bool>>) -> Vec<Vec<Complex64>> {
    s.iter().map(|s| qpsk(p, s)).collect()
}

fn bits32(n: u32) -> Vec<bool> {
    let mut v = Vec::new();
    v.resize(32, false);
    for i in 0..31 {
        v[i] = ((n >> i) & 1) == 1
    }
    v
}

fn qpsk(p: &Params, d: &Vec<bool>) -> Vec<Complex64> {
    let symb_br = p.bit_rate / 2.0;
    let symb_t = 1.0 / symb_br;
    let symb_n = (d.len() / 2) as usize;
    let n = ((symb_t * (symb_n as f64)) * p.sample_rate).floor() as usize;

    let mut res = Vec::new();
    res.reserve(n);

    for i in 0..n {
        let t = (i as f64) / p.sample_rate;
        let symb_i = (t * symb_br).floor() as usize;
        let (a, b) = (d[2 * symb_i] as i32, d[2 * symb_i + 1] as i32);
        res.push(Complex64::new(
            (a * 2 - 1) as f64,
            (b * 2 - 1) as f64
        ));
    }

    res
}

fn correlate(s1: &[Complex64], s2: &[Complex64]) -> f64 {
    let mut res = Complex64::zero();
    for (i, c1) in s1.iter().enumerate() {
        res += c1 * s2[i].conj();
    }
    res.re
}

fn correlate_multi(s1: &[Complex64], s2: &[Complex64]) -> Vec<f64> {
    let len = s1.len() - s2.len();
    (0..len).map(|i| {
        correlate(&s1[i..(i + s2.len())], s2)
    }).collect()
}

fn apply_filter(p: &Params, d: &Vec<Complex64>) -> Vec<Vec<f64>> {
    make_gold_filters(p, &make_gold_seqs()).iter()
        .map(|gsi| correlate_multi(&d, &gsi)).collect()
}

fn filter_max(p: &Params, v: &Vec<f64>, t: f64) -> Vec<f64> {
    let l = t * p.sample_rate;
    let n = ((v.len() as f64) / l).ceil() as usize;
    let ln = l.floor() as usize;

    let mut res = Vec::new();
    res.resize(n, 0.0);

    for i in 0..res.len() {
        let j = ((i as f64) * l).round() as usize;
        let j1 = if i + 1 == 1         { j } else { j - ln / 2};
        let j2 = if i + 1 == res.len() { j } else { j + ln / 2};
        res[i] = v[j1..j2].iter().map(|&x| x).max_by(cmp_f64).unwrap();
    }

    res
}

fn decode(p: &Params, f: &Vec<Vec<f64>>) -> Vec<u8> {
    let gbit_t = 1.0 / p.bit_rate;
    let symb_t = 32.0 * gbit_t;

    let wnd_t = symb_t / 2.0;
    
    let vecs: Vec<_> = f.iter().map(|v| filter_max(&p, v, wnd_t)).collect();

    let mut res = Vec::new();
    res.resize(vecs[0].len() / 2, 0_u8);

    for i in 0..res.len() {
        let mut j = 0_u8; let mut t = vecs[0][2 * i];
        if t < vecs[1][2 * i] { j = 1_u8; t = vecs[1][2 * i]; };
        if t < vecs[2][2 * i] { j = 2_u8; t = vecs[2][2 * i]; };
        if t < vecs[3][2 * i] { j = 3_u8; };
        res[i] = j;
    }

    res
}

fn cmp_f64(a: &f64, b: &f64) -> std::cmp::Ordering {
    if a < b {
        return std::cmp::Ordering::Less;
    } else if a > b {
        return std::cmp::Ordering::Greater;
    }
    return std::cmp::Ordering::Equal;
}
