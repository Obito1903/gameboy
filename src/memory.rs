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

pub struct MemoryBus {
    pub memory: [u8; 0x10000],
    pub interupt_enable: InteruptsFlags,
    pub interupt_flags: InteruptsFlags,
    pub joypad_flags: JoypadFlags,
}

impl MemoryBus {
    pub fn new() -> Self {
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
    pub fn read_next_byte(&self, current: u16) -> u8 {
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
