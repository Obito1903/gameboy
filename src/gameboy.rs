use crate::{cpu::CPU, lcd_screen::LCDScreen};
use std::fs;
pub struct Gameboy {
    pub cpu: CPU,
    pub lcd_screen: LCDScreen,
}

impl Gameboy {
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            lcd_screen: LCDScreen::new(),
        }
    }

    pub fn init(&mut self, boot_rom: String, debug: bool, walk: bool) {
        // Load boot rom
        let boot_rom = fs::read(boot_rom).unwrap();
        self.cpu.memory_bus.load_boot_rom(&boot_rom);
        self.cpu.memory_bus.print_section(0x00, 0x100);

        if debug {
            self.cpu.debug = true;
        }

        if walk {
            self.cpu.walk = true;
        }
    }

    pub fn run(&mut self, cpu_speed: f32) {
        let (mut rl, thread) = raylib::init()
            .size(160 * 3, 144 * 3)
            .title("Gameboy Emulator")
            .build();

        rl.set_target_fps(60);

        while !rl.window_should_close() {
            self.cpu.step(cpu_speed);
            self.lcd_screen.update(&self.cpu.memory_bus);
            self.lcd_screen.draw(&mut rl, &thread);
        }
    }
}
