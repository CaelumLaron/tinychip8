extern crate piston_window;
use piston_window::*;

pub(crate) enum KeyMap {
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

pub(crate) struct Graphics {
    width: u32,
    pixel_size: u32,
    window: PistonWindow,
}

impl Graphics {
    pub(crate) fn with_size(width: u32, height: u32) -> Self {
        let pixel_size = 10;
        Self {
            width,
            pixel_size,
            window: WindowSettings::new("TinyChip8", [width * pixel_size, height * pixel_size])
                .build()
                .unwrap(),
        }
    }

    pub(crate) fn set_keys(&mut self, event: &Event, key: &mut [u8; 16]) {
        if let Some(but) = event.press_args() {
            match but {
                Button::Keyboard(Key::D1) => key[KeyMap::NumPad1 as usize] = 1,
                Button::Keyboard(Key::D2) => key[KeyMap::NumPad2 as usize] = 1,
                Button::Keyboard(Key::D3) => key[KeyMap::NumPad3 as usize] = 1,
                Button::Keyboard(Key::D4) => key[KeyMap::NumPad4 as usize] = 1,
                Button::Keyboard(Key::Q) => key[KeyMap::Q as usize] = 1,
                Button::Keyboard(Key::W) => key[KeyMap::W as usize] = 1,
                Button::Keyboard(Key::E) => key[KeyMap::E as usize] = 1,
                Button::Keyboard(Key::R) => key[KeyMap::R as usize] = 1,
                Button::Keyboard(Key::A) => key[KeyMap::A as usize] = 1,
                Button::Keyboard(Key::S) => key[KeyMap::S as usize] = 1,
                Button::Keyboard(Key::D) => key[KeyMap::D as usize] = 1,
                Button::Keyboard(Key::F) => key[KeyMap::F as usize] = 1,
                Button::Keyboard(Key::Z) => key[KeyMap::Z as usize] = 1,
                Button::Keyboard(Key::X) => key[KeyMap::X as usize] = 1,
                Button::Keyboard(Key::C) => key[KeyMap::C as usize] = 1,
                Button::Keyboard(Key::V) => key[KeyMap::V as usize] = 1,
                _ => (),
            }
        }
        if let Some(but) = event.release_args() {
            match but {
                Button::Keyboard(Key::D1) => key[KeyMap::NumPad1 as usize] = 0,
                Button::Keyboard(Key::D2) => key[KeyMap::NumPad2 as usize] = 0,
                Button::Keyboard(Key::D3) => key[KeyMap::NumPad3 as usize] = 0,
                Button::Keyboard(Key::D4) => key[KeyMap::NumPad4 as usize] = 0,
                Button::Keyboard(Key::Q) => key[KeyMap::Q as usize] = 0,
                Button::Keyboard(Key::W) => key[KeyMap::W as usize] = 0,
                Button::Keyboard(Key::E) => key[KeyMap::E as usize] = 0,
                Button::Keyboard(Key::R) => key[KeyMap::R as usize] = 0,
                Button::Keyboard(Key::A) => key[KeyMap::A as usize] = 0,
                Button::Keyboard(Key::S) => key[KeyMap::S as usize] = 0,
                Button::Keyboard(Key::D) => key[KeyMap::D as usize] = 0,
                Button::Keyboard(Key::F) => key[KeyMap::F as usize] = 0,
                Button::Keyboard(Key::Z) => key[KeyMap::Z as usize] = 0,
                Button::Keyboard(Key::X) => key[KeyMap::X as usize] = 0,
                Button::Keyboard(Key::C) => key[KeyMap::C as usize] = 0,
                Button::Keyboard(Key::V) => key[KeyMap::V as usize] = 0,
                _ => (),
            }
        }
    }

    pub(crate) fn wait_key(&mut self) -> usize {
        loop {
            if let Some(event) = self.window.next() {
                if let Some(but) = event.press_args() {
                    let key = match but {
                        Button::Keyboard(Key::NumPad1) => Some(KeyMap::NumPad1),
                        Button::Keyboard(Key::NumPad2) => Some(KeyMap::NumPad2),
                        Button::Keyboard(Key::NumPad3) => Some(KeyMap::NumPad3),
                        Button::Keyboard(Key::NumPad4) => Some(KeyMap::NumPad4),
                        Button::Keyboard(Key::Q) => Some(KeyMap::Q),
                        Button::Keyboard(Key::W) => Some(KeyMap::W),
                        Button::Keyboard(Key::E) => Some(KeyMap::E),
                        Button::Keyboard(Key::R) => Some(KeyMap::R),
                        Button::Keyboard(Key::A) => Some(KeyMap::A),
                        Button::Keyboard(Key::S) => Some(KeyMap::S),
                        Button::Keyboard(Key::D) => Some(KeyMap::D),
                        Button::Keyboard(Key::F) => Some(KeyMap::F),
                        Button::Keyboard(Key::Z) => Some(KeyMap::Z),
                        Button::Keyboard(Key::X) => Some(KeyMap::X),
                        Button::Keyboard(Key::C) => Some(KeyMap::C),
                        Button::Keyboard(Key::V) => Some(KeyMap::V),
                        _ => None,
                    };

                    if let Some(mapped_key) = key {
                        return mapped_key as usize;
                    }
                }
            }
        }
    }

    pub(crate) fn clear_screen(&mut self) {
        if let Some(event) = self.window.next() {
            self.window.draw_2d(&event, |_context, graphics, _device| {
                clear([0.0; 4], graphics);
            });
        }
    }

    pub(crate) fn tick(&mut self, gfx: &[u8], key: &mut [u8; 16]) {
        if let Some(event) = self.window.next() {
            let width = self.width;
            let pixel_size = self.pixel_size;
            self.window.draw_2d(&event, |context, graphics, _device| {
                clear([0.0; 4], graphics);
                for (idx, &value) in gfx.iter().enumerate() {
                    let x_line = idx as u32 % width;
                    let y_line = idx as u32 / width;

                    if value == 1 {
                        rectangle(
                            [1.0, 1.0, 1.0, 1.0],
                            [
                                (x_line * 10) as f64,
                                (y_line * 10) as f64,
                                pixel_size as f64,
                                pixel_size as f64,
                            ],
                            context.transform,
                            graphics,
                        );
                    }
                }
            });
            self.set_keys(&event, key);
        }
    }
}
