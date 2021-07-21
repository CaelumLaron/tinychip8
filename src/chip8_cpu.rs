use crate::constants;
use crate::graphics::Graphics;
use std::time::Instant;

pub(crate) struct Chip8Cpu {
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
    draw_flag: bool,
    interface: Graphics,
}

impl Chip8Cpu {
    pub(crate) fn start() -> Self {
        let pc = 512;
        let mut memory = [0; 4096];

        memory[..80].clone_from_slice(&constants::FONT_SET[..80]);

        Self {
            memory,
            pc,
            v: [0; 16],
            i: 0,
            gfx: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16],
            draw_flag: false,
            interface: Graphics::with_size(64, 32),
        }
    }

    pub(crate) fn load_game(&mut self, file_path: &str) -> std::io::Result<()> {
        let buffer: Vec<u8> = std::fs::read(file_path)?;

        self.memory[512..(buffer.len() + 512)].clone_from_slice(&buffer[..]);

        Ok(())
    }

    pub fn emulate(&mut self) {
        let mut last_time_cycle = Instant::now();
        let mut last_time_tick = Instant::now();
        loop {
            if last_time_cycle.elapsed().as_millis() > 16 {
                self.cycle();
                last_time_cycle = Instant::now();
            }

            if last_time_tick.elapsed().as_millis() > 8 {
                self.interface.tick(&self.gfx, &mut self.key);
                last_time_tick = Instant::now();
            }
        }
    }

    fn cycle(&mut self) {
        // Fetch op_code
        let op_code: u16 = ((self.memory[self.pc as usize] as u16) << 8)
            | self.memory[self.pc as usize + 1] as u16;

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
                0x000E => self.cls(),
                0x00EE => self.ret(),
                _ => self.sys(),
            },
            0x1000 => self.jp_nnn(nnn),
            0x2000 => self.call_nnn(nnn),
            0x3000 => self.se_vx_kk(x, kk),
            0x4000 => self.sne_vx_kk(x, kk),
            0x5000 => self.se_vx_vy(x, y),
            0x6000 => self.ld_vx_kk(x, kk),
            0x7000 => self.add_vx_kk(x, kk),
            0x8000 => match op_code & 0x000F {
                0x0000 => self.ld_vx_vy(x, y),
                0x0001 => self.or_vx_vy(x, y),
                0x0002 => self.and_vx_vy(x, y),
                0x0003 => self.xor_vx_vy(x, y),
                0x0004 => self.add_vx_vy(x, y),
                0x0005 => self.sub_vx_vy(x, y),
                0x0006 => self.shr(x),
                0x0007 => self.subn_vx_vy(x, y),
                0x000E => self.shl(x),
                _ => println!("Unknown opcode [0x800N]: {:X}", op_code),
            },
            0x9000 => self.sne_vx_vy(x, y),
            0xA000 => self.ld_i_nnn(nnn),
            0xB000 => self.jp_v0_nnn(nnn),
            0xC000 => self.rnd_vx_kk(x, kk),
            0xD000 => self.drw_vx_vy_n(x, y, n),
            0xE000 => match op_code & 0x00FF {
                0x009E => self.skp_vx(x),
                0x00A1 => self.sknp_vx(x),
                _ => println!("Unknown opcode [0xE000]: 0x{:X}", op_code),
            },
            0xF000 => match op_code & 0x00FF {
                0x0007 => self.ld_vx_dt(x),
                0x000A => self.ld_vx_k(x),
                0x0015 => self.ld_dt_vx(x),
                0x0018 => self.ld_st_vx(x),
                0x001E => self.add_i_vx(x),
                0x0029 => self.ld_f_vx(x),
                0x0033 => self.ld_b_vx(x),
                0x0055 => self.ld_i_vx(x),
                0x0065 => self.ld_vx_i(x),
                _ => println!("Unknown opcode [0xF000]: 0x{:X}", op_code),
            },
            _ => println!("Unknown opcode: 0x{:X}", op_code),
        }
    }

    fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                print!("\x07");
            }
            self.sound_timer -= 1;
        }
    }

    fn cls(&mut self) {
        self.gfx.fill_with(Default::default);
        self.draw_flag = true;
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn ret(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn sys(&mut self) {
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn jp_nnn(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    fn call_nnn(&mut self, nnn: u16) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = nnn;
    }

    fn se_vx_kk(&mut self, x: u16, kk: u16) {
        if self.v[x as usize] == kk as u8 {
            self.pc = (self.pc + 2) & 0x0FFF;
        }
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn sne_vx_kk(&mut self, x: u16, kk: u16) {
        if self.v[x as usize] != kk as u8 {
            self.pc = (self.pc + 2) & 0x0FFF;
        }
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn se_vx_vy(&mut self, x: u16, y: u16) {
        if self.v[x as usize] == self.v[y as usize] {
            self.pc = (self.pc + 2) & 0x0FFF;
        }
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn ld_vx_kk(&mut self, x: u16, kk: u16) {
        self.v[x as usize] = kk as u8;
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn add_vx_kk(&mut self, x: u16, kk: u16) {
        self.v[x as usize] = self.v[x as usize].wrapping_add(kk as u8);
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn ld_vx_vy(&mut self, x: u16, y: u16) {
        self.v[x as usize] = self.v[y as usize];
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn or_vx_vy(&mut self, x: u16, y: u16) {
        self.v[x as usize] = self.v[x as usize] | self.v[y as usize];
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn and_vx_vy(&mut self, x: u16, y: u16) {
        self.v[x as usize] = self.v[x as usize] & self.v[y as usize];
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn xor_vx_vy(&mut self, x: u16, y: u16) {
        self.v[x as usize] = self.v[x as usize] ^ self.v[y as usize];
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn add_vx_vy(&mut self, x: u16, y: u16) {
        self.v[0xF] = if self.v[y as usize] > 255 - self.v[x as usize] {
            1
        } else {
            0
        };
        self.v[x as usize] = self.v[x as usize].wrapping_add(self.v[y as usize]);
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn sub_vx_vy(&mut self, x: u16, y: u16) {
        self.v[0xF] = if self.v[x as usize] >= self.v[y as usize] {
            1
        } else {
            0
        };
        self.v[x as usize] = self.v[x as usize].wrapping_sub(self.v[y as usize]);
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn shr(&mut self, x: u16) {
        self.v[0xF] = self.v[x as usize] & 1;
        self.v[x as usize] >>= 1;
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn subn_vx_vy(&mut self, x: u16, y: u16) {
        self.v[0xF] = if self.v[y as usize] >= self.v[x as usize] {
            1
        } else {
            0
        };
        self.v[x as usize] = self.v[y as usize].wrapping_sub(self.v[x as usize]);
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn shl(&mut self, x: u16) {
        self.v[0xF] = self.v[x as usize] & 0x80;
        self.v[x as usize] <<= 1;
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn sne_vx_vy(&mut self, x: u16, y: u16) {
        if self.v[x as usize] != self.v[y as usize] {
            self.pc = (self.pc + 2) & 0x0FFF;
        }
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn ld_i_nnn(&mut self, nnn: u16) {
        self.i = nnn;
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn jp_v0_nnn(&mut self, nnn: u16) {
        self.pc = (nnn + self.v[0x0] as u16) & 0x0FFF;
    }

    fn rnd_vx_kk(&mut self, x: u16, kk: u16) {
        self.v[x as usize] = (kk as u8) & rand::random::<u8>();
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn drw_vx_vy_n(&mut self, x: u16, y: u16, n: u16) {
        self.v[0xF] = 0;

        for yline in 0..n {
            let pixel = self.memory[(self.i + yline as u16) as usize];
            for xline in 0..8_u16 {
                if (pixel & (0x80 >> xline)) != 0 {
                    let pos_x = (self.v[x as usize] as u16 + xline) % 64;
                    let pos_y = (self.v[y as usize] as u16 + yline) % 32;
                    let pos_pixel = (pos_x + (pos_y * 64)) as usize;
                    if self.gfx[pos_pixel] == 1 {
                        self.v[0xF] = 1;
                    }
                    self.gfx[pos_pixel] ^= 1;
                }
            }
        }
        self.draw_flag = true;
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn skp_vx(&mut self, x: u16) {
        if self.key[self.v[x as usize] as usize] != 0 {
            self.pc = (self.pc + 2) & 0x0FFF;
        }
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn sknp_vx(&mut self, x: u16) {
        if self.key[self.v[x as usize] as usize] == 0 {
            self.pc = (self.pc + 2) & 0x0FFF;
        }
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn ld_vx_dt(&mut self, x: u16) {
        self.v[x as usize] = self.delay_timer;
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn ld_vx_k(&mut self, x: u16) {
        let key = self.interface.wait_key();
        self.v[x as usize] = key as u8;
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn ld_dt_vx(&mut self, x: u16) {
        self.delay_timer = self.v[x as usize];
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn ld_st_vx(&mut self, x: u16) {
        self.sound_timer = self.v[x as usize];
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn add_i_vx(&mut self, x: u16) {
        self.i = (self.i + self.v[x as usize] as u16) & 0xFFF;
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn ld_f_vx(&mut self, x: u16) {
        self.i = 5 * self.v[x as usize] as u16;
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn ld_b_vx(&mut self, x: u16) {
        self.memory[self.i as usize] = self.v[x as usize] / 100;
        self.memory[self.i as usize + 1] = (self.v[x as usize] % 100) / 10;
        self.memory[self.i as usize + 2] = self.v[x as usize] % 10;
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn ld_i_vx(&mut self, x: u16) {
        for i in 0..=x as usize {
            self.memory[self.i as usize + i] = self.v[i];
        }
        self.i += x + 1;
        self.pc = (self.pc + 2) & 0x0FFF;
    }

    fn ld_vx_i(&mut self, x: u16) {
        for i in 0..=x as usize {
            self.v[i] = self.memory[self.i as usize + i];
        }

        self.i += x + 1;
        self.pc = (self.pc + 2) & 0x0FFF;
    }
}
