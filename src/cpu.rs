#[derive(Clone, Copy)]
pub struct Cpu {
    pub v: [u8; 16],
    pub sp: u8,
    pub dt: u8,
    pub st: u8,
    pub pc: u16,
    pub i: u16,
    stack: [u16; 16]
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
            stack: [0; 16]
        }
    }

    pub fn stack_push(&mut self, stack_val: u16) -> Result<u16, &'static str> {
        if self.sp >= 16 {
            return Err("stack overflow");
        }
        self.stack[self.sp as usize] = stack_val;
        self.sp += 1;
        
        Ok(stack_val)
    }

    pub fn stack_pop(&mut self) -> Result<u16, &'static str> {
        if self.sp <= 0 {
            return  Err("stack overflow");
        }

        self.sp -= 1;

        Ok(self.stack[self.sp as usize])
    }

}