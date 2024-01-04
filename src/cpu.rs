#[derive(Clone, Copy)]
pub struct Cpu {
    pub v: [u8; 16],
    pub sp: u8,
    pub dt: u8,
    pub st: u8,
    pub pc: u16,
    pub i: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            v: [0; 16],
            sp: 0,
            dt: 0,
            st: 0,
            pc: 0x0200,
            i: 0,
        }
    }

}