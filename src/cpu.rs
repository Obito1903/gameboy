use std::u8;

use crate::{
    memory::{MemoryBus, MemoryBusClient},
    opcodes::Instruction,
    ppu::PPU,
};

#[derive(Debug, Clone, Copy)]
pub struct FlagsRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl FlagsRegister {
    fn new() -> Self {
        Self {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false,
        }
    }
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION
            | (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION
            | (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION
            | (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Register {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister,
    pub h: u8,
    pub l: u8,
}

impl Register {
    fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister::new(),
            h: 0,
            l: 0,
        }
    }

    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | (<FlagsRegister as Into<u8>>::into(self.f) as u16)
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value as u8).into();
    }

    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | (self.c as u16)
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | (self.e as u16)
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | (self.l as u16)
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }
}

// #[derive(Debug, Clone, Copy)]
// pub enum JoypadButton {
//     A,
//     B,
//     Select,
//     Start,
//     Right,
//     Left,
//     Up,
//     Down,
// }

pub struct CPU {
    pub registers: Register,
    pub program_counter: u16,
    pub stack_pointer: u16,
    pub interupt_master_enable: bool,
    pub memory_bus: MemoryBus,
    pub ppu: PPU,
    pub debug: bool,
    pub walk: bool,
    pub is_halted: bool,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Register::new(),
            program_counter: 0,
            stack_pointer: 0xFFFF,
            interupt_master_enable: false,
            memory_bus: MemoryBus::new(),
            ppu: PPU::new(),
            debug: false,
            walk: false,
            is_halted: false,
        }
    }

    #[inline]
    fn advance_pc(&mut self, nb_bytes: u8) {
        self.program_counter += nb_bytes as u16;
    }

    #[inline]
    fn read_instruction(&mut self) -> Option<u8> {
        let instruction = Instruction::from_byte(self, self.program_counter);
        if self.debug {
            println!(
                "PC: {:0004?} OP: {:#02x?} I: {:?}",
                self.program_counter,
                self.memory_bus.read_byte(self.program_counter),
                instruction
            );
        }

        match instruction {
            Instruction::STOP => {
                self.advance_pc(1);
                None
            }
            _ => {
                self.advance_pc(Instruction::nb_bytes(
                    self.memory_bus.read_byte(self.program_counter),
                ));
                let cycles = instruction.execute(self);
                Some(cycles)
            }
        }
    }

    #[inline]
    pub fn handle_interupt(&mut self) {
        if self.interupt_master_enable {
            if self.memory_bus.interupt_enable.v_blank && self.memory_bus.interupt_flags.v_blank {
                self.call(0x40);
                self.memory_bus.interupt_flags.v_blank = false;
            } else if self.memory_bus.interupt_enable.lcd_stat
                && self.memory_bus.interupt_flags.lcd_stat
            {
                self.call(0x48);
                self.memory_bus.interupt_flags.lcd_stat = false;
            } else if self.memory_bus.interupt_enable.timer && self.memory_bus.interupt_flags.timer
            {
                self.call(0x50);
                self.memory_bus.interupt_flags.timer = false;
            } else if self.memory_bus.interupt_enable.serial
                && self.memory_bus.interupt_flags.serial
            {
                self.call(0x58);
                self.memory_bus.interupt_flags.serial = false;
            } else if self.memory_bus.interupt_enable.joypad
                && self.memory_bus.interupt_flags.joypad
            {
                self.call(0x60);
                self.memory_bus.interupt_flags.joypad = false;
            }
        }
    }

    pub fn run(&mut self, mhz: f32) {
        let cycles_per_second = mhz * 1_000_000.0;

        loop {
            // handle interupts
            self.handle_interupt();
            match self.read_instruction() {
                Some(cycles) => {
                    let seconds = cycles as f32 / cycles_per_second;
                    std::thread::sleep(std::time::Duration::from_secs_f32(seconds));
                    self.ppu.run_for(&mut self.memory_bus, cycles);
                    self.memory_bus.client = MemoryBusClient::CPU;
                }
                None => break,
            }
            if self.walk {
                println!("CPU Registers: {:?}", self.registers);
                println!("CPU Program Counter: {:#06x?}", self.program_counter);
                println!("CPU Stack Pointer: {:#06x?}", self.stack_pointer);
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
            }
        }
    }

    /// Read instruction from memory and execute it
    /// The full set of steps is as follows:
    /// - Use the program counter to read the instruction byte from memory.
    /// - Translate the byte to one of the instances of the Instruction enum
    /// - If we can successfully translate the instruction call our execute method else panic which now returns the next program counter
    /// - Set this next program counter on our CPU
    // fn step(&mut self) {
    //     let mut opcode = self.memory_bus.read_byte(self.program_counter);
    //     let prefixed = opcode == 0xCB;
    //     if prefixed {
    //         opcode = self.memory_bus.read_next_byte();
    //     }
    //     let next_pc = if let Some(instruction) = Instruction::from_byte(opcode, prefixed) {
    //         self.execute(instruction)
    //     } else {
    //         panic!("Instruction {:?} not implemented", opcode)
    //     };
    // }

    fn jump(&mut self, should_jump: bool) -> u16 {
        todo!("Jump not implemented")
    }

    pub fn call(&mut self, address: u16) {
        self.push_word(self.program_counter);
        self.program_counter = address;
    }

    pub fn ret(&mut self, should_return: bool) -> u16 {
        if should_return {
            self.program_counter = self.pop_word();
        }
        self.program_counter
    }

    // Stack
    pub fn push_word(&mut self, value: u16) {
        self.stack_pointer -= 2;
        self.memory_bus.write_word(self.stack_pointer, value);
    }

    pub fn pop_word(&mut self) -> u16 {
        let value = self.memory_bus.read_word(self.stack_pointer);
        self.stack_pointer += 2;
        value
    }
}

// Tests
