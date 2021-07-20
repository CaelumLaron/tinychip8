use crate::chip8_cpu::Chip8Cpu;

pub struct TinyChip8 {
    cpu: Chip8Cpu,
}

impl TinyChip8 {
    pub fn initialize() -> Self {
        Self {
            cpu: Chip8Cpu::start(),
        }
    }

    pub fn start(&mut self, game_path: &str) -> Result<(), String> {
        match self.cpu.load_game(game_path) {
            Ok(_) => println!("Game loaded!"),
            Err(error) => panic!("Problem when loadind the game {:?}", error),
        }

        self.cpu.emulate();

        Ok(())
    }
}
