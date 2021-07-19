use crate::constants;
use crate::graphics::Graphics;

struct CPU {
    opcode: u16,
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    pc: u16,
    gfx: [u8; 64 * 32],
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    sp: u16,
    key: [u8; 16],
    graphics: Graphics,
}

impl CPU {
    fn start() -> Self {
        let pc = 0x200;
        let mut memory = [0; 4096];

        for i in 0..80 {
            memory[i] = super::constants::FONT_SET[i];
        }

        Self {
            memory,
            pc,
            opcode: 0,
            v: [0; 16],
            i: 0,
            gfx: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16],
            graphics: Graphics::with_size(640, 320),
        }
    }

    fn initialize(&mut self) {
        self.pc = 0x200;
        self.opcode = 0;
        self.i = 0;
        self.sp = 0;

        // Clear display
        self.gfx.fill_with(Default::default);
        // Clear Stack
        self.stack.fill_with(Default::default);
        // Clear Registers
        self.v.fill_with(Default::default);
        // Clear memory
        self.memory.fill_with(Default::default);

        // Load fontset
        for i in 0..80 {
            self.memory[i] = super::constants::FONT_SET[i];
        }

        // Reset Timers
        self.delay_timer = 0;
        self.sound_timer = 0;
    }

    pub fn load_game(&mut self, file_path: &str) -> std::io::Result<()> {
        let buffer: Vec<u8> = std::fs::read(file_path)?;

        for i in 0..buffer.len() {
            self.memory[i + 512] = buffer[i];
        }

        Ok(())
    }

    fn next(&mut self) {
        // Fetch op_code
        let op_code: u16 =
            (self.memory[self.pc as usize] as u16) << 8 | self.memory[self.pc as usize + 1] as u16;

        // Decode op_code & execute
        self.decode(op_code);

        self.update_timers();
    }

    fn decode(&mut self, op_code: u16) {
        // Decode op_code
        let x = (op_code & 0x0F00) >> 8;
        let y = (op_code & 0x00F0) >> 4;
        let kk = op_code & 0x00FF;
        let nnn = op_code & 0x0FFF;
        let n = op_code & 0x000F;

        match op_code & 0xF000 {
            0x0000 => match op_code & 0x00FF {
                0x000E => {
                    self.cls();
                }
                0x00EE => {
                    self.rts();
                }
                _ => {
                    self.rca();
                }
            },
            0x1000 => {
                self.jump(nnn);
            }
            0x2000 => {
                self.call(nnn);
            }
            0x3000 => {
                self.skip_eq(x, kk);
            }
            0x4000 => {
                self.skip_ne(x, kk);
            }
            0x5000 => {
                self.skip_eqr(x, y);
            }
            0x6000 => {
                self.load(x, kk);
            }
            0x7000 => {
                self.add_c(x, kk);
            }
            0x8000 => match op_code & 0x000F {
                0x0000 => {
                    self.mov(x, y);
                }
                0x0001 => {
                    self.or(x, y);
                }
                0x0002 => {
                    self.and(x, y);
                }
                0x0003 => {
                    self.xor(x, y);
                }
                0x0004 => {
                    self.add(x, y);
                }
                0x0005 => {
                    self.sub(x, y);
                }
                0x0006 => {
                    self.shr(x);
                }
                0x0007 => {
                    self.subn(x, y);
                }
                0x000E => {
                    self.shl(x);
                }
                _ => (println!("Unknown opcode [0x800N]: {:X}", op_code)),
            },
            0x9000 => {
                self.skip_ner(x, y);
            }
            0xA000 => {
                self.load_addr(nnn);
            }
            0xB000 => {
                self.jump_r(nnn);
            }
            0xC000 => {
                self.rnd(x, kk);
            }
            0xD000 => {
                self.drw(x, y, n);
            }
            0xE000 => match op_code & 0x00FF {
                0x009E => {
                    self.skip_key_pressed(x);
                }
                0x00A1 => {
                    self.skip_key_npressed(x);
                }
                _ => (println!("Unknown opcode [0xE000]: 0x{:X}", op_code)),
            },
            0xF000 => match op_code & 0x0FFF {
                0x0007 => {
                    self.load_rdt(x);
                }
                0x000A => {
                    self.load_key(x);
                }
                0x0015 => {
                    self.load_dtr(x);
                }
                0x0018 => {
                    self.load_str(x);
                }
                0x001E => {
                    self.addi(x);
                }
                0x0029 => {
                    self.load_fi(x);
                }
                0x0033 => {
                    self.load_mem(x);
                }
                0x0055 => {
                    self.load_memi(x);
                }
                0x0065 => {
                    self.load_ri(x);
                }
                _ => (println!("Unknown opcode [0xF000]: 0x{:X}", op_code)),
            },
            _ => (println!("Unknown opcode: 0x{:X}", op_code)),
        }
    }

    fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEP");
            }
            self.sound_timer -= 1;
        }
    }

    fn cls(&mut self) {
        self.gfx.fill_with(Default::default);
        self.graphics.clear_screen();
        self.pc += 2;
    }

    fn rts(&mut self) {
        self.pc = self.stack[self.sp as usize - 1];
        self.sp -= 1;
    }

    fn rca(&mut self) {
        self.pc += 2;
    }

    fn jump(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    fn call(&mut self, nnn: u16) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = nnn;
    }

    fn skip_eq(&mut self, x: u16, kk: u16) {
        if self.v[x as usize] == kk as u8 {
            self.pc += 2;
        }
        self.pc += 2;
    }

    fn skip_ne(&mut self, x: u16, kk: u16) {
        if self.v[x as usize] != kk as u8 {
            self.pc += 2;
        }
        self.pc += 2;
    }

    fn skip_eqr(&mut self, x: u16, y: u16) {
        if self.v[x as usize] == self.v[y as usize] {
            self.pc += 2;
        }
        self.pc += 2;
    }

    fn load(&mut self, x: u16, kk: u16) {
        self.v[x as usize] = kk as u8;
        self.pc += 2;
    }

    fn add_c(&mut self, x: u16, kk: u16) {
        self.v[x as usize] += kk as u8;
        self.pc += 2;
    }

    fn mov(&mut self, x: u16, y: u16) {
        self.v[x as usize] = self.v[y as usize];
        self.pc += 2;
    }

    fn or(&mut self, x: u16, y: u16) {
        self.v[x as usize] |= self.v[y as usize];
        self.pc += 2;
    }

    fn and(&mut self, x: u16, y: u16) {
        self.v[x as usize] &= self.v[y as usize];
        self.pc += 2;
    }

    fn xor(&mut self, x: u16, y: u16) {
        self.v[x as usize] ^= self.v[y as usize];
        self.pc += 2;
    }

    fn add(&mut self, x: u16, y: u16) {
        self.v[0xF] = if self.v[y as usize] > 255 - self.v[x as usize] {
            1
        } else {
            0
        };
        self.v[x as usize] = self.v[x as usize].wrapping_add(self.v[y as usize]);
        self.pc += 2;
    }

    fn sub(&mut self, x: u16, y: u16) {
        self.v[0xF] = if self.v[y as usize] > self.v[x as usize] {
            0
        } else {
            1
        };
        self.v[x as usize] = self.v[x as usize].wrapping_sub(self.v[y as usize]);
        self.pc += 2;
    }

    fn shr(&mut self, x: u16) {
        self.v[0xF] = self.v[x as usize] & 1;
        self.v[x as usize] >>= 1;
        self.pc += 2;
    }

    fn subn(&mut self, x: u16, y: u16) {
        self.v[0xF] = if self.v[y as usize] < self.v[x as usize] {
            0
        } else {
            1
        };
        self.v[x as usize] = self.v[y as usize].wrapping_sub(self.v[x as usize]);
        self.pc += 2;
    }

    fn shl(&mut self, x: u16) {
        self.v[0xF] = self.v[x as usize] & 0x80;
        self.v[x as usize] <<= 1;
        self.pc += 2;
    }

    fn skip_ner(&mut self, x: u16, y: u16) {
        if self.v[x as usize] != self.v[y as usize] {
            self.pc += 2;
        }
        self.pc += 2;
    }

    fn load_addr(&mut self, nnn: u16) {
        self.i = nnn;
        self.pc += 2;
    }

    fn jump_r(&mut self, nnn: u16) {
        self.pc = nnn + self.v[0x0] as u16;
    }

    fn rnd(&mut self, x: u16, kk: u16) {
        self.v[x as usize] = (kk as u8) & rand::random::<u8>();
        self.pc += 2;
    }

    fn drw(&mut self, x: u16, y: u16, n: u16) {
        self.v[0xF] = 0;

        for yline in 0..n {
            let pixel = self.memory[(self.i + yline as u16) as usize];
            for xline in 0..8_u16 {
                if (pixel & (0x80 >> xline)) != 0 {
                    if self.gfx[((x + xline + ((y + yline) * 64)) % (32 * 64)) as usize] == 1 {
                        self.v[0xF] = 1;
                    }
                    self.gfx[((x + xline + ((y + yline) * 64)) % (32 * 64)) as usize] ^= 1;
                }
            }
        }

        self.pc += 2;
        self.graphics.update_screen(&self.gfx);
    }

    fn skip_key_pressed(&mut self, x: u16) {
        if self.key[self.v[x as usize] as usize] != 0 {
            self.pc += 2;
        }
        self.pc += 2;
    }

    fn skip_key_npressed(&mut self, x: u16) {
        if self.key[self.v[x as usize] as usize] == 0 {
            self.pc += 2;
        }
        self.pc += 2;
    }

    fn load_rdt(&mut self, x: u16) {
        self.v[x as usize] = self.delay_timer;
        self.pc += 2;
    }

    fn load_key(&mut self, x: u16) {
        // Wait a new key
        self.v[x as usize] = 1;
        self.pc += 2;
    }

    fn load_dtr(&mut self, x: u16) {
        self.delay_timer = self.v[x as usize];
        self.pc += 2;
    }

    fn load_str(&mut self, x: u16) {
        self.sound_timer = self.v[x as usize];
        self.pc += 2;
    }

    fn addi(&mut self, x: u16) {
        self.i += self.v[x as usize] as u16;
        self.pc += 2;
    }

    fn load_fi(&mut self, x: u16) {
        self.i = 5 * self.v[x as usize] as u16;
        self.pc += 2;
    }

    fn load_mem(&mut self, x: u16) {
        self.memory[self.i as usize] = self.v[x as usize] / 100;
        self.memory[self.i as usize + 1] = (self.v[x as usize] / 10) % 10;
        self.memory[self.i as usize + 2] = self.v[x as usize] % 10;
        self.pc += 2;
    }

    fn load_memi(&mut self, x: u16) {
        let mut i = self.i as usize;
        for x in 0..=x as usize {
            self.memory[i] = self.v[x];
            i += 1;
        }
        self.pc += 2;
    }

    fn load_ri(&mut self, x: u16) {
        let mut i = self.i as usize;
        for x in 0..=x as usize {
            self.v[x] = self.memory[i];
            i += 1;
        }
        self.pc += 2;
    }
}
