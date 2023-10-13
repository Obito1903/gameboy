use std::{
    fmt::Display,
    ops::{Shl, Shr},
    u8,
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
pub struct InteruptsFlags {
    pub v_blank: bool,
    pub lcd_stat: bool,
    pub timer: bool,
    pub serial: bool,
    pub joypad: bool,
}

impl InteruptsFlags {
    fn new() -> Self {
        Self {
            v_blank: false,
            lcd_stat: false,
            timer: false,
            serial: false,
            joypad: false,
        }
    }
}

impl std::convert::From<InteruptsFlags> for u8 {
    fn from(flag: InteruptsFlags) -> u8 {
        (if flag.v_blank { 1 } else { 0 }) << 0
            | (if flag.lcd_stat { 1 } else { 0 }) << 1
            | (if flag.timer { 1 } else { 0 }) << 2
            | (if flag.serial { 1 } else { 0 }) << 3
            | (if flag.joypad { 1 } else { 0 }) << 4
    }
}

impl std::convert::From<u8> for InteruptsFlags {
    fn from(byte: u8) -> Self {
        let v_blank = ((byte >> 0) & 0b1) != 0;
        let lcd_stat = ((byte >> 1) & 0b1) != 0;
        let timer = ((byte >> 2) & 0b1) != 0;
        let serial = ((byte >> 3) & 0b1) != 0;
        let joypad = ((byte >> 4) & 0b1) != 0;

        InteruptsFlags {
            v_blank,
            lcd_stat,
            timer,
            serial,
            joypad,
        }
    }
}

pub struct JoypadFlags {
    pub buttons: bool,
    pub direction: bool,
    pub a: bool,
    pub b: bool,
    pub select: bool,
    pub start: bool,
    pub right: bool,
    pub left: bool,
    pub up: bool,
    pub down: bool,
}

impl JoypadFlags {
    fn new() -> Self {
        Self {
            buttons: false,
            direction: false,
            a: false,
            b: false,
            select: false,
            start: false,
            right: false,
            left: false,
            up: false,
            down: false,
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

#[derive(Debug, Clone, Copy)]
enum RegisterName {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

impl RegisterName {
    fn get(&self, cpu: &CPU) -> u8 {
        match self {
            Self::A => cpu.registers.a,
            Self::B => cpu.registers.b,
            Self::C => cpu.registers.c,
            Self::D => cpu.registers.d,
            Self::E => cpu.registers.e,
            Self::F => cpu.registers.f.into(),
            Self::H => cpu.registers.h,
            Self::L => cpu.registers.l,
        }
    }

    fn set(&self, cpu: &mut CPU, value: u8) {
        match self {
            Self::A => cpu.registers.a = value,
            Self::B => cpu.registers.b = value,
            Self::C => cpu.registers.c = value,
            Self::D => cpu.registers.d = value,
            Self::E => cpu.registers.e = value,
            Self::F => cpu.registers.f = value.into(),
            Self::H => cpu.registers.h = value,
            Self::L => cpu.registers.l = value,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum RegisterPair {
    AF,
    BC,
    DE,
    HL,
    SP,
}

impl RegisterPair {
    fn get(&self, cpu: &CPU) -> u16 {
        match self {
            Self::AF => cpu.registers.get_af(),
            Self::BC => cpu.registers.get_bc(),
            Self::DE => cpu.registers.get_de(),
            Self::HL => cpu.registers.get_hl(),
            Self::SP => cpu.stack_pointer,
        }
    }

    fn set(&self, cpu: &mut CPU, value: u16) {
        match self {
            Self::AF => cpu.registers.set_af(value),
            Self::BC => cpu.registers.set_bc(value),
            Self::DE => cpu.registers.set_de(value),
            Self::HL => cpu.registers.set_hl(value),
            Self::SP => cpu.stack_pointer = value,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum TargetSize {
    Bit(bool),
    Byte(u8),
    SignedByte(i8),
    Word(u16),
}

#[derive(Debug, Clone, Copy)]
enum FlagOperand {
    Zero,
    Subtract,
    HalfCarry,
    Carry,
    NZ,
    NC,
}

impl FlagOperand {
    fn get(&self, cpu: &CPU) -> bool {
        match self {
            Self::Zero => cpu.registers.f.zero,
            Self::Subtract => cpu.registers.f.subtract,
            Self::HalfCarry => cpu.registers.f.half_carry,
            Self::Carry => cpu.registers.f.carry,
            Self::NZ => !cpu.registers.f.zero,
            Self::NC => !cpu.registers.f.carry,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum OperandTypes {
    Flags(FlagOperand),
    Register(RegisterName),
    RegisterPair(RegisterPair),
    D8(u8),
    D16(u16),
    A8(u8),
    A16(u16),
    R8(i8),
    Memory(u16),
}

impl OperandTypes {
    fn set(&self, cpu: &mut CPU, value: TargetSize) {
        match self {
            Self::Flags(_) => panic!("Cannot set flags"),
            Self::Register(register) => match value {
                TargetSize::Byte(byte) => register.set(cpu, byte),
                _ => panic!("Cannot set register with word"),
            },
            Self::RegisterPair(register_pair) => match value {
                TargetSize::Word(word) => register_pair.set(cpu, word),
                _ => panic!("Cannot set register pair with byte"),
            },
            Self::D8(_) => panic!("Cannot set immediate value"),
            Self::D16(_) => panic!("Cannot set immediate value"),
            Self::A8(_) => panic!("Cannot set immediate value"),
            Self::A16(_) => panic!("Cannot set immediate value"),
            Self::R8(_) => panic!("Cannot set immediate value"),
            Self::Memory(address) => match value {
                TargetSize::Byte(byte) => {
                    cpu.memory_bus.write_byte(*address, byte);
                }
                TargetSize::SignedByte(byte) => {
                    cpu.memory_bus.write_byte(*address, byte as u8);
                }
                TargetSize::Word(word) => {
                    cpu.memory_bus.write_byte(*address, (word >> 8) as u8);
                    cpu.memory_bus.write_byte(address + 1, word as u8);
                }
                TargetSize::Bit(_) => panic!("Cannot set bit"),
            },
        }
    }

    fn get(&self, cpu: &CPU) -> TargetSize {
        match self {
            Self::Flags(_) => panic!("Cannot get flags"),
            Self::Register(register) => TargetSize::Byte(register.get(cpu)),
            Self::RegisterPair(register_pair) => TargetSize::Word(register_pair.get(cpu)),
            Self::D8(value) => TargetSize::Byte(*value),
            Self::D16(value) => TargetSize::Word(*value),
            Self::A8(value) => TargetSize::Word(0xFF00 | u16::from(*value)),
            Self::A16(value) => TargetSize::Word(*value),
            Self::R8(value) => TargetSize::SignedByte(*value),
            Self::Memory(address) => TargetSize::Byte(cpu.memory_bus.read_byte(*address)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    ADD(OperandTypes, OperandTypes),
    ADDSP(OperandTypes),
    ADC(OperandTypes, OperandTypes),
    AND(OperandTypes),
    BIT(u8, OperandTypes),
    CALL(Option<FlagOperand>, OperandTypes),
    CCF,
    CP(OperandTypes),
    CPL,
    DAA,
    DEC(OperandTypes),
    DI,
    EI,
    HALT,
    INC(OperandTypes),
    JR(Option<FlagOperand>, i8),
    JP(Option<FlagOperand>, OperandTypes),
    LD(OperandTypes, OperandTypes),
    NOP,
    OR(OperandTypes),
    POP(RegisterPair),
    PUSH(RegisterPair),
    RES(u8, OperandTypes),
    RET(Option<FlagOperand>),
    RETI,
    RL(OperandTypes),
    RLA,
    RLC(OperandTypes),
    RLCA,
    RR(OperandTypes),
    RRA,
    RRC(OperandTypes),
    RRCA,
    RST(OperandTypes),
    SBC(OperandTypes),
    SCF,
    SET(u8, OperandTypes),
    SLA(OperandTypes),
    SRA(OperandTypes),
    SRL(OperandTypes),
    SUB(OperandTypes),
    STOP,
    SWAP(OperandTypes),
    XOR(OperandTypes),

    PREFIX,
}

pub type Cycles = u8;
pub type InstrLength = u8;

impl Instruction {
    fn from_byte(cpu: &mut CPU, pc: u16) -> Self {
        let byte = cpu.memory_bus.read_byte(pc);
        match byte {
            0x00 => Self::NOP,
            0x01 => Self::LD(
                OperandTypes::RegisterPair(RegisterPair::BC),
                OperandTypes::D16(cpu.memory_bus.read_word(pc + 1)),
            ),
            0x02 => Self::LD(
                OperandTypes::Memory(RegisterPair::BC.get(cpu)),
                OperandTypes::Register(RegisterName::A),
            ),
            0x03 => Self::INC(OperandTypes::RegisterPair(RegisterPair::BC)),
            0x04 => Self::INC(OperandTypes::Register(RegisterName::B)),
            0x05 => Self::DEC(OperandTypes::Register(RegisterName::B)),
            0x06 => Self::LD(
                OperandTypes::Register(RegisterName::B),
                OperandTypes::D8(cpu.memory_bus.read_byte(pc + 1)),
            ),
            0x07 => Self::RLCA,
            0x08 => Self::LD(
                OperandTypes::Memory(cpu.memory_bus.read_word(pc + 1)),
                OperandTypes::RegisterPair(RegisterPair::SP),
            ),
            0x09 => Self::ADD(
                OperandTypes::RegisterPair(RegisterPair::HL),
                OperandTypes::RegisterPair(RegisterPair::BC),
            ),
            0x0A => Self::LD(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Memory(RegisterPair::BC.get(cpu)),
            ),
            0x0B => Self::DEC(OperandTypes::RegisterPair(RegisterPair::BC)),
            0x0C => Self::INC(OperandTypes::Register(RegisterName::C)),
            0x0D => Self::DEC(OperandTypes::Register(RegisterName::C)),
            0x0E => Self::LD(
                OperandTypes::Register(RegisterName::C),
                OperandTypes::D8(cpu.memory_bus.read_byte(pc + 1)),
            ),
            0x0F => Self::RRCA,

            0x10 => Self::STOP,
            0x11 => Self::LD(
                OperandTypes::RegisterPair(RegisterPair::DE),
                OperandTypes::D16(cpu.memory_bus.read_word(pc + 1)),
            ),
            0x12 => Self::LD(
                OperandTypes::Memory(RegisterPair::DE.get(cpu)),
                OperandTypes::Register(RegisterName::A),
            ),
            0x13 => Self::INC(OperandTypes::RegisterPair(RegisterPair::DE)),
            0x14 => Self::INC(OperandTypes::Register(RegisterName::D)),
            0x15 => Self::DEC(OperandTypes::Register(RegisterName::D)),
            0x16 => Self::LD(
                OperandTypes::Register(RegisterName::D),
                OperandTypes::D8(cpu.memory_bus.read_byte(pc + 1)),
            ),
            0x17 => Self::RLA,
            0x18 => Self::JR(None, cpu.memory_bus.read_byte(pc + 1) as i8),
            0x19 => Self::ADD(
                OperandTypes::RegisterPair(RegisterPair::HL),
                OperandTypes::RegisterPair(RegisterPair::DE),
            ),
            0x1A => Self::LD(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Memory(RegisterPair::DE.get(cpu)),
            ),
            0x1B => Self::DEC(OperandTypes::RegisterPair(RegisterPair::DE)),
            0x1C => Self::INC(OperandTypes::Register(RegisterName::E)),
            0x1D => Self::DEC(OperandTypes::Register(RegisterName::E)),
            0x1E => Self::LD(
                OperandTypes::Register(RegisterName::E),
                OperandTypes::D8(cpu.memory_bus.read_byte(pc + 1)),
            ),
            0x1F => Self::RRA,
            0x20 => Self::JR(
                Some(FlagOperand::NZ),
                cpu.memory_bus.read_byte(pc + 1) as i8,
            ),
            0x21 => Self::LD(
                OperandTypes::RegisterPair(RegisterPair::HL),
                OperandTypes::D16(cpu.memory_bus.read_word(pc + 1)),
            ),
            0x22 => Self::LD(
                OperandTypes::Memory(RegisterPair::HL.get(cpu) + 1),
                OperandTypes::Register(RegisterName::A),
            ),
            0x23 => Self::INC(OperandTypes::RegisterPair(RegisterPair::HL)),
            0x24 => Self::INC(OperandTypes::Register(RegisterName::H)),
            0x25 => Self::DEC(OperandTypes::Register(RegisterName::H)),
            0x26 => Self::LD(
                OperandTypes::Register(RegisterName::H),
                OperandTypes::D8(cpu.memory_bus.read_byte(pc + 1)),
            ),
            0x27 => Self::DAA,
            0x28 => Self::JR(
                Some(FlagOperand::Zero),
                cpu.memory_bus.read_byte(pc + 1) as i8,
            ),
            0x29 => Self::ADD(
                OperandTypes::RegisterPair(RegisterPair::HL),
                OperandTypes::RegisterPair(RegisterPair::HL),
            ),
            0x2A => Self::LD(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Memory(RegisterPair::HL.get(cpu) + 1),
            ),
            0x2B => Self::DEC(OperandTypes::RegisterPair(RegisterPair::HL)),
            0x2C => Self::INC(OperandTypes::Register(RegisterName::L)),
            0x2D => Self::DEC(OperandTypes::Register(RegisterName::L)),
            0x2E => Self::LD(
                OperandTypes::Register(RegisterName::L),
                OperandTypes::D8(cpu.memory_bus.read_byte(pc + 1)),
            ),
            0x2F => Self::CPL,
            0x30 => Self::JR(
                Some(FlagOperand::NC),
                cpu.memory_bus.read_byte(pc + 1) as i8,
            ),
            0x31 => Self::LD(
                OperandTypes::RegisterPair(RegisterPair::SP),
                OperandTypes::D16(cpu.memory_bus.read_word(pc + 1)),
            ),
            0x32 => Self::LD(
                OperandTypes::Memory(RegisterPair::HL.get(cpu) - 1),
                OperandTypes::Register(RegisterName::A),
            ),
            0x33 => Self::INC(OperandTypes::RegisterPair(RegisterPair::SP)),
            0x34 => Self::INC(OperandTypes::Memory(RegisterPair::HL.get(cpu))),
            0x35 => Self::DEC(OperandTypes::Memory(RegisterPair::HL.get(cpu))),
            0x36 => Self::LD(
                OperandTypes::Memory(RegisterPair::HL.get(cpu)),
                OperandTypes::D8(cpu.memory_bus.read_byte(pc + 1)),
            ),
            0x37 => Self::SCF,
            0x38 => Self::JR(
                Some(FlagOperand::Carry),
                cpu.memory_bus.read_byte(pc + 1) as i8,
            ),
            0x39 => Self::ADD(
                OperandTypes::RegisterPair(RegisterPair::HL),
                OperandTypes::RegisterPair(RegisterPair::SP),
            ),
            0x3A => Self::LD(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Memory(RegisterPair::HL.get(cpu) - 1),
            ),
            0x3B => Self::DEC(OperandTypes::RegisterPair(RegisterPair::SP)),
            0x3C => Self::INC(OperandTypes::Register(RegisterName::A)),
            0x3D => Self::DEC(OperandTypes::Register(RegisterName::A)),
            0x3E => Self::LD(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::D8(cpu.memory_bus.read_byte(pc + 1)),
            ),
            0x3F => Self::CCF,
            0x40 => Self::LD(
                OperandTypes::Register(RegisterName::B),
                OperandTypes::Register(RegisterName::B),
            ),
            0x41 => Self::LD(
                OperandTypes::Register(RegisterName::B),
                OperandTypes::Register(RegisterName::C),
            ),
            0x42 => Self::LD(
                OperandTypes::Register(RegisterName::B),
                OperandTypes::Register(RegisterName::D),
            ),
            0x43 => Self::LD(
                OperandTypes::Register(RegisterName::B),
                OperandTypes::Register(RegisterName::E),
            ),
            0x44 => Self::LD(
                OperandTypes::Register(RegisterName::B),
                OperandTypes::Register(RegisterName::H),
            ),
            0x45 => Self::LD(
                OperandTypes::Register(RegisterName::B),
                OperandTypes::Register(RegisterName::L),
            ),
            0x46 => Self::LD(
                OperandTypes::Register(RegisterName::B),
                OperandTypes::Memory(RegisterPair::HL.get(cpu)),
            ),
            0x47 => Self::LD(
                OperandTypes::Register(RegisterName::B),
                OperandTypes::Register(RegisterName::A),
            ),
            0x48 => Self::LD(
                OperandTypes::Register(RegisterName::C),
                OperandTypes::Register(RegisterName::B),
            ),
            0x49 => Self::LD(
                OperandTypes::Register(RegisterName::C),
                OperandTypes::Register(RegisterName::C),
            ),
            0x4A => Self::LD(
                OperandTypes::Register(RegisterName::C),
                OperandTypes::Register(RegisterName::D),
            ),
            0x4B => Self::LD(
                OperandTypes::Register(RegisterName::C),
                OperandTypes::Register(RegisterName::E),
            ),
            0x4C => Self::LD(
                OperandTypes::Register(RegisterName::C),
                OperandTypes::Register(RegisterName::H),
            ),
            0x4D => Self::LD(
                OperandTypes::Register(RegisterName::C),
                OperandTypes::Register(RegisterName::L),
            ),
            0x4E => Self::LD(
                OperandTypes::Register(RegisterName::C),
                OperandTypes::Memory(RegisterPair::HL.get(cpu)),
            ),
            0x4F => Self::LD(
                OperandTypes::Register(RegisterName::C),
                OperandTypes::Register(RegisterName::A),
            ),
            0x50 => Self::LD(
                OperandTypes::Register(RegisterName::D),
                OperandTypes::Register(RegisterName::B),
            ),
            0x51 => Self::LD(
                OperandTypes::Register(RegisterName::D),
                OperandTypes::Register(RegisterName::C),
            ),
            0x52 => Self::LD(
                OperandTypes::Register(RegisterName::D),
                OperandTypes::Register(RegisterName::D),
            ),
            0x53 => Self::LD(
                OperandTypes::Register(RegisterName::D),
                OperandTypes::Register(RegisterName::E),
            ),
            0x54 => Self::LD(
                OperandTypes::Register(RegisterName::D),
                OperandTypes::Register(RegisterName::H),
            ),
            0x55 => Self::LD(
                OperandTypes::Register(RegisterName::D),
                OperandTypes::Register(RegisterName::L),
            ),
            0x56 => Self::LD(
                OperandTypes::Register(RegisterName::D),
                OperandTypes::Memory(RegisterPair::HL.get(cpu)),
            ),
            0x57 => Self::LD(
                OperandTypes::Register(RegisterName::D),
                OperandTypes::Register(RegisterName::A),
            ),
            0x58 => Self::LD(
                OperandTypes::Register(RegisterName::E),
                OperandTypes::Register(RegisterName::B),
            ),
            0x59 => Self::LD(
                OperandTypes::Register(RegisterName::E),
                OperandTypes::Register(RegisterName::C),
            ),
            0x5A => Self::LD(
                OperandTypes::Register(RegisterName::E),
                OperandTypes::Register(RegisterName::D),
            ),
            0x5B => Self::LD(
                OperandTypes::Register(RegisterName::E),
                OperandTypes::Register(RegisterName::E),
            ),
            0x5C => Self::LD(
                OperandTypes::Register(RegisterName::E),
                OperandTypes::Register(RegisterName::H),
            ),
            0x5D => Self::LD(
                OperandTypes::Register(RegisterName::E),
                OperandTypes::Register(RegisterName::L),
            ),
            0x5E => Self::LD(
                OperandTypes::Register(RegisterName::E),
                OperandTypes::Memory(RegisterPair::HL.get(cpu)),
            ),
            0x5F => Self::LD(
                OperandTypes::Register(RegisterName::E),
                OperandTypes::Register(RegisterName::A),
            ),
            0x60 => Self::LD(
                OperandTypes::Register(RegisterName::H),
                OperandTypes::Register(RegisterName::B),
            ),
            0x61 => Self::LD(
                OperandTypes::Register(RegisterName::H),
                OperandTypes::Register(RegisterName::C),
            ),
            0x62 => Self::LD(
                OperandTypes::Register(RegisterName::H),
                OperandTypes::Register(RegisterName::D),
            ),
            0x63 => Self::LD(
                OperandTypes::Register(RegisterName::H),
                OperandTypes::Register(RegisterName::E),
            ),
            0x64 => Self::LD(
                OperandTypes::Register(RegisterName::H),
                OperandTypes::Register(RegisterName::H),
            ),
            0x65 => Self::LD(
                OperandTypes::Register(RegisterName::H),
                OperandTypes::Register(RegisterName::L),
            ),
            0x66 => Self::LD(
                OperandTypes::Register(RegisterName::H),
                OperandTypes::Memory(RegisterPair::HL.get(cpu)),
            ),
            0x67 => Self::LD(
                OperandTypes::Register(RegisterName::H),
                OperandTypes::Register(RegisterName::A),
            ),
            0x68 => Self::LD(
                OperandTypes::Register(RegisterName::L),
                OperandTypes::Register(RegisterName::B),
            ),
            0x69 => Self::LD(
                OperandTypes::Register(RegisterName::L),
                OperandTypes::Register(RegisterName::C),
            ),
            0x6A => Self::LD(
                OperandTypes::Register(RegisterName::L),
                OperandTypes::Register(RegisterName::D),
            ),
            0x6B => Self::LD(
                OperandTypes::Register(RegisterName::L),
                OperandTypes::Register(RegisterName::E),
            ),
            0x6C => Self::LD(
                OperandTypes::Register(RegisterName::L),
                OperandTypes::Register(RegisterName::H),
            ),
            0x6D => Self::LD(
                OperandTypes::Register(RegisterName::L),
                OperandTypes::Register(RegisterName::L),
            ),
            0x6E => Self::LD(
                OperandTypes::Register(RegisterName::L),
                OperandTypes::Memory(RegisterPair::HL.get(cpu)),
            ),
            0x6F => Self::LD(
                OperandTypes::Register(RegisterName::L),
                OperandTypes::Register(RegisterName::A),
            ),
            0x70 => Self::LD(
                OperandTypes::Memory(RegisterPair::HL.get(cpu)),
                OperandTypes::Register(RegisterName::B),
            ),
            0x71 => Self::LD(
                OperandTypes::Memory(RegisterPair::HL.get(cpu)),
                OperandTypes::Register(RegisterName::C),
            ),
            0x72 => Self::LD(
                OperandTypes::Memory(RegisterPair::HL.get(cpu)),
                OperandTypes::Register(RegisterName::D),
            ),
            0x73 => Self::LD(
                OperandTypes::Memory(RegisterPair::HL.get(cpu)),
                OperandTypes::Register(RegisterName::E),
            ),
            0x74 => Self::LD(
                OperandTypes::Memory(RegisterPair::HL.get(cpu)),
                OperandTypes::Register(RegisterName::H),
            ),
            0x75 => Self::LD(
                OperandTypes::Memory(RegisterPair::HL.get(cpu)),
                OperandTypes::Register(RegisterName::L),
            ),
            0x76 => Self::HALT,
            0x77 => Self::LD(
                OperandTypes::Memory(RegisterPair::HL.get(cpu)),
                OperandTypes::Register(RegisterName::A),
            ),
            0x78 => Self::LD(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Register(RegisterName::B),
            ),
            0x79 => Self::LD(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Register(RegisterName::C),
            ),
            0x7A => Self::LD(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Register(RegisterName::D),
            ),
            0x7B => Self::LD(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Register(RegisterName::E),
            ),
            0x7C => Self::LD(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Register(RegisterName::H),
            ),
            0x7D => Self::LD(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Register(RegisterName::L),
            ),
            0x7E => Self::LD(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Memory(RegisterPair::HL.get(cpu)),
            ),
            0x7F => Self::LD(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Register(RegisterName::A),
            ),
            0x80 => Self::ADD(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Register(RegisterName::B),
            ),
            0x81 => Self::ADD(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Register(RegisterName::C),
            ),
            0x82 => Self::ADD(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Register(RegisterName::D),
            ),
            0x83 => Self::ADD(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Register(RegisterName::E),
            ),
            0x84 => Self::ADD(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Register(RegisterName::H),
            ),
            0x85 => Self::ADD(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Register(RegisterName::L),
            ),
            0x86 => Self::ADD(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Memory(RegisterPair::HL.get(cpu)),
            ),
            0x87 => Self::ADD(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Register(RegisterName::A),
            ),
            0x88 => Self::ADC(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Register(RegisterName::B),
            ),
            0x89 => Self::ADC(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Register(RegisterName::C),
            ),
            0x8A => Self::ADC(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Register(RegisterName::D),
            ),
            0x8B => Self::ADC(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Register(RegisterName::E),
            ),
            0x8C => Self::ADC(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Register(RegisterName::H),
            ),
            0x8D => Self::ADC(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Register(RegisterName::L),
            ),
            0x8E => Self::ADC(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Memory(RegisterPair::HL.get(cpu)),
            ),
            0x8F => Self::ADC(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Register(RegisterName::A),
            ),
            0x90 => Self::SUB(OperandTypes::Register(RegisterName::B)),
            0x91 => Self::SUB(OperandTypes::Register(RegisterName::C)),
            0x92 => Self::SUB(OperandTypes::Register(RegisterName::D)),
            0x93 => Self::SUB(OperandTypes::Register(RegisterName::E)),
            0x94 => Self::SUB(OperandTypes::Register(RegisterName::H)),
            0x95 => Self::SUB(OperandTypes::Register(RegisterName::L)),
            0x96 => Self::SUB(OperandTypes::Memory(RegisterPair::HL.get(cpu))),
            0x97 => Self::SUB(OperandTypes::Register(RegisterName::A)),
            0x98 => Self::SBC(OperandTypes::Register(RegisterName::B)),
            0x99 => Self::SBC(OperandTypes::Register(RegisterName::C)),
            0x9A => Self::SBC(OperandTypes::Register(RegisterName::D)),
            0x9B => Self::SBC(OperandTypes::Register(RegisterName::E)),
            0x9C => Self::SBC(OperandTypes::Register(RegisterName::H)),
            0x9D => Self::SBC(OperandTypes::Register(RegisterName::L)),
            0x9E => Self::SBC(OperandTypes::Memory(RegisterPair::HL.get(cpu))),
            0x9F => Self::SBC(OperandTypes::Register(RegisterName::A)),
            0xA0 => Self::AND(OperandTypes::Register(RegisterName::B)),
            0xA1 => Self::AND(OperandTypes::Register(RegisterName::C)),
            0xA2 => Self::AND(OperandTypes::Register(RegisterName::D)),
            0xA3 => Self::AND(OperandTypes::Register(RegisterName::E)),
            0xA4 => Self::AND(OperandTypes::Register(RegisterName::H)),
            0xA5 => Self::AND(OperandTypes::Register(RegisterName::L)),
            0xA6 => Self::AND(OperandTypes::Memory(RegisterPair::HL.get(cpu))),
            0xA7 => Self::AND(OperandTypes::Register(RegisterName::A)),
            0xA8 => Self::XOR(OperandTypes::Register(RegisterName::B)),
            0xA9 => Self::XOR(OperandTypes::Register(RegisterName::C)),
            0xAA => Self::XOR(OperandTypes::Register(RegisterName::D)),
            0xAB => Self::XOR(OperandTypes::Register(RegisterName::E)),
            0xAC => Self::XOR(OperandTypes::Register(RegisterName::H)),
            0xAD => Self::XOR(OperandTypes::Register(RegisterName::L)),
            0xAE => Self::XOR(OperandTypes::Memory(RegisterPair::HL.get(cpu))),
            0xAF => Self::XOR(OperandTypes::Register(RegisterName::A)),
            0xB0 => Self::OR(OperandTypes::Register(RegisterName::B)),
            0xB1 => Self::OR(OperandTypes::Register(RegisterName::C)),
            0xB2 => Self::OR(OperandTypes::Register(RegisterName::D)),
            0xB3 => Self::OR(OperandTypes::Register(RegisterName::E)),
            0xB4 => Self::OR(OperandTypes::Register(RegisterName::H)),
            0xB5 => Self::OR(OperandTypes::Register(RegisterName::L)),
            0xB6 => Self::OR(OperandTypes::Memory(RegisterPair::HL.get(cpu))),
            0xB7 => Self::OR(OperandTypes::Register(RegisterName::A)),
            0xB8 => Self::CP(OperandTypes::Register(RegisterName::B)),
            0xB9 => Self::CP(OperandTypes::Register(RegisterName::C)),
            0xBA => Self::CP(OperandTypes::Register(RegisterName::D)),
            0xBB => Self::CP(OperandTypes::Register(RegisterName::E)),
            0xBC => Self::CP(OperandTypes::Register(RegisterName::H)),
            0xBD => Self::CP(OperandTypes::Register(RegisterName::L)),
            0xBE => Self::CP(OperandTypes::Memory(RegisterPair::HL.get(cpu))),
            0xBF => Self::CP(OperandTypes::Register(RegisterName::A)),
            0xC0 => Self::RET(Some(FlagOperand::NZ)),
            0xC1 => Self::POP(RegisterPair::BC),
            0xC2 => Self::JP(
                Some(FlagOperand::NZ),
                OperandTypes::D16(cpu.memory_bus.read_word(pc + 1)),
            ),
            0xC3 => Self::JP(None, OperandTypes::D16(cpu.memory_bus.read_word(pc + 1))),
            0xC4 => Self::CALL(
                Some(FlagOperand::NZ),
                OperandTypes::D16(cpu.memory_bus.read_word(pc + 1)),
            ),
            0xC5 => Self::PUSH(RegisterPair::BC),
            0xC6 => Self::ADD(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::D8(cpu.memory_bus.read_byte(pc + 1)),
            ),
            0xC7 => Self::RST(OperandTypes::D8(0x00)),
            0xC8 => Self::RET(Some(FlagOperand::Zero)),
            0xC9 => Self::RET(None),
            0xCA => Self::JP(
                Some(FlagOperand::Zero),
                OperandTypes::D16(cpu.memory_bus.read_word(pc + 1)),
            ),
            0xCC => Self::CALL(
                Some(FlagOperand::Zero),
                OperandTypes::D16(cpu.memory_bus.read_word(pc + 1)),
            ),
            0xCD => Self::CALL(None, OperandTypes::D16(cpu.memory_bus.read_word(pc + 1))),
            0xCE => Self::ADC(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::D8(cpu.memory_bus.read_byte(pc + 1)),
            ),
            0xCF => Self::RST(OperandTypes::D8(0x08)),
            0xD0 => Self::RET(Some(FlagOperand::NC)),
            0xD1 => Self::POP(RegisterPair::DE),
            0xD2 => Self::JP(
                Some(FlagOperand::NC),
                OperandTypes::D16(cpu.memory_bus.read_word(pc + 1)),
            ),
            0xD4 => Self::CALL(
                Some(FlagOperand::NC),
                OperandTypes::D16(cpu.memory_bus.read_word(pc + 1)),
            ),
            0xD5 => Self::PUSH(RegisterPair::DE),
            0xD6 => Self::SUB(OperandTypes::D8(cpu.memory_bus.read_byte(pc + 1))),
            0xD7 => Self::RST(OperandTypes::D8(0x10)),
            0xD8 => Self::RET(Some(FlagOperand::Carry)),
            0xD9 => Self::RETI,
            0xDA => Self::JP(
                Some(FlagOperand::Carry),
                OperandTypes::D16(cpu.memory_bus.read_word(pc + 1)),
            ),
            0xDC => Self::CALL(
                Some(FlagOperand::Carry),
                OperandTypes::D16(cpu.memory_bus.read_word(pc + 1)),
            ),
            0xDE => Self::SBC(OperandTypes::D8(cpu.memory_bus.read_byte(pc + 1))),
            0xDF => Self::RST(OperandTypes::D8(0x18)),
            0xE0 => Self::LD(
                OperandTypes::Memory(0xFF00 + cpu.memory_bus.read_byte(pc + 1) as u16),
                OperandTypes::Register(RegisterName::A),
            ),
            0xE1 => Self::POP(RegisterPair::HL),
            0xE2 => Self::LD(
                OperandTypes::Memory(0xFF00 + RegisterName::C.get(cpu) as u16),
                OperandTypes::Register(RegisterName::A),
            ),
            0xE5 => Self::PUSH(RegisterPair::HL),
            0xE6 => Self::AND(OperandTypes::D8(cpu.memory_bus.read_byte(pc + 1))),
            0xE7 => Self::RST(OperandTypes::D8(0x20)),
            0xE8 => Self::ADD(
                OperandTypes::RegisterPair(RegisterPair::SP),
                OperandTypes::R8(cpu.memory_bus.read_byte(pc + 1) as i8),
            ),
            0xE9 => Self::JP(None, OperandTypes::RegisterPair(RegisterPair::HL)),
            0xEA => Self::LD(
                OperandTypes::Memory(cpu.memory_bus.read_word(pc + 1)),
                OperandTypes::Register(RegisterName::A),
            ),
            0xEE => Self::XOR(OperandTypes::D8(cpu.memory_bus.read_byte(pc + 1))),
            0xEF => Self::RST(OperandTypes::D8(0x28)),
            0xF0 => Self::LD(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Memory(0xFF00 + cpu.memory_bus.read_byte(pc + 1) as u16),
            ),
            0xF1 => Self::POP(RegisterPair::AF),
            0xF2 => Self::LD(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Memory(0xFF00 + RegisterName::C.get(cpu) as u16),
            ),
            0xF3 => Self::DI,
            0xF5 => Self::PUSH(RegisterPair::AF),
            0xF6 => Self::OR(OperandTypes::D8(cpu.memory_bus.read_byte(pc + 1))),
            0xF7 => Self::RST(OperandTypes::D8(0x30)),
            0xF8 => Self::LD(
                OperandTypes::RegisterPair(RegisterPair::HL),
                OperandTypes::D16(pc + (cpu.memory_bus.read_byte(pc + 1) as u16)),
            ),
            0xF9 => Self::LD(
                OperandTypes::RegisterPair(RegisterPair::SP),
                OperandTypes::RegisterPair(RegisterPair::HL),
            ),
            0xFA => Self::LD(
                OperandTypes::Register(RegisterName::A),
                OperandTypes::Memory(cpu.memory_bus.read_word(pc + 1)),
            ),
            0xFB => Self::EI,
            0xFE => Self::CP(OperandTypes::D8(cpu.memory_bus.read_byte(pc + 1))),
            0xFF => Self::RST(OperandTypes::D8(0x38)),
            0xCB => {
                let next_byte = cpu.memory_bus.read_next_byte(pc);
                match next_byte {
                    0x00 => Self::RLC(OperandTypes::Register(RegisterName::B)),
                    0x01 => Self::RLC(OperandTypes::Register(RegisterName::C)),
                    0x02 => Self::RLC(OperandTypes::Register(RegisterName::D)),
                    0x03 => Self::RLC(OperandTypes::Register(RegisterName::E)),
                    0x04 => Self::RLC(OperandTypes::Register(RegisterName::H)),
                    0x05 => Self::RLC(OperandTypes::Register(RegisterName::L)),
                    0x06 => Self::RLC(OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0x07 => Self::RLC(OperandTypes::Register(RegisterName::A)),
                    0x08 => Self::RRC(OperandTypes::Register(RegisterName::B)),
                    0x09 => Self::RRC(OperandTypes::Register(RegisterName::C)),
                    0x0A => Self::RRC(OperandTypes::Register(RegisterName::D)),
                    0x0B => Self::RRC(OperandTypes::Register(RegisterName::E)),
                    0x0C => Self::RRC(OperandTypes::Register(RegisterName::H)),
                    0x0D => Self::RRC(OperandTypes::Register(RegisterName::L)),
                    0x0E => Self::RRC(OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0x0F => Self::RRC(OperandTypes::Register(RegisterName::A)),
                    0x10 => Self::RL(OperandTypes::Register(RegisterName::B)),
                    0x11 => Self::RL(OperandTypes::Register(RegisterName::C)),
                    0x12 => Self::RL(OperandTypes::Register(RegisterName::D)),
                    0x13 => Self::RL(OperandTypes::Register(RegisterName::E)),
                    0x14 => Self::RL(OperandTypes::Register(RegisterName::H)),
                    0x15 => Self::RL(OperandTypes::Register(RegisterName::L)),
                    0x16 => Self::RL(OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0x17 => Self::RL(OperandTypes::Register(RegisterName::A)),
                    0x18 => Self::RR(OperandTypes::Register(RegisterName::B)),
                    0x19 => Self::RR(OperandTypes::Register(RegisterName::C)),
                    0x1A => Self::RR(OperandTypes::Register(RegisterName::D)),
                    0x1B => Self::RR(OperandTypes::Register(RegisterName::E)),
                    0x1C => Self::RR(OperandTypes::Register(RegisterName::H)),
                    0x1D => Self::RR(OperandTypes::Register(RegisterName::L)),
                    0x1E => Self::RR(OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0x1F => Self::RR(OperandTypes::Register(RegisterName::A)),
                    0x20 => Self::SLA(OperandTypes::Register(RegisterName::B)),
                    0x21 => Self::SLA(OperandTypes::Register(RegisterName::C)),
                    0x22 => Self::SLA(OperandTypes::Register(RegisterName::D)),
                    0x23 => Self::SLA(OperandTypes::Register(RegisterName::E)),
                    0x24 => Self::SLA(OperandTypes::Register(RegisterName::H)),
                    0x25 => Self::SLA(OperandTypes::Register(RegisterName::L)),
                    0x26 => Self::SLA(OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0x27 => Self::SLA(OperandTypes::Register(RegisterName::A)),
                    0x28 => Self::SRA(OperandTypes::Register(RegisterName::B)),
                    0x29 => Self::SRA(OperandTypes::Register(RegisterName::C)),
                    0x2A => Self::SRA(OperandTypes::Register(RegisterName::D)),
                    0x2B => Self::SRA(OperandTypes::Register(RegisterName::E)),
                    0x2C => Self::SRA(OperandTypes::Register(RegisterName::H)),
                    0x2D => Self::SRA(OperandTypes::Register(RegisterName::L)),
                    0x2E => Self::SRA(OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0x2F => Self::SRA(OperandTypes::Register(RegisterName::A)),
                    0x30 => Self::SWAP(OperandTypes::Register(RegisterName::B)),
                    0x31 => Self::SWAP(OperandTypes::Register(RegisterName::C)),
                    0x32 => Self::SWAP(OperandTypes::Register(RegisterName::D)),
                    0x33 => Self::SWAP(OperandTypes::Register(RegisterName::E)),
                    0x34 => Self::SWAP(OperandTypes::Register(RegisterName::H)),
                    0x35 => Self::SWAP(OperandTypes::Register(RegisterName::L)),
                    0x36 => Self::SWAP(OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0x37 => Self::SWAP(OperandTypes::Register(RegisterName::A)),
                    0x38 => Self::SRL(OperandTypes::Register(RegisterName::B)),
                    0x39 => Self::SRL(OperandTypes::Register(RegisterName::C)),
                    0x3A => Self::SRL(OperandTypes::Register(RegisterName::D)),
                    0x3B => Self::SRL(OperandTypes::Register(RegisterName::E)),
                    0x3C => Self::SRL(OperandTypes::Register(RegisterName::H)),
                    0x3D => Self::SRL(OperandTypes::Register(RegisterName::L)),
                    0x3E => Self::SRL(OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0x3F => Self::SRL(OperandTypes::Register(RegisterName::A)),
                    0x40 => Self::BIT(0, OperandTypes::Register(RegisterName::B)),
                    0x41 => Self::BIT(0, OperandTypes::Register(RegisterName::C)),
                    0x42 => Self::BIT(0, OperandTypes::Register(RegisterName::D)),
                    0x43 => Self::BIT(0, OperandTypes::Register(RegisterName::E)),
                    0x44 => Self::BIT(0, OperandTypes::Register(RegisterName::H)),
                    0x45 => Self::BIT(0, OperandTypes::Register(RegisterName::L)),
                    0x46 => Self::BIT(0, OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0x47 => Self::BIT(0, OperandTypes::Register(RegisterName::A)),
                    0x48 => Self::BIT(1, OperandTypes::Register(RegisterName::B)),
                    0x49 => Self::BIT(1, OperandTypes::Register(RegisterName::C)),
                    0x4A => Self::BIT(1, OperandTypes::Register(RegisterName::D)),
                    0x4B => Self::BIT(1, OperandTypes::Register(RegisterName::E)),
                    0x4C => Self::BIT(1, OperandTypes::Register(RegisterName::H)),
                    0x4D => Self::BIT(1, OperandTypes::Register(RegisterName::L)),
                    0x4E => Self::BIT(1, OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0x4F => Self::BIT(1, OperandTypes::Register(RegisterName::A)),
                    0x50 => Self::BIT(2, OperandTypes::Register(RegisterName::B)),
                    0x51 => Self::BIT(2, OperandTypes::Register(RegisterName::C)),
                    0x52 => Self::BIT(2, OperandTypes::Register(RegisterName::D)),
                    0x53 => Self::BIT(2, OperandTypes::Register(RegisterName::E)),
                    0x54 => Self::BIT(2, OperandTypes::Register(RegisterName::H)),
                    0x55 => Self::BIT(2, OperandTypes::Register(RegisterName::L)),
                    0x56 => Self::BIT(2, OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0x57 => Self::BIT(2, OperandTypes::Register(RegisterName::A)),
                    0x58 => Self::BIT(3, OperandTypes::Register(RegisterName::B)),
                    0x59 => Self::BIT(3, OperandTypes::Register(RegisterName::C)),
                    0x5A => Self::BIT(3, OperandTypes::Register(RegisterName::D)),
                    0x5B => Self::BIT(3, OperandTypes::Register(RegisterName::E)),
                    0x5C => Self::BIT(3, OperandTypes::Register(RegisterName::H)),
                    0x5D => Self::BIT(3, OperandTypes::Register(RegisterName::L)),
                    0x5E => Self::BIT(3, OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0x5F => Self::BIT(3, OperandTypes::Register(RegisterName::A)),
                    0x60 => Self::BIT(4, OperandTypes::Register(RegisterName::B)),
                    0x61 => Self::BIT(4, OperandTypes::Register(RegisterName::C)),
                    0x62 => Self::BIT(4, OperandTypes::Register(RegisterName::D)),
                    0x63 => Self::BIT(4, OperandTypes::Register(RegisterName::E)),
                    0x64 => Self::BIT(4, OperandTypes::Register(RegisterName::H)),
                    0x65 => Self::BIT(4, OperandTypes::Register(RegisterName::L)),
                    0x66 => Self::BIT(4, OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0x67 => Self::BIT(4, OperandTypes::Register(RegisterName::A)),
                    0x68 => Self::BIT(5, OperandTypes::Register(RegisterName::B)),
                    0x69 => Self::BIT(5, OperandTypes::Register(RegisterName::C)),
                    0x6A => Self::BIT(5, OperandTypes::Register(RegisterName::D)),
                    0x6B => Self::BIT(5, OperandTypes::Register(RegisterName::E)),
                    0x6C => Self::BIT(5, OperandTypes::Register(RegisterName::H)),
                    0x6D => Self::BIT(5, OperandTypes::Register(RegisterName::L)),
                    0x6E => Self::BIT(5, OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0x6F => Self::BIT(5, OperandTypes::Register(RegisterName::A)),
                    0x70 => Self::BIT(6, OperandTypes::Register(RegisterName::B)),
                    0x71 => Self::BIT(6, OperandTypes::Register(RegisterName::C)),
                    0x72 => Self::BIT(6, OperandTypes::Register(RegisterName::D)),
                    0x73 => Self::BIT(6, OperandTypes::Register(RegisterName::E)),
                    0x74 => Self::BIT(6, OperandTypes::Register(RegisterName::H)),
                    0x75 => Self::BIT(6, OperandTypes::Register(RegisterName::L)),
                    0x76 => Self::BIT(6, OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0x77 => Self::BIT(6, OperandTypes::Register(RegisterName::A)),
                    0x78 => Self::BIT(7, OperandTypes::Register(RegisterName::B)),
                    0x79 => Self::BIT(7, OperandTypes::Register(RegisterName::C)),
                    0x7A => Self::BIT(7, OperandTypes::Register(RegisterName::D)),
                    0x7B => Self::BIT(7, OperandTypes::Register(RegisterName::E)),
                    0x7C => Self::BIT(7, OperandTypes::Register(RegisterName::H)),
                    0x7D => Self::BIT(7, OperandTypes::Register(RegisterName::L)),
                    0x7E => Self::BIT(7, OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0x7F => Self::BIT(7, OperandTypes::Register(RegisterName::A)),
                    0x80 => Self::RES(0, OperandTypes::Register(RegisterName::B)),
                    0x81 => Self::RES(0, OperandTypes::Register(RegisterName::C)),
                    0x82 => Self::RES(0, OperandTypes::Register(RegisterName::D)),
                    0x83 => Self::RES(0, OperandTypes::Register(RegisterName::E)),
                    0x84 => Self::RES(0, OperandTypes::Register(RegisterName::H)),
                    0x85 => Self::RES(0, OperandTypes::Register(RegisterName::L)),
                    0x86 => Self::RES(0, OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0x87 => Self::RES(0, OperandTypes::Register(RegisterName::A)),
                    0x88 => Self::RES(1, OperandTypes::Register(RegisterName::B)),
                    0x89 => Self::RES(1, OperandTypes::Register(RegisterName::C)),
                    0x8A => Self::RES(1, OperandTypes::Register(RegisterName::D)),
                    0x8B => Self::RES(1, OperandTypes::Register(RegisterName::E)),
                    0x8C => Self::RES(1, OperandTypes::Register(RegisterName::H)),
                    0x8D => Self::RES(1, OperandTypes::Register(RegisterName::L)),
                    0x8E => Self::RES(1, OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0x8F => Self::RES(1, OperandTypes::Register(RegisterName::A)),
                    0x90 => Self::RES(2, OperandTypes::Register(RegisterName::B)),
                    0x91 => Self::RES(2, OperandTypes::Register(RegisterName::C)),
                    0x92 => Self::RES(2, OperandTypes::Register(RegisterName::D)),
                    0x93 => Self::RES(2, OperandTypes::Register(RegisterName::E)),
                    0x94 => Self::RES(2, OperandTypes::Register(RegisterName::H)),
                    0x95 => Self::RES(2, OperandTypes::Register(RegisterName::L)),
                    0x96 => Self::RES(2, OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0x97 => Self::RES(2, OperandTypes::Register(RegisterName::A)),
                    0x98 => Self::RES(3, OperandTypes::Register(RegisterName::B)),
                    0x99 => Self::RES(3, OperandTypes::Register(RegisterName::C)),
                    0x9A => Self::RES(3, OperandTypes::Register(RegisterName::D)),
                    0x9B => Self::RES(3, OperandTypes::Register(RegisterName::E)),
                    0x9C => Self::RES(3, OperandTypes::Register(RegisterName::H)),
                    0x9D => Self::RES(3, OperandTypes::Register(RegisterName::L)),
                    0x9E => Self::RES(3, OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0x9F => Self::RES(3, OperandTypes::Register(RegisterName::A)),
                    0xA0 => Self::RES(4, OperandTypes::Register(RegisterName::B)),
                    0xA1 => Self::RES(4, OperandTypes::Register(RegisterName::C)),
                    0xA2 => Self::RES(4, OperandTypes::Register(RegisterName::D)),
                    0xA3 => Self::RES(4, OperandTypes::Register(RegisterName::E)),
                    0xA4 => Self::RES(4, OperandTypes::Register(RegisterName::H)),
                    0xA5 => Self::RES(4, OperandTypes::Register(RegisterName::L)),
                    0xA6 => Self::RES(4, OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0xA7 => Self::RES(4, OperandTypes::Register(RegisterName::A)),
                    0xA8 => Self::RES(5, OperandTypes::Register(RegisterName::B)),
                    0xA9 => Self::RES(5, OperandTypes::Register(RegisterName::C)),
                    0xAA => Self::RES(5, OperandTypes::Register(RegisterName::D)),
                    0xAB => Self::RES(5, OperandTypes::Register(RegisterName::E)),
                    0xAC => Self::RES(5, OperandTypes::Register(RegisterName::H)),
                    0xAD => Self::RES(5, OperandTypes::Register(RegisterName::L)),
                    0xAE => Self::RES(5, OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0xAF => Self::RES(5, OperandTypes::Register(RegisterName::A)),
                    0xB0 => Self::RES(6, OperandTypes::Register(RegisterName::B)),
                    0xB1 => Self::RES(6, OperandTypes::Register(RegisterName::C)),
                    0xB2 => Self::RES(6, OperandTypes::Register(RegisterName::D)),
                    0xB3 => Self::RES(6, OperandTypes::Register(RegisterName::E)),
                    0xB4 => Self::RES(6, OperandTypes::Register(RegisterName::H)),
                    0xB5 => Self::RES(6, OperandTypes::Register(RegisterName::L)),
                    0xB6 => Self::RES(6, OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0xB7 => Self::RES(6, OperandTypes::Register(RegisterName::A)),
                    0xB8 => Self::RES(7, OperandTypes::Register(RegisterName::B)),
                    0xB9 => Self::RES(7, OperandTypes::Register(RegisterName::C)),
                    0xBA => Self::RES(7, OperandTypes::Register(RegisterName::D)),
                    0xBB => Self::RES(7, OperandTypes::Register(RegisterName::E)),
                    0xBC => Self::RES(7, OperandTypes::Register(RegisterName::H)),
                    0xBD => Self::RES(7, OperandTypes::Register(RegisterName::L)),
                    0xBE => Self::RES(7, OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0xBF => Self::RES(7, OperandTypes::Register(RegisterName::A)),
                    0xC0 => Self::SET(0, OperandTypes::Register(RegisterName::B)),
                    0xC1 => Self::SET(0, OperandTypes::Register(RegisterName::C)),
                    0xC2 => Self::SET(0, OperandTypes::Register(RegisterName::D)),
                    0xC3 => Self::SET(0, OperandTypes::Register(RegisterName::E)),
                    0xC4 => Self::SET(0, OperandTypes::Register(RegisterName::H)),
                    0xC5 => Self::SET(0, OperandTypes::Register(RegisterName::L)),
                    0xC6 => Self::SET(0, OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0xC7 => Self::SET(0, OperandTypes::Register(RegisterName::A)),
                    0xC8 => Self::SET(1, OperandTypes::Register(RegisterName::B)),
                    0xC9 => Self::SET(1, OperandTypes::Register(RegisterName::C)),
                    0xCA => Self::SET(1, OperandTypes::Register(RegisterName::D)),
                    0xCB => Self::SET(1, OperandTypes::Register(RegisterName::E)),
                    0xCC => Self::SET(1, OperandTypes::Register(RegisterName::H)),
                    0xCD => Self::SET(1, OperandTypes::Register(RegisterName::L)),
                    0xCE => Self::SET(1, OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0xCF => Self::SET(1, OperandTypes::Register(RegisterName::A)),
                    0xD0 => Self::SET(2, OperandTypes::Register(RegisterName::B)),
                    0xD1 => Self::SET(2, OperandTypes::Register(RegisterName::C)),
                    0xD2 => Self::SET(2, OperandTypes::Register(RegisterName::D)),
                    0xD3 => Self::SET(2, OperandTypes::Register(RegisterName::E)),
                    0xD4 => Self::SET(2, OperandTypes::Register(RegisterName::H)),
                    0xD5 => Self::SET(2, OperandTypes::Register(RegisterName::L)),
                    0xD6 => Self::SET(2, OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0xD7 => Self::SET(2, OperandTypes::Register(RegisterName::A)),
                    0xD8 => Self::SET(3, OperandTypes::Register(RegisterName::B)),
                    0xD9 => Self::SET(3, OperandTypes::Register(RegisterName::C)),
                    0xDA => Self::SET(3, OperandTypes::Register(RegisterName::D)),
                    0xDB => Self::SET(3, OperandTypes::Register(RegisterName::E)),
                    0xDC => Self::SET(3, OperandTypes::Register(RegisterName::H)),
                    0xDD => Self::SET(3, OperandTypes::Register(RegisterName::L)),
                    0xDE => Self::SET(3, OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0xDF => Self::SET(3, OperandTypes::Register(RegisterName::A)),
                    0xE0 => Self::SET(4, OperandTypes::Register(RegisterName::B)),
                    0xE1 => Self::SET(4, OperandTypes::Register(RegisterName::C)),
                    0xE2 => Self::SET(4, OperandTypes::Register(RegisterName::D)),
                    0xE3 => Self::SET(4, OperandTypes::Register(RegisterName::E)),
                    0xE4 => Self::SET(4, OperandTypes::Register(RegisterName::H)),
                    0xE5 => Self::SET(4, OperandTypes::Register(RegisterName::L)),
                    0xE6 => Self::SET(4, OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0xE7 => Self::SET(4, OperandTypes::Register(RegisterName::A)),
                    0xE8 => Self::SET(5, OperandTypes::Register(RegisterName::B)),
                    0xE9 => Self::SET(5, OperandTypes::Register(RegisterName::C)),
                    0xEA => Self::SET(5, OperandTypes::Register(RegisterName::D)),
                    0xEB => Self::SET(5, OperandTypes::Register(RegisterName::E)),
                    0xEC => Self::SET(5, OperandTypes::Register(RegisterName::H)),
                    0xED => Self::SET(5, OperandTypes::Register(RegisterName::L)),
                    0xEE => Self::SET(5, OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0xEF => Self::SET(5, OperandTypes::Register(RegisterName::A)),
                    0xF0 => Self::SET(6, OperandTypes::Register(RegisterName::B)),
                    0xF1 => Self::SET(6, OperandTypes::Register(RegisterName::C)),
                    0xF2 => Self::SET(6, OperandTypes::Register(RegisterName::D)),
                    0xF3 => Self::SET(6, OperandTypes::Register(RegisterName::E)),
                    0xF4 => Self::SET(6, OperandTypes::Register(RegisterName::H)),
                    0xF5 => Self::SET(6, OperandTypes::Register(RegisterName::L)),
                    0xF6 => Self::SET(6, OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0xF7 => Self::SET(6, OperandTypes::Register(RegisterName::A)),
                    0xF8 => Self::SET(7, OperandTypes::Register(RegisterName::B)),
                    0xF9 => Self::SET(7, OperandTypes::Register(RegisterName::C)),
                    0xFA => Self::SET(7, OperandTypes::Register(RegisterName::D)),
                    0xFB => Self::SET(7, OperandTypes::Register(RegisterName::E)),
                    0xFC => Self::SET(7, OperandTypes::Register(RegisterName::H)),
                    0xFD => Self::SET(7, OperandTypes::Register(RegisterName::L)),
                    0xFE => Self::SET(7, OperandTypes::Memory(RegisterPair::HL.get(cpu))),
                    0xFF => Self::SET(7, OperandTypes::Register(RegisterName::A)),

                    _ => panic!("Prefixed instruction {:?} not implemented", next_byte),
                }
            }

            _ => panic!("Instruction {:#02x?} not implemented", byte),
        }
    }

    fn nb_bytes(opcode: u8) -> u8 {
        match opcode {
            0x01 => 3,
            0x06 => 2,
            0x08 => 3,
            0x0E => 2,
            0x10 => 2,
            0x11 => 3,
            0x16 => 2,
            0x18 => 2,
            0x1E => 2,
            0x20 => 2,
            0x21 => 3,
            0x26 => 2,
            0x28 => 2,
            0x2E => 2,
            0x30 => 2,
            0x31 => 3,
            0x36 => 2,
            0x38 => 2,
            0x3E => 2,
            0xC2 => 3,
            0xC3 => 3,
            0xC4 => 3,
            0xC6 => 2,
            0xCA => 3,
            0xCC => 3,
            0xCD => 3,
            0xCE => 2,
            0xD2 => 3,
            0xD4 => 3,
            0xD6 => 2,
            0xDA => 3,
            0xDC => 3,
            0xDE => 2,
            0xE0 => 2,
            0xE2 => 2,
            0xE6 => 2,
            0xEA => 3,
            0xEE => 2,
            0xF0 => 2,
            0xF2 => 2,
            0xF6 => 2,
            0xFA => 3,
            0xFE => 2,
            0xCB => 2,
            _ => 1,
        }
    }

    // Return the clock cycles taken by the instruction
    fn execute(&self, cpu: &mut CPU) -> u8 {
        if !cpu.is_halted {
            match self {
                Self::ADD(target, source) => Self::add(cpu, *target, *source),
                Self::ADC(target, source) => Self::adc(cpu, *target, *source),
                Self::AND(source) => Self::and(cpu, *source),
                Self::BIT(bit, source) => Self::bit(cpu, *bit, *source),
                Self::CALL(condition, address) => Self::call(cpu, *condition, *address),
                Self::CCF => Self::ccf(cpu),
                Self::CP(source) => Self::cp(cpu, *source),
                Self::CPL => Self::cpl(cpu),
                Self::DAA => Self::daa(cpu),
                Self::DEC(target) => Self::dec(cpu, *target),
                Self::DI => Self::di(cpu),
                Self::EI => Self::ei(cpu),
                Self::HALT => Self::halt(cpu),
                Self::INC(target) => Self::inc(cpu, *target),
                Self::JR(condition, offset) => Self::jr(cpu, *condition, *offset),
                Self::JP(condition, address) => Self::jp(cpu, *condition, *address),
                Self::LD(target, source) => Self::ld(cpu, *target, *source),
                Self::NOP => Self::nop(cpu),
                Self::OR(source) => Self::or(cpu, *source),
                Self::POP(target) => Self::pop(cpu, *target),
                Self::PUSH(source) => Self::push(cpu, *source),
                Self::RES(bit, target) => Self::res(cpu, *bit, *target),
                Self::RET(condition) => Self::ret(cpu, *condition),
                Self::RETI => Self::reti(cpu),
                Self::RL(target) => Self::rl(cpu, *target),
                Self::RLA => Self::rla(cpu),
                Self::RLC(target) => Self::rlc(cpu, *target),
                Self::RLCA => Self::rlca(cpu),
                Self::RR(target) => Self::rr(cpu, *target),
                Self::RRA => Self::rra(cpu),
                Self::RRC(target) => Self::rrc(cpu, *target),
                Self::RRCA => Self::rrca(cpu),
                Self::RST(address) => Self::rst(cpu, *address),
                Self::SBC(source) => Self::sbc(cpu, *source),
                Self::SCF => Self::scf(cpu),
                Self::SET(bit, target) => Self::set(cpu, *bit, *target),
                Self::SLA(target) => Self::sla(cpu, *target),
                Self::SRA(target) => Self::sra(cpu, *target),
                Self::SRL(target) => Self::srl(cpu, *target),
                Self::STOP => Self::stop(cpu),
                Self::SUB(source) => Self::sub(cpu, *source),
                Self::SWAP(target) => Self::swap(cpu, *target),
                Self::XOR(source) => Self::xor(cpu, *source),
                _ => 0,
            }
        } else {
            // CPU is halted, do nothing
            0
        }
    }

    #[inline]
    fn add(cpu: &mut CPU, target: OperandTypes, source: OperandTypes) -> u8 {
        let mut cycle = 4;
        let (zero, overflow) = match target.get(cpu) {
            TargetSize::Byte(target_value) => {
                let (new_value, overflow) = match source.get(cpu) {
                    TargetSize::Byte(source_value) => target_value.overflowing_add(source_value),
                    TargetSize::Word(source_value) => {
                        cycle = cycle + 4;
                        target_value.overflowing_add(source_value as u8)
                    }
                    TargetSize::Bit(_) => panic!("Cannot ADD bit"),
                    source => panic!("Cannot ADD {:?} to byte", source),
                };
                target.set(cpu, TargetSize::Byte(new_value));
                ((new_value == 0), overflow)
            }
            TargetSize::Word(target_value) => {
                let (new_value, overflow) = match source.get(cpu) {
                    TargetSize::Byte(source_value) => {
                        target_value.overflowing_add(source_value as u16)
                    }

                    TargetSize::Word(source_value) => {
                        cycle = cycle + 4;
                        target_value.overflowing_add(source_value)
                    }
                    TargetSize::Bit(_) => panic!("Cannot ADD bit"),
                    source => panic!("Cannot ADD {:?} to word", source),
                };
                target.set(cpu, TargetSize::Word(new_value));
                ((new_value == 0), overflow)
            }
            TargetSize::Bit(_) => panic!("Cannot ADD bit"),
            target => panic!("Cannot ADD {:?} to byte", target),
        };
        cpu.registers.f.zero = zero;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = overflow;
        cycle
    }

    #[inline]
    fn add_sp(cpu: &mut CPU, offset: OperandTypes) -> u8 {
        todo!("ADD SP not implemented")
    }

    #[inline]
    fn adc(cpu: &mut CPU, target: OperandTypes, source: OperandTypes) -> u8 {
        let (zero, overflow) = match target.get(cpu) {
            TargetSize::Byte(target_value) => {
                let (mut new_value, overflow) = match source.get(cpu) {
                    TargetSize::Byte(source_value) => {
                        cpu.registers.f.half_carry =
                            (target_value & 0xF) + (source_value & 0xF) > 0xF;
                        target_value.overflowing_add(source_value)
                    }
                    _ => panic!("ADC Only available for bytes sources"),
                };
                let mut overflow2 = false;
                if cpu.registers.f.carry {
                    (new_value, overflow2) = new_value.overflowing_add(1);
                }

                target.set(cpu, TargetSize::Byte(new_value));
                ((new_value == 0), overflow || overflow2)
            }
            _ => panic!("ADC is only available for bytes targets"),
        };
        cpu.registers.f.zero = zero;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = overflow;
        match source {
            OperandTypes::D8(_) => 8,
            _ => 4,
        }
    }

    #[inline]
    fn and(cpu: &mut CPU, source: OperandTypes) -> u8 {
        let new_value = match source.get(cpu) {
            TargetSize::Byte(source_value) => cpu.registers.a & source_value,
            _ => panic!("AND only available for bytes sources"),
        };
        cpu.registers.a = new_value;

        cpu.registers.f.zero = new_value == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = false;
        match source {
            OperandTypes::D8(_) => 8,
            _ => 4,
        }
    }

    #[inline]
    fn bit(cpu: &mut CPU, bit: u8, source: OperandTypes) -> u8 {
        let (is_set, cycles) = match source.get(cpu) {
            TargetSize::Byte(source_value) => (source_value & (1 << bit) != 0, 8),
            TargetSize::Word(source_value) => (source_value & (1 << bit) != 0, 16),
            _ => panic!("BIT only available for bytes sources"),
        };
        cpu.registers.f.zero = is_set;
        cpu.registers.f.subtract = false;
        cpu.registers.f.half_carry = true;
        cycles
    }

    #[inline]
    fn call(cpu: &mut CPU, condition: Option<FlagOperand>, address: OperandTypes) -> u8 {
        let call_impl = |cpu: &mut CPU, address: OperandTypes| {
            let address = match address {
                OperandTypes::D16(address) => address,
                OperandTypes::A16(address) => address,
                _ => panic!("CALL only available for 16 bits addresses"),
            };
            cpu.call(address);
        };

        match condition {
            Some(flag) => {
                if flag.get(cpu) {
                    call_impl(cpu, address);
                    24
                } else {
                    12
                }
            }
            None => {
                call_impl(cpu, address);
                24
            }
        }
    }

    #[inline]
    fn ccf(cpu: &mut CPU) -> u8 {
        let cycle = 4;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = !cpu.registers.f.carry;
        cpu.registers.f.half_carry = false;
        cycle
    }

    ///Subtracts from the 8-bit A register, the 8-bit register r, and updates flags based on the result.
    #[inline]
    fn cp(cpu: &mut CPU, source: OperandTypes) -> u8 {
        let cycle = 8;
        let (zero, overflow) = match source.get(cpu) {
            TargetSize::Byte(source_value) => {
                let (new_value, overflow) = cpu.registers.a.overflowing_sub(source_value);
                ((new_value == 0), overflow)
            }
            _ => panic!("This instruction is only available for bytes"),
        };
        cpu.registers.f.subtract = true;
        cpu.registers.f.zero = zero;
        cpu.registers.f.carry = overflow;
        cycle
    }

    #[inline]
    fn cpl(cpu: &mut CPU) -> u8 {
        let cycle = 4;
        cpu.registers.a = !cpu.registers.a;
        cpu.registers.f.subtract = true;
        cpu.registers.f.half_carry = true;
        cycle
    }

    #[inline]
    fn daa(cpu: &mut CPU) -> u8 {
        if !cpu.registers.f.zero {
            if cpu.registers.f.carry || cpu.registers.a > 0x99 {
                cpu.registers.a = cpu.registers.a.wrapping_add(0x60);
                cpu.registers.f.carry = true;
            }

            if cpu.registers.f.half_carry || (cpu.registers.a & 0x0F) > 0x09 {
                cpu.registers.a = cpu.registers.a.wrapping_add(0x06);
            }
        } else {
            if cpu.registers.f.carry {
                cpu.registers.a = cpu.registers.a.wrapping_sub(0x90);
                cpu.registers.f.carry = false;
            }

            if cpu.registers.f.half_carry {
                cpu.registers.a = cpu.registers.a.wrapping_sub(0x6);
            }
        }

        cpu.registers.f.zero = cpu.registers.a == 0;
        cpu.registers.f.half_carry = false;

        4
    }

    #[inline]
    fn dec(cpu: &mut CPU, target: OperandTypes) -> u8 {
        let cycle = 4;
        let (zero, overflow) = match target.get(cpu) {
            TargetSize::Byte(target_value) => {
                let (new_value, overflow) = target_value.overflowing_sub(1);
                target.set(cpu, TargetSize::Byte(new_value));
                ((new_value == 0), overflow)
            }
            TargetSize::Word(target_value) => {
                let (new_value, overflow) = target_value.overflowing_sub(1);
                target.set(cpu, TargetSize::Word(new_value));
                ((new_value == 0), overflow)
            }
            TargetSize::Bit(_) => panic!("Cannot DEC bit"),
            target => panic!("Cannot DEC {:?} to byte", target),
        };
        cpu.registers.f.zero = zero;
        cpu.registers.f.subtract = true;
        cpu.registers.f.carry = overflow;
        cycle
    }

    #[inline]
    fn di(cpu: &mut CPU) -> u8 {
        cpu.interupt_master_enable = false;
        4
    }

    #[inline]
    fn ei(cpu: &mut CPU) -> u8 {
        cpu.interupt_master_enable = true;
        4
    }

    #[inline]
    fn halt(cpu: &mut CPU) -> u8 {
        cpu.is_halted = true;
        4
    }

    #[inline]
    fn inc(cpu: &mut CPU, target: OperandTypes) -> u8 {
        let cycle = 4;

        let (zero, overflow) = match target.get(cpu) {
            TargetSize::Byte(target_value) => {
                let (new_value, overflow) = target_value.overflowing_add(1);
                target.set(cpu, TargetSize::Byte(new_value));
                ((new_value == 0), overflow)
            }
            TargetSize::Word(target_value) => {
                let (new_value, overflow) = target_value.overflowing_add(1);
                target.set(cpu, TargetSize::Word(new_value));
                ((new_value == 0), overflow)
            }
            TargetSize::Bit(_) => panic!("Cannot INC bit"),
            target => panic!("Cannot INC {:?} to byte", target),
        };

        cpu.registers.f.zero = zero;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = overflow;

        cycle
    }

    #[inline]
    fn jr(cpu: &mut CPU, condition: Option<FlagOperand>, offset: i8) -> u8 {
        match condition {
            Some(c) => {
                if c.get(cpu) {
                    cpu.program_counter = cpu.program_counter.wrapping_add_signed(offset as i16);
                    12
                } else {
                    8
                }
            }
            None => {
                cpu.program_counter = cpu.program_counter.wrapping_add_signed(offset as i16);
                12
            }
        }
    }

    #[inline]
    fn jp(cpu: &mut CPU, condition: Option<FlagOperand>, address: OperandTypes) -> u8 {
        todo!("JP not implemented")
    }

    #[inline]
    fn ld(cpu: &mut CPU, target: OperandTypes, source: OperandTypes) -> u8 {
        target.set(cpu, source.get(cpu));
        // TODO : Fix cpu cylces
        match target {
            OperandTypes::Register(_) => match source {
                OperandTypes::Register(_) => 4,
                OperandTypes::Memory(_) => 8,
                OperandTypes::D8(_) => 8,
                source => panic!("Cannot LD from {:?} to {:?}", source, target),
            },
            OperandTypes::RegisterPair(_) => match source {
                OperandTypes::RegisterPair(_) => 8,
                OperandTypes::D16(_) => 12,
                source => panic!("Cannot LD from {:?} to {:?}", source, target),
            },
            OperandTypes::Memory(address) => match source {
                OperandTypes::Register(_) => {
                    if (address & 0xFF00) == 0xFF00 {
                        12
                    } else {
                        16
                    }
                }
                source => panic!("Cannot LD to memory from {:?}", source),
            },
            target => panic!("LD not implemented for this target {:?}", target),
        }

        // todo!("LD not implemented")
    }

    #[inline]
    fn nop(cpu: &mut CPU) -> u8 {
        todo!("NOP not implemented")
    }

    #[inline]
    fn or(cpu: &mut CPU, source: OperandTypes) -> u8 {
        match source.get(cpu) {
            TargetSize::Byte(source_value) => {
                cpu.registers.a = cpu.registers.a | source_value;
            }
            _ => panic!("OR only available for bytes sources"),
        };
        cpu.registers.f.zero = cpu.registers.a == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = false;
        cpu.registers.f.half_carry = false;
        4
    }

    #[inline]
    fn pop(cpu: &mut CPU, target: RegisterPair) -> u8 {
        let value = cpu.pop_word();
        target.set(cpu, value);
        12
    }

    #[inline]
    fn push(cpu: &mut CPU, target: RegisterPair) -> u8 {
        cpu.push_word(target.get(cpu));
        16
    }

    #[inline]
    fn res(cpu: &mut CPU, bit: u8, target: OperandTypes) -> u8 {
        let cycles = match target.get(cpu) {
            TargetSize::Byte(target_value) => {
                let bitmask = !(1 << bit);
                let new_value = target_value & bitmask;
                target.set(cpu, TargetSize::Byte(new_value));
                8
            }
            TargetSize::Word(target_value) => {
                let bitmask = !(1 << bit);
                let new_value = target_value & bitmask;
                target.set(cpu, TargetSize::Word(new_value));
                16
            }
            _ => panic!("BIT only available for bytes sources"),
        };
        cycles
    }

    #[inline]
    fn ret(cpu: &mut CPU, condition: Option<FlagOperand>) -> u8 {
        todo!("RET not implemented")
    }

    #[inline]
    fn reti(cpu: &mut CPU) -> u8 {
        todo!("RETI not implemented")
    }

    #[inline]
    fn rl(cpu: &mut CPU, target: OperandTypes) -> u8 {
        let cycle = 8;

        let (new_carry, mut new_value) = match target.get(cpu) {
            TargetSize::Byte(target_value) => {
                let new_value = target_value << 1;
                (target_value & 0b1000_0000 != 0, new_value)
            }
            _ => panic!("RL is only available for bytes targets"),
        };

        target.set(cpu, TargetSize::Byte(new_value));

        cpu.registers.f.zero = false;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = new_carry;
        cycle
    }

    #[inline]
    fn rla(cpu: &mut CPU) -> u8 {
        let cycle = 4;

        let (new_carry, mut new_value) = {
            let new_value = cpu.registers.a << 1;
            (cpu.registers.a & 0b1000_0000 != 0, new_value)
        };

        cpu.registers.a = new_value;
        cpu.registers.f.zero = false;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = new_carry;
        cycle
    }

    #[inline]
    fn rlc(cpu: &mut CPU, target: OperandTypes) -> u8 {
        let cycle = 8;

        let (mut new_value, new_carry) = match target.get(cpu) {
            TargetSize::Byte(target_value) => target_value.overflowing_shl(1),
            _ => panic!("RLC is only available for bytes targets"),
        };

        if cpu.registers.f.carry {
            new_value = new_value | 0b0000_0001;
        }

        target.set(cpu, TargetSize::Byte(new_value));
        cpu.registers.f.zero = false;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = new_carry;
        cycle
    }

    #[inline]
    fn rlca(cpu: &mut CPU) -> u8 {
        let cycle = 4;

        let (mut new_value, new_carry) = cpu.registers.a.overflowing_shl(1);

        if cpu.registers.f.carry {
            new_value = new_value | 0b0000_0001;
        }

        cpu.registers.a = new_value;
        cpu.registers.f.zero = false;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = new_carry;
        cycle
    }

    #[inline]
    fn rr(cpu: &mut CPU, target: OperandTypes) -> u8 {
        let cycle = 8;

        let (new_carry, new_value) = match target.get(cpu) {
            TargetSize::Byte(target_value) => {
                let new_value = target_value >> 1;
                (target_value & 0b0000_0001 != 0, new_value)
            }
            _ => panic!("RR is only available for bytes targets"),
        };

        target.set(cpu, TargetSize::Byte(new_value));

        cpu.registers.f.zero = false;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = new_carry;
        cycle
    }

    #[inline]
    fn rra(cpu: &mut CPU) -> u8 {
        let cycle = 4;

        let (new_carry, new_value) = {
            let new_value = cpu.registers.a >> 1;
            (cpu.registers.a & 0b0000_0001 != 0, new_value)
        };

        cpu.registers.a = new_value;
        cpu.registers.f.zero = false;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = new_carry;
        cycle
    }

    #[inline]
    fn rrc(cpu: &mut CPU, target: OperandTypes) -> u8 {
        let cycle = 8;

        let (mut new_value, new_carry) = match target.get(cpu) {
            TargetSize::Byte(target_value) => target_value.overflowing_shr(1),
            _ => panic!("RRC is only available for bytes targets"),
        };

        if cpu.registers.f.carry {
            new_value = new_value | 0b1000_0000;
        }

        target.set(cpu, TargetSize::Byte(new_value));
        cpu.registers.f.zero = false;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = new_carry;
        cycle
    }

    #[inline]
    fn rrca(cpu: &mut CPU) -> u8 {
        let cycle = 4;

        let (mut new_value, new_carry) = cpu.registers.a.overflowing_shr(1);

        if cpu.registers.f.carry {
            new_value = new_value | 0b1000_0000;
        }
        cpu.registers.a = new_value;
        cpu.registers.f.zero = false;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = new_carry;
        cycle
    }

    #[inline]
    fn rst(cpu: &mut CPU, address: OperandTypes) -> u8 {
        todo!("RST not implemented")
    }

    #[inline]
    fn sbc(cpu: &mut CPU, source: OperandTypes) -> u8 {
        let (new_value, overflow) = match source.get(cpu) {
            TargetSize::Byte(source_value) => {
                let (new_value, overflow) = cpu
                    .registers
                    .a
                    .overflowing_sub(cpu.registers.f.carry as u8 + source_value);
                (new_value, overflow)
            }
            _ => panic!("SBC only available for bytes sources"),
        };

        cpu.registers.f.zero = new_value == 0;
        cpu.registers.f.subtract = true;
        cpu.registers.f.carry = overflow;
        let old_val = match source.get(cpu) {
            TargetSize::Byte(source_value) => source_value,
            _ => panic!("SBC only available for bytes sources"),
        };

        cpu.registers.f.half_carry = (cpu.registers.a & 0xF) + (old_val & 0xF) > 0xF;

        cpu.registers.a = new_value;
        match source {
            OperandTypes::D8(_) => 8,
            _ => 4,
        }
    }

    #[inline]
    fn scf(cpu: &mut CPU) -> u8 {
        let cycle = 4;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = true;
        cpu.registers.f.half_carry = false;
        cycle
    }

    #[inline]
    fn set(cpu: &mut CPU, bit: u8, target: OperandTypes) -> u8 {
        let cycles = match target.get(cpu) {
            TargetSize::Byte(target_value) => {
                let bitmask = 1 << bit;
                let new_value = target_value | bitmask;
                target.set(cpu, TargetSize::Byte(new_value));
                8
            }
            TargetSize::Word(target_value) => {
                let bitmask = 1 << bit;
                let new_value = target_value | bitmask;
                target.set(cpu, TargetSize::Word(new_value));
                16
            }
            _ => panic!("BIT only available for bytes sources"),
        };
        cycles
    }

    ///Shift n left into Carry. LSB of target set to 0
    #[inline]
    fn sla(cpu: &mut CPU, target: OperandTypes) -> u8 {
        let (new_carry, new_value) = match target.get(cpu) {
            TargetSize::Byte(target_value) => {
                let new_carry = target_value & 0b1000_0000 != 0;
                let new_value = target_value << 1;
                (new_carry, new_value)
            }
            _ => panic!("SLA is only available for bytes targets"),
        };
        target.set(cpu, TargetSize::Byte(new_value));
        cpu.registers.f.zero = new_value == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = new_carry;
        8
    }

    /// Shift n right into Carry.
    #[inline]
    fn sra(cpu: &mut CPU, target: OperandTypes) -> u8 {
        let (new_carry, new_value) = match target.get(cpu) {
            TargetSize::Byte(target_value) => {
                let new_carry = target_value & 0b0000_0001 != 0;
                let new_value = target_value >> 1;
                (new_carry, new_value)
            }
            _ => panic!("SRA is only available for bytes targets"),
        };
        target.set(cpu, TargetSize::Byte(new_value));
        cpu.registers.f.zero = new_value == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = new_carry;
        8
    }

    #[inline]
    fn srl(cpu: &mut CPU, target: OperandTypes) -> u8 {
        let cycle = 8;

        let (new_carry, new_value) = match target.get(cpu) {
            TargetSize::Byte(target_value) => {
                let new_carry = target_value & 0b0000_0001 != 0;
                let new_value = target_value >> 1;
                (new_carry, new_value)
            }
            _ => panic!("SRL is only available for bytes targets"),
        };

        target.set(cpu, TargetSize::Byte(new_value));
        cpu.registers.f.zero = new_value == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = new_carry;
        cycle
    }

    #[inline]
    fn stop(cpu: &mut CPU) -> u8 {
        todo!("STOP not implemented")
    }

    #[inline]
    fn sub(cpu: &mut CPU, source: OperandTypes) -> u8 {
        let cycle = 8;

        let (zero, overflow) = match source.get(cpu) {
            TargetSize::Byte(source_value) => {
                let (new_value, overflow) = cpu.registers.a.overflowing_sub(source_value);
                cpu.registers.a = new_value;
                ((new_value == 0), overflow)
            }
            _ => panic!("SUB only available for bytes sources"),
        };

        cpu.registers.f.zero = zero;
        cpu.registers.f.subtract = true;
        cpu.registers.f.carry = overflow;
        cycle
    }

    #[inline]
    fn swap(cpu: &mut CPU, target: OperandTypes) -> u8 {
        let value = match target.get(cpu) {
            TargetSize::Byte(target_value) => target_value,
            _ => panic!("SWAP only available for bytes sources"),
        };
        let b = value >> 4;
        let c = value << 4;
        target.set(cpu, TargetSize::Byte(b ^ c));
        cpu.registers.f.zero = b ^ c == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = false;
        cpu.registers.f.half_carry = false;
        8
    }

    #[inline]
    fn xor(cpu: &mut CPU, source: OperandTypes) -> u8 {
        match source.get(cpu) {
            TargetSize::Byte(source_value) => {
                cpu.registers.a = cpu.registers.a ^ source_value;
            }
            _ => panic!("XOR only available for bytes sources"),
        };
        cpu.registers.f.zero = cpu.registers.a == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.carry = false;
        cpu.registers.f.half_carry = false;
        4
    }
}

#[derive(Debug, Clone, Copy)]
pub enum JoypadButton {
    A,
    B,
    Select,
    Start,
    Right,
    Left,
    Up,
    Down,
}

pub struct MemoryBus {
    pub memory: [u8; 0x10000],
    pub interupt_enable: InteruptsFlags,
    pub interupt_flags: InteruptsFlags,
    pub joypad_flags: JoypadFlags,
}

impl MemoryBus {
    fn new() -> Self {
        Self {
            memory: [0; 0x10000],
            interupt_enable: InteruptsFlags::new(),
            interupt_flags: InteruptsFlags::new(),
            joypad_flags: JoypadFlags::new(),
        }
    }

    #[inline]
    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.memory[address as usize],
            0x4000..=0x7FFF => match self.read_byte(0x0147) {
                0x00 => self.memory[address as usize],
                _ => todo!("Banked ROM not implemented"),
            },
            0x8000..=0x9FFF => self.memory[address as usize],
            0xA000..=0xBFFF => self.memory[address as usize],
            0xE000..=0xFDFF => self.read_byte(address - 0x2000),
            0xFE00..=0xFEFF => self.memory[address as usize],
            0xFF00 => self.read_joypad(),
            0xFF01 => todo!("SB: Serial transfer data"),
            0xFF02 => todo!("SC: Serial transfer control"),
            0xFF04 => todo!("DIV: Divider register"),
            0xFF05 => todo!("TIMA: Time counter"),
            0xFF06 => todo!("TMA: Timer Module"),
            0xFF07 => todo!("TAC: Timer control"),
            0xFF0F => self.interupt_flags.into(),
            0xFF80..=0xFFFE => self.memory[address as usize],
            0xFFFF => self.interupt_enable.into(),
            address => {
                eprintln!(
                    "Address {:#06x?} probably not implemented corrctly",
                    address
                );
                self.memory[address as usize]
            }
        }
    }

    #[inline]
    pub fn read_word(&self, address: u16) -> u16 {
        (self.read_byte(address + 1) as u16) << 8 | self.read_byte(address) as u16
    }

    #[inline]
    fn read_next_byte(&self, current: u16) -> u8 {
        self.read_byte(current + 1)
    }

    #[inline]
    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        match addr {
            0x0000..=0xDFFF => self.memory[addr as usize] = byte,
            0xE000..=0xFDFF => self.write_byte(addr - 0x2000, byte),
            0xFE00..=0xFEFF => self.memory[addr as usize] = byte,
            0xFF00 => self.write_joypad(byte),
            0xFF04 => todo!("DIV: Divider register"),
            0xFF05 => todo!("TIMA: Time counter"),
            0xFF06 => todo!("TMA: Timer Module"),
            0xFF07 => todo!("TAC: Timer control"),
            0xFF0F => self.interupt_flags = InteruptsFlags::from(byte),
            0xFF80..=0xFFFE => self.memory[addr as usize] = byte,
            0xFFFF => self.interupt_enable = InteruptsFlags::from(byte),
            addr => {
                eprintln!("Address {:#06x?} probably not implemented corrctly", addr);
                self.memory[addr as usize] = byte;
            }
        }
    }

    #[inline]
    pub fn write_word(&mut self, addr: u16, word: u16) {
        self.write_byte(addr, word as u8);
        self.write_byte(addr + 1, (word >> 8) as u8);
    }

    fn read_joypad(&self) -> u8 {
        if self.joypad_flags.buttons {
            (if self.joypad_flags.a { 0 } else { 1 } << 0)
                | (if self.joypad_flags.b { 0 } else { 1 } << 1)
                | (if self.joypad_flags.select { 0 } else { 1 } << 2)
                | (if self.joypad_flags.start { 0 } else { 1 } << 3)
        } else if self.joypad_flags.direction {
            (if self.joypad_flags.a { 0 } else { 1 } << 0)
                | (if self.joypad_flags.right { 0 } else { 1 } << 0)
                | (if self.joypad_flags.left { 0 } else { 1 } << 1)
                | (if self.joypad_flags.up { 0 } else { 1 } << 2)
                | (if self.joypad_flags.down { 0 } else { 1 } << 3)
        } else {
            0b0011_1111
        }
    }

    fn write_joypad(&mut self, value: u8) {
        // extract buttons flag
        if ((value >> 4) & 0b1) == 0 {
            self.joypad_flags.direction = true;
        } else if ((value >> 5) & 0b1) == 0 {
            self.joypad_flags.buttons = true;
        }
    }

    pub fn load_boot_rom(&mut self, boot_rom: &[u8]) {
        self.memory[..boot_rom.len()].copy_from_slice(boot_rom);
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.memory[0x0100..(rom.len() + 0x0100)].copy_from_slice(rom);
    }

    pub fn print_section(&self, start: u16, end: u16) {
        for i in (start..end).step_by(16) {
            for j in 0..16 {
                print!("{:02x} ", self.memory[(i + j) as usize]);
            }
            println!();
        }
    }
}

pub struct CPU {
    pub registers: Register,
    pub program_counter: u16,
    pub stack_pointer: u16,
    pub interupt_master_enable: bool,
    pub memory_bus: MemoryBus,
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

    fn ret(&mut self, should_return: bool) -> u16 {
        todo!("Ret not implemented")
    }

    // Stack
    fn push_word(&mut self, value: u16) {
        self.stack_pointer -= 2;
        self.memory_bus.write_word(self.stack_pointer, value);
    }

    fn pop_word(&mut self) -> u16 {
        let value = self.memory_bus.read_word(self.stack_pointer);
        self.stack_pointer += 2;
        value
    }
}

// Tests
