use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
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
struct Register {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegister,
    h: u8,
    l: u8,
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

    fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | (<FlagsRegister as Into<u8>>::into(self.f) as u16)
    }

    fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value as u8).into();
    }

    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | (self.c as u16)
    }

    fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | (self.e as u16)
    }

    fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | (self.l as u16)
    }

    fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }
}

#[derive(Debug, Clone, Copy)]
enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    BC,
    DE,
    SP,
    d8,
    r8,
}

impl Display for ArithmeticTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArithmeticTarget::A => write!(f, "A"),
            ArithmeticTarget::B => write!(f, "B"),
            ArithmeticTarget::C => write!(f, "C"),
            ArithmeticTarget::D => write!(f, "D"),
            ArithmeticTarget::E => write!(f, "E"),
            ArithmeticTarget::H => write!(f, "H"),
            ArithmeticTarget::L => write!(f, "L"),
            ArithmeticTarget::HL => write!(f, "HL"),
            ArithmeticTarget::BC => write!(f, "BC"),
            ArithmeticTarget::DE => write!(f, "DE"),
            ArithmeticTarget::SP => write!(f, "SP"),
            ArithmeticTarget::d8 => write!(f, "d8"),
            ArithmeticTarget::r8 => write!(f, "r8"),
            _ => todo!()
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    ADD(ArithmeticTarget),
    ADDHL(ArithmeticTarget),
    ADC(ArithmeticTarget),
}

impl Instruction {
    fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Self::from_byte_prefixed(byte)
        } else {
            Self::from_byte_unprefixed(byte)
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),
            0x86 => Some(Instruction::ADD(ArithmeticTarget::HL)),
            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),

            0x88 => Some(Instruction::ADC(ArithmeticTarget::B)),
            0x89 => Some(Instruction::ADC(ArithmeticTarget::C)),
            0x8A => Some(Instruction::ADC(ArithmeticTarget::D)),
            0x8B => Some(Instruction::ADC(ArithmeticTarget::E)),
            0x8C => Some(Instruction::ADC(ArithmeticTarget::H)),
            0x8D => Some(Instruction::ADC(ArithmeticTarget::L)),
            0x8E => Some(Instruction::ADC(ArithmeticTarget::HL)),
            0x8F => Some(Instruction::ADC(ArithmeticTarget::A)),

            0x09 => Some(Instruction::ADDHL(ArithmeticTarget::BC)),
            0x19 => Some(Instruction::ADDHL(ArithmeticTarget::DE)),
            0x29 => Some(Instruction::ADDHL(ArithmeticTarget::HL)),
            0x39 => Some(Instruction::ADDHL(ArithmeticTarget::SP)),

            0xC6 => Some(Instruction::ADD(ArithmeticTarget::d8)),
            0xCE => Some(Instruction::ADC(ArithmeticTarget::d8)),
            // 0xE8 => Some(Instruction::ADDSP(ArithmeticTarget::r8)),

            _ => None,
        }
    }

    fn from_byte_unprefixed(byte: u8) -> Option<Instruction> {
        todo!("Instruction::from_byte_unprefixed not implemented")
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::ADD(target) => write!(f, "ADD {}", target),
            Instruction::ADC(target) => write!(f, "ADC {}", target),
            Instruction::ADDHL(target) => write!(f, "ADDHL {}", target),
            _ => write!(f, "Display not implemented for {:?}", self),
        }
    }
}

struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl MemoryBus {
    fn new() -> Self {
        Self {
            memory: [0; 0xFFFF],
        }
    }

    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn read_next_byte(&self) -> u8 {
        0
    }
    fn write_byte(&self, addr: u16, byte: u8) {}
}

enum LoadByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}
enum LoadByteSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    D8,
    HLI,
}
enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
}

struct CPU {
    registers: Register,
    program_counter: u16,
    stack_pointer: u16,
    memory_bus: MemoryBus,
}

