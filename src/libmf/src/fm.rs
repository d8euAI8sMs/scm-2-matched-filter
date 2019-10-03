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
}

impl Demo {
    fn new() -> Demo {
        Demo {
            i: Vec::new(),
            q: Vec::new(),
            f: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
        }
    }
}

pub fn demo(p: Params) -> Demo {
    let mut r = Demo::new();

    let mut iq = gen_iq(&p);
    noisify_complex(&mut iq, p.snr);
    split_complex(&p, &iq, &mut r.i, &mut r.q);
    let res = apply_filter(&p, &iq);
    for (i, f) in r.f.iter_mut().enumerate() {
        timify(&p, &res[i], f);
    }
    r
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

fn gen_iq(p: &Params) -> Vec<Complex64> {
    let gs = make_gold_seqs();

    let symb_n = (p.n / 2) as usize;

    let symbs = gen_symbs(symb_n);

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

    (0..3)
        .map(|i| seq1 ^ reg::Reg31::rot(seq2, i))
        .map(|s| bits31(s))
        .collect()
}

fn make_gold_filters(p: &Params, s: &Vec<Vec<bool>>) -> Vec<Vec<Complex64>> {
    s.iter().map(|s| {
        let mut s0 = s.clone();
        s0.push(false);
        qpsk(p, &s0)
    }).collect()
}

fn bits31(n: u32) -> Vec<bool> {
    let mut v = Vec::new();
    v.resize(31, false);
    for i in 0..31 {
        v[i] = ((n >> i) & 1) == 1
    }
    v
}

fn qpsk(p: &Params, d: &Vec<bool>) -> Vec<Complex64> {
    let symb_br = 2.0 * p.bit_rate;
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
