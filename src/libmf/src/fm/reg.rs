use std::fmt;

#[derive(Clone)]
pub struct Reg31 {
    val: u8,
    msk: u8,
}

#[allow(dead_code)]
impl Reg31 {
    pub fn new(m: u8) -> Reg31 {
        let r = Reg31 { val: 0b11111_u8, msk: m };
        r.next()
    }

    pub fn rot(v: u32, n: u32) -> u32 {
        let t = v.rotate_right(n);
        let m = 1 << 31;
        (t & !m) | (t >> 1) & m
    }

    pub fn peek(&self) -> bool {
        self.peek_val(self.val)
    }

    pub fn val(&self) -> u8 {
        self.clean_val(self.val)
    }

    pub fn collect(&self) -> (u8, u32) {
        let stop = self.clean_val(self.val);
        let mut cur = stop;
        let mut i: u8 = 0;
        let mut r: u32 = 0;
        while {
            cur = self.next_val(cur);
            r |= (self.peek_val(cur) as u32) << i;
            i += 1;
            self.clean_val(cur) != stop && cur != 0
        } {}
        (i, r)
    }

    pub fn next(&self) -> Reg31 {
        Reg31 { val: self.next_val(self.val), ..*self }
    }
    
    fn next_val(&self, val: u8) -> u8 {
        let peek = ((val & self.msk).count_ones() as u8) & 1;
        (val << 1) | peek
    }
    
    fn peek_val(&self, val: u8) -> bool {
        (val & (1 << 5)) != 0
    }
    
    fn clean_val(&self, val: u8) -> u8 {
        val & 0b11111
    }
}

impl fmt::Debug for Reg31 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Reg31 {{ val: {:b}, mask: {:b} }}", self.val, self.msk)
    }
}
