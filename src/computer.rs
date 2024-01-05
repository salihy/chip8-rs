use std::usize;
use rand::Rng;
use crate::cpu::Cpu;
use crate::display::Display;

pub struct Computer {
    memory: [u8; 4096],
    cpu: Cpu,
    display: Display
}

impl Computer {
    pub fn reset(&mut self) {
        self.cpu = Cpu::new();
        self.display = Display::new();

        for n in 0..4096 {
            self.memory[n] = 0;
        }
    }

    pub fn dump(&self) -> Vec<u8> {
        self.memory.to_vec()
    }

    pub fn new(data: Vec<u8> ) -> Self {
        let mut mem = [0; 4096];

        let sprites = 
        [0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80]; //F

        for (pos, e) in sprites.iter().enumerate() {
            mem[pos] = *e; 
        }

        for (pos, e) in data.iter().enumerate() {
            mem[0x200 + pos] = *e;
        }


        Self {
            cpu: Cpu::new(),
            display: Display::new(),
            memory: mem
        }
    }

    pub fn tick(&mut self) {
        let m1 = self.memory[self.cpu.pc as usize];
        let m2: u8 = self.memory[(self.cpu.pc + 1) as usize];
        let val: u16 = ((m1 as u16) << 8) | (m2 as u16);

        self.cpu.pc += 2;

        //println!("pc: {} - {:#06x}", self.cpu.pc, val);

        if val == 0x00E0 { // cls
            println!("cls");
            self.display.clear();
        }
        else if val == 0x00EE { //ret
            match self.cpu.stack_pop() {
                Ok(stack_val) => {
                    self.cpu.pc = stack_val;
                },
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
        else if val >= 0x1000 && val < 0x2000 { // jump
            self.cpu.pc = val & 0x0FFF;
            //let ct: Vec<u8> = self.display().dump().into_iter().map(|m| m as u8).collect();
            //fs::write("fb.dat", ct).unwrap();
        }
        else if val >= 0x2000 && val < 0x3000 { // call
            let address = val & 0x0FFF;

            match self.cpu.stack_push(self.cpu.pc) {
                Ok(_) => {
                    self.cpu.pc = address;
                },
                Err(e) => {
                    println!("{}", e);
                }
            }
            
        }
        else if val >= 0x3000 && val < 0x4000 { //skip instruction
            let x = (val & 0x0F00) >> 8 as u8;
            let kk = (val & 0x00FF) as u8;

            if self.cpu.v[x as usize] == kk {
                self.cpu.pc += 2;
            }

        }
        else if val >= 0x4000 && val < 0x5000 { //skip instruction if not eq
            let x = (val & 0x0F00) >> 8 as u8;
            let kk = (val & 0x00FF) as u8;

            if self.cpu.v[x as usize] != kk {
                self.cpu.pc += 2;
            }

        }
        else if val >= 0x5000 && val < 0x6000 { //skip instruction if not eq
            let x = (val & 0x0F00) >> 8 as u8;
            let y = (val & 0x00F0) >> 4 as u8;

            if self.cpu.v[x as usize] == self.cpu.v[y as usize] {
                self.cpu.pc += 2;
            }

        }
        else if val >= 0x6000 && val < 0x7000 { // set register vx

            //println!("set vx");

            let x = val & 0x0FFF;
            let idx = x >> 8;

            self.cpu.v[idx as usize] = (val & 0x00FF) as u8;
        }
        else if val >= 0x7000 && val < 0x8000 { // add register vx

            //println!("add vx");

            let x = val & 0x0FFF;
            let idx = x >> 8;

            self.cpu.v[idx as usize] += (val & 0x00FF)  as u8;
        }
        else if val >= 0x8000 && val < 0x9000 && (val & 0x000F) == 0x0000  { //ld vx vy
            let x = ((val & 0x0F00) >> 8) as u8;
            let y = ((val & 0x00F0) >> 4) as u8;

            self.cpu.v[x as usize] = self.cpu.v[y as usize];
        }
        else if val >= 0x8000 && val < 0x9000 && (val & 0x000F) == 0x0001  { //or vx vy
            let x = ((val & 0x0F00) >> 8) as u8;
            let y = ((val & 0x00F0) >> 4) as u8;

            self.cpu.v[x as usize] = self.cpu.v[x as usize] | self.cpu.v[y as usize];
        }
        else if val >= 0x8000 && val < 0x9000 && (val & 0x000F) == 0x0002  { //and vx vy
            let x = ((val & 0x0F00) >> 8) as u8;
            let y = ((val & 0x00F0) >> 4) as u8;

            self.cpu.v[x as usize] = self.cpu.v[x as usize] & self.cpu.v[y as usize];
        }
        else if val >= 0x8000 && val < 0x9000 && (val & 0x000F) == 0x0003  { //xor vx vy
            let x = ((val & 0x0F00) >> 8) as u8;
            let y = ((val & 0x00F0) >> 4) as u8;

            self.cpu.v[x as usize] = self.cpu.v[x as usize] ^ self.cpu.v[y as usize];
        }
        else if val >= 0x8000 && val < 0x9000 && (val & 0x000F) == 0x0004  { //add vx vy
            let x = ((val & 0x0F00) >> 8) as u8;
            let y = ((val & 0x00F0) >> 4) as u8;

            if (self.cpu.v[x as usize] as u16 + self.cpu.v[y as usize] as u16) > 255 {
                self.cpu.v[0x000F] = 1;
            }
            else {
                self.cpu.v[0x000F] = 0;
            }

            self.cpu.v[x as usize]  = ((self.cpu.v[x as usize] as u16 + self.cpu.v[y as usize] as u16) & 0x00FF) as u8;
        }
        else if val >= 0x8000 && val < 0x9000 && (val & 0x000F) == 0x0005  { //sub vx vy 
            let x = ((val & 0x0F00) >> 8) as u8;
            let y = ((val & 0x00F0) >> 4) as u8;

            if self.cpu.v[x as usize] >= self.cpu.v[y as usize] {
                self.cpu.v[0x000F] = 1;
                self.cpu.v[x as usize] = self.cpu.v[x as usize] - self.cpu.v[y as usize];
            }
            else {
                self.cpu.v[0x000F] = 0;
                self.cpu.v[x as usize] = ((self.cpu.v[x as usize] as u16) + 0x0100 - (self.cpu.v[y as usize] as u16)) as u8;
            }

        }
        else if val >= 0x8000 && val < 0x9000 && (val & 0x000F) == 0x0006  { //shr vx vy 
            let x = ((val & 0x0F00) >> 8) as u8;
            //let y = ((val & 0x00F0) >> 4) as u8;

            if self.cpu.v[x as usize] & 0x0001 == 0x0001 {
                self.cpu.v[0x000F] = 1;
            }
            else {
                self.cpu.v[0x000F] = 0;
            }

            self.cpu.v[x as usize] = self.cpu.v[x as usize] >> 1;
        }
        else if val >= 0x8000 && val < 0x9000 && (val & 0x000F) == 0x0007  { //sub vy vx 
            let x = ((val & 0x0F00) >> 8) as u8;
            let y = ((val & 0x00F0) >> 4) as u8;

            if self.cpu.v[y as usize] >= self.cpu.v[x as usize] {
                self.cpu.v[0x000F] = 1;
                self.cpu.v[x as usize] = self.cpu.v[y as usize] - self.cpu.v[x as usize];
            }
            else {
                self.cpu.v[0x000F] = 0;
                self.cpu.v[x as usize] = ((self.cpu.v[y as usize] as u16) + 0x0100 - (self.cpu.v[x as usize] as u16)) as u8;
            }

        }
        else if val >= 0x8000 && val < 0x9000 && (val & 0x000F) == 0x000E  { //shl vx vy 
            let x = ((val & 0x0F00) >> 8) as u8;
            //let y = ((val & 0x00F0) >> 4) as u8;

            if (self.cpu.v[x as usize] >> 7) & 0x0001 == 0x0001 {
                self.cpu.v[0x000F] = 1;
            }
            else {
                self.cpu.v[0x000F] = 0;
            }

            self.cpu.v[x as usize] = self.cpu.v[x as usize] << 1;
        }
        else if val >= 0x9000 && val < 0xA000 && (val & 0x000F) == 0x0000 { // sne vx vy
            let x = ((val & 0x0F00) >> 8) as u8;
            let y = ((val & 0x00F0) >> 4) as u8;

            if self.cpu.v[x as usize] != self.cpu.v[y as usize] {
                self.cpu.pc += 2;
            }

        }
        else if val >= 0xA000 && val < 0xB000 { // ld i
            self.cpu.i = val & 0x0FFF;
        }
        else if val >= 0xB000 && val < 0xC000 { // jmp v0 addr
            let mut addr = val & 0x0FFF;
            addr += self.cpu.v[0 as usize] as u16;
            self.cpu.pc = addr;
        }
        else if val >= 0xC000 && val < 0xD000 { // rnd vx, byte
            let x = (val & 0x0F00) >> 8 as u8;
            let kk = (val & 0x00FF) as u8;

            let mut rng = rand::thread_rng();

            let r: u8 = rng.gen();

            self.cpu.v[x as usize] = kk & r;
        }
        else if val >= 0xD000 && val < 0xE000 { // display
            let x: u8 = ((val & 0x0F00) >> 8) as u8;
            let y: u8 = ((val & 0x00F0) >> 4) as u8;
            let n: u8 = (val & 0x000F) as u8;

            let start_idx = self.cpu.i as usize;
            let end_idx = (self.cpu.i + n as u16) as usize;

            let mut sprite:[u8; 15] = [0; 15];

            println!("start_idx: {}, end_idx: {}", start_idx, end_idx);

            // sprite.copy_from_slice(&self.memory[start_idx..end_idx]);
            for d in start_idx..end_idx {
                sprite[d - start_idx] = self.memory[start_idx];
            }


            //println!("{:?}", sprite);

            let vx: u8 = self.cpu.v[x as usize];
            let vy: u8 = self.cpu.v[y as usize];

            for i in 0..n {
                //println!("sprite line: {}", i);
                for b in 0..8 {

                    //println!("sprite bit: {}", b);

                    let mut bit = false;
                    if (sprite[i as usize] >> (8 - b - 1) & 0b00000001) == 0b00000001 {
                        bit = true;
                        
                    }

                    //println!("sprite val: {}", bit);

                    let px = self.display.get((vx + b) as usize, (vy + i) as usize) ^ bit;

                    if px {
                        self.cpu.v[0x0f] = 0;
                    }
                    else {
                        self.cpu.v[0x0f] = 1;
                    }

                    //println!("sprite set");
                    self.display.set((vx + b) as usize, (vy + i) as usize, px);
                }
            }
        }
        //not complete
        else if val & 0xF0FF == 0xE09E { //skp Vx
            self.cpu.pc -= 2; //key not pressed
        }
        //not complete
        else if val & 0xF0FF == 0xE0A1 { //sknp Vx
            self.cpu.pc -= 2; //key not pressed
        }
        else if val & 0xF0FF == 0xF007 { //ld vx dt
            let x: u8 = ((val & 0x0F00) >> 8) as u8;
            self.cpu.v[x as usize] = self.cpu.dt;
        }
        //not complete
        else if val & 0xF0FF == 0xF00A { //ld vx k
            self.cpu.pc -= 2; //key not pressed
        }
        else if val & 0xF0FF == 0xF015 { //ld dt vx
            let x: u8 = ((val & 0x0F00) >> 8) as u8;
            self.cpu.dt = self.cpu.v[x as usize];
        }
        else if val & 0xF0FF == 0xF018 { //ld st vx
            let x: u8 = ((val & 0x0F00) >> 8) as u8;
            self.cpu.st = self.cpu.v[x as usize];
        }
        else if val & 0xF0FF == 0xF01E { //add i vx
            let x: u8 = ((val & 0x0F00) >> 8) as u8;
            self.cpu.i += self.cpu.v[x as usize] as u16;
        }
        else if val & 0xF0FF == 0xF029 { //ld f vx
            let x: u8 = ((val & 0x0F00) >> 8) as u8;
            self.cpu.i += (5 as u16) * (self.cpu.v[x as usize] as u16);
        }
        else if val & 0xF0FF == 0xF033 { //ld b vx
            let x: u8 = ((val & 0x0F00) >> 8) as u8;
            self.memory[self.cpu.i as usize] = (self.cpu.v[x as usize] - (self.cpu.v[x as usize] % 100)) / 100;
            self.memory[(self.cpu.i + 1) as usize] = ((self.cpu.v[x as usize] % 100) - (self.cpu.v[x as usize] % 10)) / 10;
            self.memory[(self.cpu.i + 2) as usize] = self.cpu.v[x as usize] % 10;
        }
        else if val & 0xF0FF == 0xF055 { //ld i vx
            let x: u8 = ((val & 0x0F00) >> 8) as u8;
            
            for d in 0..x {
                self.memory[(self.cpu.i + d as u16) as usize] = self.cpu.v[d as usize];
            }

        }
        else if val & 0xF0FF == 0xF065 { //ld vx i
            let x: u8 = ((val & 0x0F00) >> 8) as u8;
            
            for d in 0..x {
                self.cpu.v[d as usize] = self.memory[(self.cpu.i + d as u16) as usize];
            }

        }
    }

    pub fn display(&self) -> Display {
        self.display.clone()
    }
}