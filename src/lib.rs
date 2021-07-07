extern crate piston_window;
use piston_window::*;

mod constants {
    pub static FONT_SET: &'static [u8] = &[
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
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
        0xF0, 0x80, 0xF0, 0x80, 0x80  // F
    ];
}

struct TinyChip8 {
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
    key: [u8; 16]
}

impl TinyChip8 {
    fn new() -> Self { 
        let pc = 0x200;
        let mut memory = [0; 4096];

        for i in 0..80 {
            memory[i] = constants::FONT_SET[i];
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
            key: [0; 16]
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
            self.memory[i] = constants::FONT_SET[i];
        }

        // Reset Timers
        self.delay_timer = 0;
        self.sound_timer = 0;
    }

    fn load_game(&mut self, file_path: &str) -> std::io::Result<()> {
        let buffer: Vec<u8> = std::fs::read(file_path)?;

        for i in 0..buffer.len() {
            self.memory[i + 512] = buffer[i];
        }

        Ok(())
    }

    fn emulate_cycle(&mut self) {
        // Fetch opcode
        let opcode: u16 = (self.memory[self.pc as usize] as u16) << 8 | self.memory[self.pc as usize + 1] as u16; 
        // Decode opcode
        match opcode & 0xF000 {
            0x0000 => {
                match opcode & 0x000F {
                    0x0000 => {
                        // Clear the screen
                    },
                    0x000E => {
                        // Returns from subroutine
                    },
                    _ => ( println!("Unknown opcode [0x0000]: 0x{:X}", opcode) )
                }
            },
            0x1000 => {
                self.pc = 0x0FFF & opcode;
            },
            0x2000 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = 0x0FFF & opcode;
            },
            0x3000 => {
                if self.v[((opcode & 0x0F00) >> 8) as usize] == (opcode & 0x00FF) as u8 {
                    self.pc += 2;
                }
                self.pc += 2;
            },
            0x4000 => {
                if self.v[((opcode & 0x0F00) >> 8) as usize] != (opcode & 0x00FF) as u8 {
                    self.pc += 2;
                }
                self.pc += 2;
            },
            0x5000 => {
                if self.v[((opcode & 0x0F00) >> 8) as usize] == self.v[((opcode & 0x00F0) >> 4) as usize] {
                    self.pc += 2;
                }
                self.pc += 2;
            },
            0x6000 => {
                self.v[((opcode & 0x0F00) >> 8) as usize] = (opcode & 0x00FF) as u8;
                self.pc += 2;
            },
            0x7000 => {
                self.v[((opcode & 0x0F00) >> 8) as usize] += (opcode & 0x00FF) as u8;
                self.pc += 2;
            },
            0x8000 => {
                match opcode & 0x000F {
                    0x0000 => {
                        self.v[((opcode & 0x0F00) >> 8) as usize] =
                            self.v[((opcode & 0x00F0) >> 4) as usize];
                    },
                    0x0001 => {
                        self.v[((opcode & 0x0F00) >> 8) as usize] |=
                            self.v[((opcode & 0x00F0) >> 4) as usize];
                    },
                    0x0002 => {
                        self.v[((opcode & 0x0F00) >> 8) as usize] &=
                            self.v[((opcode & 0x00F0) >> 4) as usize];
                    },
                    0x0003 => {
                        self.v[((opcode & 0x0F00) >> 8) as usize] ^=
                            self.v[((opcode & 0x00F0) >> 4) as usize];
                    },
                    0x0004 => {
                        self.v[0xF] = 
                            if self.v[((opcode & 0x00F0) >> 4) as usize] > 255 - self.v[((opcode & 0x0F00) >> 8) as usize] { 1 } 
                            else { 0 };
                        self.v[((opcode & 0x0F00) >> 8) as usize] =
                            self.v[((opcode & 0x0F00) >> 8) as usize].wrapping_add(self.v[((opcode & 0x00F0) >> 4) as usize]);
                    },
                    0x0005 => {
                        self.v[0xF] = 
                            if self.v[((opcode & 0x00F0) >> 4) as usize] > self.v[((opcode & 0x0F00) >> 8) as usize] { 0 } 
                            else { 1 };
                        self.v[((opcode & 0x0F00) >> 8) as usize] =
                            self.v[((opcode & 0x0F00) >> 8) as usize].wrapping_sub(self.v[((opcode & 0x00F0) >> 4) as usize]);
                    },
                    0x0006 => {
                        self.v[0xF] =
                            self.v[((opcode & 0x0F00) >> 8) as usize] & 1;
                        self.v[((opcode & 0x0F00) >> 8) as usize] >>= 1;
                    },
                    0x0007 => {
                        self.v[0xF] = 
                            if self.v[((opcode & 0x00F0) >> 4) as usize] < self.v[((opcode & 0x0F00) >> 8) as usize] { 0 } 
                            else { 1 };
                        self.v[((opcode & 0x0F00) >> 8) as usize] =
                            self.v[((opcode & 0x00F0) >> 4) as usize].wrapping_sub(self.v[((opcode & 0x0F00) >> 8) as usize]);
                    },
                    0x000E => {
                        self.v[0xF] =
                            self.v[((opcode & 0x0F00) >> 8) as usize] & 0x80;
                        self.v[((opcode & 0x0F00) >> 8) as usize] <<= 1;
                    },
                    _ => ( println!("Unknown opcode [0x800N]: {:X}", opcode) )
                }
                self.pc += 2;
            },
            0x9000 => {
                if self.v[((opcode & 0x0F00) >> 8) as usize] != self.v[((opcode & 0x00F0) >> 4) as usize] {
                    self.pc += 2;
                }
                self.pc += 2;
            },
            0xA000 => {
                self.i = opcode & 0x0FFF;
                self.pc += 2;
            },
            0xB000 => {
                self.pc = (0x0FFF & opcode) + self.v[0x0] as u16;
            },
            0xC000 => {
                self.v[((opcode & 0x0F00) >> 8) as usize] =
                    ((opcode & 0x00FF) as u8) & rand::random::<u8>();
                self.pc += 2;
            },
            0xD000 => {
                let x = self.v[((opcode & 0x0F00) >> 8) as usize];
                let y = self.v[((opcode & 0x00F0) >> 4) as usize];
                let height = (opcode & 0x000F) as u8;
                self.v[0xF] = 0;

                for yline in 0..height {
                    let pixel = self.memory[(self.i + yline as u16) as usize];
                    for xline in 0..8_u8 {
                        if (pixel & (0x80 >> xline)) != 0 {
                            if self.gfx[(x + xline + ((y + yline) * 64)) as usize] == 1 {
                                self.v[0xF] = 1;
                            }
                            self.gfx[(x + xline + ((y + yline) * 64)) as usize] ^= 1;
                        }
                    }
                }
                self.pc += 2;
            },
            0xE000 => {
                match opcode & 0x00FF {
                    0x009E => {
                        if self.key[self.v[((opcode & 0x0F00) >> 8) as usize] as usize] != 0 {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    },
                    0x00A1 => {
                        if self.key[self.v[((opcode & 0x0F00) >> 8) as usize] as usize] == 0 {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    },
                    _ => ( println!("Unknown opcode [0xE000]: 0x{:X}", opcode) )
                }
            },
            0xF000 => {
                match opcode & 0x0FFF {
                    0x0007 => {
                        self.v[((opcode & 0x0F00) >> 8) as usize] = self.delay_timer;
                    },
                    0x000A => {
                        // Wait new key
                        self.v[((opcode & 0x0F00) >> 8) as usize] = 1;
                    },
                    0x0015 => {
                        self.delay_timer = self.v[((opcode & 0x0F00) >> 8) as usize];
                    },
                    0x0018 => {
                        self.sound_timer = self.v[((opcode & 0x0F00) >> 8) as usize];
                    },
                    0x001E => {
                        self.i += self.v[((opcode & 0x0F00) >> 8) as usize] as u16;
                    },
                    0x0029 => {
                        self.i = 5 * self.v[((opcode & 0x0F00) >> 8) as usize] as u16;
                    },
                    0x0033 => {
                        self.memory[self.i as usize]     = self.v[((opcode & 0x0F00) >> 8) as usize] / 100;
                        self.memory[self.i as usize + 1] = (self.v[((opcode & 0x0F00) >> 8) as usize] / 10) % 10;
                        self.memory[self.i as usize + 2] = self.v[((opcode & 0x0F00) >> 8) as usize] % 10;
                    },
                    0x0055 => {
                        let mut i = self.i as usize;
                        for x in 0..=((opcode & 0x0F00) >> 8) as usize {
                            self.memory[i] = self.v[x];
                            i += 1;
                        }
                    },
                    0x0065 => {
                        let mut i = self.i as usize;
                        for x in 0..=((opcode & 0x0F00) >> 8) as usize {
                            self.v[x] = self.memory[i];
                            i += 1;
                        }
                    },
                    _ => ( println!("Unknown opcode [0xF000]: 0x{:X}", opcode) )
                }
                self.pc += 2;
            }
            _ => ( println!("Unknown opcode: 0x{:X}", opcode) )
        }
        
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
}
