use std::fs;

use clap::Parser;
use gb::{self, cpu::CPU, gameboy::Gameboy, ppu::PPU};
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

fn main() {
    let args = Args::parse();

    let mut gb = Gameboy::new();
    gb.init(args.boot_rom, args.debug, args.walk);

    gb.run(args.cpu_speed);
}