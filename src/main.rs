extern crate piston_window;

use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Tiny8", [640, 320])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.0; 4], graphics);
            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [200.0, 200.0, 10.0, 10.0],
                context.transform,
                graphics,
            );
        });
    }
}
