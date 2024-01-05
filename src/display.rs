
#[derive(Copy, Clone)]
pub struct Display {
    framebuffer: [bool; 64*32],
}

impl Display {
    pub fn new() -> Self {
        Self { framebuffer: [true; 64*32] }
    }

    pub fn set(&mut self, x: usize, y: usize, data: bool) {
        let idx = y * 64 + x;
        if idx < 2048 {
            self.framebuffer[idx] = data;
        }
        //println!("idx: {}, fb: {}, data: {}", idx, self.framebuffer[idx], data);
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        let idx = y * 64 + x;
        if idx < 2048 {
            self.framebuffer[idx]
        }
        else{
            false
        }
    }

    pub fn clear(&mut self) {
        for i in 0..64*32 {
            self.framebuffer[i] = false;
        }
    }

    pub fn dump(&self) -> Vec<bool> {
        self.framebuffer.clone().to_vec()
    }
}