impl CPU {
    fn new() -> Self {
        Self {
            registers: Register::new(),
            program_counter: 0,
            stack_pointer: 0,
            memory_bus: MemoryBus::new(),
        }
    }

    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::ADD(target) => add(self, target),
            Instruction::ADC(target) => adc(self, target),
            Instruction::ADDHL(target) => addhl(self, target),
            _ => {
                panic!("Instruction {:?} not implemented", instruction)
            }
        }
    }

    /// Read instruction from memory and execute it
    /// The full set of steps is as follows:
    /// - Use the program counter to read the instruction byte from memory.
    /// - Translate the byte to one of the instances of the Instruction enum
    /// - If we can successfully translate the instruction call our execute method else panic which now returns the next program counter
    /// - Set this next program counter on our CPU
    fn step(&mut self) {
        let mut opcode = self.memory_bus.read_byte(self.program_counter);
        let prefixed = opcode == 0xCB;
        if prefixed {
            opcode = self.memory_bus.read_next_byte();
        }
        let next_pc = if let Some(instruction) = Instruction::from_byte(opcode, prefixed) {
            self.execute(instruction)
        } else {
            panic!("Instruction {:?} not implemented", opcode)
        };

        self.program_counter = next_pc;
    }

    fn jump(&mut self, should_jump: bool) -> u16 {
        todo!("Jump not implemented")
    }

    fn call(&mut self, should_call: bool) -> u16 {
        todo!("Call not implemented")
    }

    fn ret(&mut self, should_return: bool) -> u16 {
        todo!("Ret not implemented")
    }

    // Stack
    fn push(&mut self, value: u16) {
        todo!("Push not implemented")
    }

    fn pop(&mut self) -> u16 {
        todo!("Pop not implemented")
    }
}

#[inline]
fn add_impl(cpu: &mut CPU, value: u8) -> u8 {
    let (new_value, overflow) = cpu.registers.a.overflowing_add(value);
    cpu.registers.f.zero = new_value == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.carry = overflow;
    // Half Carry is set if adding the lower nibbles of the value and register A
    // together result in a value bigger than 0xF. If the result is larger than 0xF
    // than the addition caused a carry from the lower nibble to the upper nibble.
    cpu.registers.f.half_carry = (cpu.registers.a & 0xF) + (value & 0xF) > 0xF;
    new_value
}

#[inline]
fn add(cpu: &mut CPU, target: ArithmeticTarget) -> u16 {

    match target {
        ArithmeticTarget::A => {
            cpu.registers.a = add_impl(cpu, cpu.registers.a);
        }
        ArithmeticTarget::B => {
            cpu.registers.a = add_impl(cpu, cpu.registers.b);
        }
        ArithmeticTarget::C => {
            cpu.registers.a = add_impl(cpu, cpu.registers.c);
        }
        ArithmeticTarget::D => {
            cpu.registers.a = add_impl(cpu,cpu.registers.d);
        }
        ArithmeticTarget::E => {
            cpu.registers.a = add_impl(cpu,cpu.registers.e);
        }
        ArithmeticTarget::H => {
            cpu.registers.a = add_impl(cpu,cpu.registers.h);
        }
        ArithmeticTarget::L => {
            cpu.registers.a = add_impl(cpu,cpu.registers.l);
        }
        ArithmeticTarget::HL => {
            cpu.registers.a = add_impl(cpu,cpu.memory_bus.read_byte(cpu.registers.get_hl()));
        }
        ArithmeticTarget::d8 => {
            cpu.registers.a = add_impl(cpu,cpu.memory_bus.read_byte(cpu.program_counter));

        }
        _ => {panic!("ADD target {} not implemented", target)}
    }

    cpu.program_counter.wrapping_add(1)
}

#[inline]
fn adc_impl(cpu: &mut CPU, value: u8) -> u8 {
    let (mut new_value, overflow1) = cpu.registers.a.overflowing_add(value);
    let mut overflow2 = false;
    if cpu.registers.f.carry {
        (new_value, overflow2) = new_value.overflowing_add(1);
    }
    cpu.registers.f.zero = new_value == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.carry = overflow1 || overflow2;
    // Half Carry is set if adding the lower nibbles of the value and register A
    // together result in a value bigger than 0xF. If the result is larger than 0xF
    // than the addition caused a carry from the lower nibble to the upper nibble.
    cpu.registers.f.half_carry = (cpu.registers.a & 0xF) + (value & 0xF) > 0xF;
    new_value
}

