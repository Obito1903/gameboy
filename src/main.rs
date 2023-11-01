use std::fs;

use clap::Parser;
use gb::{self, cpu::CPU};
use raylib::prelude::*;

#[derive(Parser, Debug)]
#[command(author, about, version, long_about = None, name = "gb")]
struct Args {
    #[arg(short, long, default_value = "assets/bootroms/dmg.bin")]
    boot_rom: String,

    #[arg(short, long, default_value = "tetris.gb")]
    rom: String,

    #[arg(short, long)]
    debug: bool,

    #[arg(short, long)]
    walk: bool,

    #[arg(short, long, default_value = "4.194304")]
    cpu_speed: f32,
}

struct Gameboy {
    pub cpu: CPU,
    pub lcd_screen: gb::lcd_screen::LCDScreen,
}

impl Gameboy {
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            lcd_screen: gb::lcd_screen::LCDScreen::new(),
        }
    }

    pub fn init(&mut self, boot_rom:String, debug: bool, walk: bool) {
        // Load boot rom
        let boot_rom = fs::read("assets/bootroms/dmg.bin").unwrap();
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

fn main() {
    let args = Args::parse();

    let mut Gameboy = Gameboy::new();
    Gameboy.init(args.boot_rom, args.debug, args.walk);

    Gameboy.run(args.cpu_speed);
}