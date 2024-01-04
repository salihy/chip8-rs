use std::{usize, fs};

use crate::cpu::Cpu;
use crate::display::{Display};

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

        let sprites = [0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
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

        if val >= 0x1000 && val < 0x2000 { // jump
            println!("jmp");

            println!("pc: {} - {:#06x}", self.cpu.pc, val);

            self.cpu.pc = val & 0x0FFF;

            let ct: Vec<u8> = self.display().dump().into_iter().map(|m| m as u8).collect();

            fs::write("fb.dat", ct).unwrap();
        }

        if val >= 0x6000 && val < 0x7000 { // set register vx

            println!("set vx");

            let x = val & 0x0FFF;
            let idx = x >> 8;

            self.cpu.v[idx as usize] = (val & 0x00FF) as u8;
        }

        if val >= 0x7000 && val < 0x8000 { // add register vx

            println!("add vx");

            let x = val & 0x0FFF;
            let idx = x >> 8;

            self.cpu.v[idx as usize] += (val & 0x00FF)  as u8;
        }

        if val >= 0xA000 && val < 0xB000 { // set I
            println!("set i");

            self.cpu.i = val & 0x0FFF;
        }
        
        if val >= 0xD000 && val < 0xE000 { // display

            println!("disp");

            //self.cpu.i = val & 0x0FFF;
            let x: u8 = ((val & 0x0F00) >> 8) as u8;
            let y: u8 = ((val & 0x00F0) >> 4) as u8;
            let n: u8 = (val & 0x000F) as u8;

            println!("x: {}, y: {}, n: {}", x, y, n);

            let start_idx = self.cpu.i as usize;
            let end_idx = (self.cpu.i + n as u16) as usize;

            println!("start: {:#06x}, end: {:#06x}", start_idx, end_idx);



            let mut sprite:[u8; 15] = [0; 15];
            sprite.copy_from_slice(&self.memory[start_idx..end_idx]);

            println!("{:?}", sprite);

            let vx: u8 = self.cpu.v[x as usize];
            let vy: u8 = self.cpu.v[y as usize];

            println!("vx: {}, vy: {}", vx, vy);


            for i in 0..n {
                println!("sprite line: {}", i);
                for b in 0..8 {

                    println!("sprite bit: {}", b);

                    let mut bit = false;
                    if (sprite[i as usize] >> (8 - b - 1) & 0b00000001) == 0b00000001 {
                        bit = true;
                        
                    }

                    println!("sprite val: {}", bit);

                    let px = self.display.get((vx + b) as usize, (vy + i) as usize) ^ bit;

                    if px {
                        self.cpu.v[0x0f] = 0;
                    }
                    else {
                        self.cpu.v[0x0f] = 1;
                    }

                    println!("sprite set");
                    self.display.set((vx + b) as usize, (vy + i) as usize, px);
                }
            }
        }


    }

    pub fn display(&self) -> Display {
        self.display.clone()
    }
}