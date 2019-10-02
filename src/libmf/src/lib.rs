mod fm;

// dummy function

#[no_mangle]
pub extern fn hello() -> *const u8 {
    "Hello from Rust!\0".as_ptr()
}

// task-specific

#[repr(C)]
pub struct Params {
    n: u32,
    sample_rate: f64,
    bit_rate: f64,
    snr: f64,
}

#[repr(C)]
pub struct Signal {
    pts: *mut Pt,
    n: usize,
}

#[repr(C)]
pub struct Demo {
    i: Signal,
    q: Signal,
    f: [Signal; 4],
}

#[repr(C)]
pub struct Pt {
    x: f64,
    y: f64,
}

impl Pt {
    fn from(o: &fm::Pt) -> Pt {
        Pt { x: o.x, y: o.y }
    }
}

#[no_mangle]
pub extern "C" fn release_demo(r: &mut Demo) {
    free_buf(&r.i);
    free_buf(&r.q);
    for f in r.f.iter() {
        free_buf(f);
    }
}

#[no_mangle]
pub extern "C" fn demo(p: Params, r: &mut Demo) {
    let d = fm::demo(to_their_params(&p));
    to_our_demo(&d, r);
}

fn free_buf(buf: &Signal) {
    if buf.n != 0 {
        let s = unsafe { std::slice::from_raw_parts_mut(buf.pts, buf.n) };
        let s = s.as_mut_ptr();
        unsafe {
            Box::from_raw(s);
        }
    }
}

fn to_buf(vct: Vec<Pt>) -> Signal {
    let mut buf = vct.into_boxed_slice();
    let len = buf.len();
    let data = buf.as_mut_ptr();
    std::mem::forget(buf);
    Signal { n: len, pts: data }
}

fn to_our_pt(vct: &Vec<fm::Pt>) -> Vec<Pt> {
    vct.into_iter().map(Pt::from).collect()
}

fn to_our_demo(d: &fm::Demo, r: &mut Demo) {
    r.i = to_buf(to_our_pt(&d.i));
    r.q = to_buf(to_our_pt(&d.q));
    for (i, f) in d.f.iter().enumerate() {
        r.f[i] = to_buf(to_our_pt(f));
    }
}

fn to_their_params(p: &Params) -> fm::Params {
    fm::Params {
        n: p.n,
        sample_rate: p.sample_rate,
        bit_rate: p.bit_rate,
        snr: p.snr
    }
}
