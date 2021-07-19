extern crate piston_window;
use piston_window::*;

pub enum KeyMap {
    NumPad1 = 0x1,
    NumPad2 = 0x2,
    NumPad3 = 0x3,
    NumPad4 = 0xC,
    Q = 0x4,
    W = 0x5,
    E = 0x6,
    R = 0xD,
    A = 0x7,
    S = 0x8,
    D = 0x9,
    F = 0xE,
    Z = 0xA,
    X = 0x0,
    C = 0xB,
    V = 0xF,
}

pub struct Graphics {
    width: u32,
    height: u32,
    pixel_size: u32,
    window: PistonWindow,
}

impl Graphics {
    pub fn with_size(width: u32, height: u32) -> Self {
        let pixel_size = 10;
        Self {
            width,
            height,
            pixel_size,
            window: WindowSettings::new("TinyChip8", [width * pixel_size, height * pixel_size])
                .exit_on_esc(true)
                .build()
                .unwrap(),
        }
    }

    pub fn read_key(&mut self) -> usize {
        if let Some(event) = self.window.next() {}
    }

    pub fn update_screen(&mut self, gfx: &[u8]) {
        if let Some(event) = self.window.next() {
            let width = self.width;
            let pixel_size = self.pixel_size;
            self.window.draw_2d(&event, |context, graphics, _device| {
                for idx in 0..gfx.len() {
                    let x_line = idx as u32 % width;
                    let y_line = idx as u32 / width;

                    if gfx[idx] == 1 {
                        rectangle(
                            [1.0, 1.0, 1.0, 1.0],
                            [
                                x_line as f64,
                                y_line as f64,
                                pixel_size as f64,
                                pixel_size as f64,
                            ],
                            context.transform,
                            graphics,
                        );
                    }
                }
            });
        }
    }

    pub fn clear_screen(&mut self) {
        if let Some(event) = self.window.next() {
            self.window.draw_2d(&event, |_context, graphics, _device| {
                clear([0.0; 4], graphics);
            });
        }
    }
}