#[inline]
fn adc(cpu: &mut CPU, target: ArithmeticTarget) -> u16 {
    
    match target {
        ArithmeticTarget::A => {
            cpu.registers.a = adc_impl(cpu, cpu.registers.a);
        }
        ArithmeticTarget::B => {
            cpu.registers.a = adc_impl(cpu, cpu.registers.b);
        }
        ArithmeticTarget::C => {
            cpu.registers.a = adc_impl(cpu, cpu.registers.c);
        }
        ArithmeticTarget::D => {
            cpu.registers.a = adc_impl(cpu,cpu.registers.d);
        }
        ArithmeticTarget::E => {
            cpu.registers.a = adc_impl(cpu,cpu.registers.e);
        }
        ArithmeticTarget::H => {
            cpu.registers.a = adc_impl(cpu,cpu.registers.h);
        }
        ArithmeticTarget::L => {
            cpu.registers.a = adc_impl(cpu,cpu.registers.l);
        }
        ArithmeticTarget::HL => {
            cpu.registers.a = adc_impl(cpu,cpu.memory_bus.read_byte(cpu.registers.get_hl()));
        }
        ArithmeticTarget::d8 => {
            cpu.registers.a = adc_impl(cpu,cpu.memory_bus.read_byte(cpu.program_counter));

        }
        _ => {panic!("ADC target {} not implemented", target)}
    }

    cpu.program_counter.wrapping_add(1)
}

#[inline]
fn addhl_impl(cpu: &mut CPU, value: u16) -> u16 {
    let (new_value, overflow) = cpu.registers.get_hl().overflowing_add(value);
    cpu.registers.f.zero = new_value == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.carry = overflow;
    // Half Carry is set if adding the lower nibbles of the value and register A
    // together result in a value bigger than 0xF. If the result is larger than 0xF
    // than the addition caused a carry from the lower nibble to the upper nibble.
    cpu.registers.f.half_carry = (cpu.registers.get_hl() & 0xF) + (value & 0xF) > 0xF;
    new_value
}

#[inline]
fn addhl(cpu: &mut CPU, target: ArithmeticTarget) -> u16 {
    
    match target {
        ArithmeticTarget::BC => {
            let value = addhl_impl(cpu, cpu.registers.get_bc());
            cpu.registers.set_hl(value);
        }
        ArithmeticTarget::DE => {
            let value = addhl_impl(cpu, cpu.registers.get_de());
            cpu.registers.set_hl(value);
        }
        ArithmeticTarget::HL => {
            let value = addhl_impl(cpu, cpu.registers.get_hl());
            cpu.registers.set_hl(value);
        }
        ArithmeticTarget::SP => {
            let value = addhl_impl(cpu, cpu.stack_pointer);
            cpu.registers.set_hl(value);
        }
        _ => {panic!("ADDHP target {} not implemented", target)}
    }

    cpu.program_counter.wrapping_add(1)
}


// Tests
#[cfg(test)]
mod tests {
    use super::CPU;

    #[test]
    fn add_c() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x01;
        cpu.registers.c = 0x02;
        cpu.program_counter = 0x0000;
        cpu.execute(super::Instruction::ADD(super::ArithmeticTarget::C));
        assert_eq!(cpu.registers.a, 0x03);
    }

    #[test]
    fn addhl_bc() {
        let mut cpu = CPU::new();
        cpu.registers.set_hl(0x01);
        cpu.registers.set_bc(0x02);
        cpu.program_counter = 0x0000;
        cpu.execute(super::Instruction::ADDHL(super::ArithmeticTarget::BC));
        assert_eq!(cpu.registers.get_hl(), 0x03);
    }

    #[test]
    fn adc_c() {
        let mut cpu = CPU::new();
        cpu.registers.c = 0x01;
        cpu.registers.a = 0x02;
        cpu.program_counter = 0x0000;
        cpu.execute(super::Instruction::ADC(super::ArithmeticTarget::C));
        assert_eq!(cpu.registers.a, 0x03);
    }
}
