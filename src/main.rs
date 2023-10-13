use std::fs;

use clap::Parser;
use gb::{self, cpu::CPU};

#[derive(Parser, Debug)]
#[command(author, about, version, long_about = None, name = "gb")]
struct Args {
    #[arg(short, long, default_value = "assets/bootroms/dmg.bin")]
    boot_rom: String,

    #[arg(short, long, default_value = "tetris.gb")]
    rom: String,

    #[arg(short, long)]
    debug: bool,

    #[arg(short, long, default_value = "4.194304")]
    cpu_speed: f32,
}

fn main() {
    let args = Args::parse();
    let mut cpu = CPU::new();

    // Load boot rom
    let boot_rom = fs::read(args.boot_rom).unwrap();
    cpu.memory_bus.load_boot_rom(&boot_rom);
    cpu.memory_bus.print_section(0x00, 0x100);

    if args.debug {
        cpu.debug = true;
    }
    cpu.run(args.cpu_speed);
    println!("Hello, world!");
}
