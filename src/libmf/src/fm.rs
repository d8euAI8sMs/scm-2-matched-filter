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
    sample_test_seq(p.n as usize, &mut r.i);
    sample_test_seq(p.n as usize, &mut r.q);
    for f in r.f.iter_mut() {
        sample_test_seq(p.n as usize, f);
    }
    r
}

fn sample_test_seq(n: usize, v: &mut Vec<Pt>) {
    *v = (0..n).map(|i| Pt {
        x: 0.11 * (i as f64),
        y: 0.12 * ((i * i) as f64)
    }).collect()
}
