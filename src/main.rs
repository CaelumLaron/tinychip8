use tiny_chip8::*;

fn main() -> Result<(), String> {
    let mut emulator = TinyChip8::initialize();

    emulator.start("/Users/lucasrochadossantos/Downloads/pong.rom")?;

    Ok(())
}
