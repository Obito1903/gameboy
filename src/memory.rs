use std::collections::HashSet;

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

#[derive(Debug, Clone, Copy)]
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
pub struct LCDCFlags {
    pub lcd_enable: bool,
    pub window_tile_map: bool,
    pub window_enable: bool,
    pub bg_window_tile_data: bool,
    pub bg_tile_map: bool,
    pub sprite_size: bool,
    pub sprite_enable: bool,
    pub bg_enable: bool,
}

impl LCDCFlags {
    fn new() -> Self {
        Self {
            lcd_enable: false,
            window_tile_map: false,
            window_enable: false,
            bg_window_tile_data: false,
            bg_tile_map: false,
            sprite_size: false,
            sprite_enable: false,
            bg_enable: false,
        }
    }
}

impl std::convert::From<LCDCFlags> for u8 {
    fn from(flag: LCDCFlags) -> u8 {
        (if flag.lcd_enable { 1 } else { 0 }) << 7
            | (if flag.window_tile_map { 1 } else { 0 }) << 6
            | (if flag.window_enable { 1 } else { 0 }) << 5
            | (if flag.bg_window_tile_data { 1 } else { 0 }) << 4
            | (if flag.bg_tile_map { 1 } else { 0 }) << 3
            | (if flag.sprite_size { 1 } else { 0 }) << 2
            | (if flag.sprite_enable { 1 } else { 0 }) << 1
            | (if flag.bg_enable { 1 } else { 0 }) << 0
    }
}

impl std::convert::From<u8> for LCDCFlags {
    fn from(byte: u8) -> Self {
        let lcd_enable = ((byte >> 7) & 0b1) != 0;
        let window_tile_map = ((byte >> 6) & 0b1) != 0;
        let window_enable = ((byte >> 5) & 0b1) != 0;
        let bg_window_tile_data = ((byte >> 4) & 0b1) != 0;
        let bg_tile_map = ((byte >> 3) & 0b1) != 0;
        let sprite_size = ((byte >> 2) & 0b1) != 0;
        let sprite_enable = ((byte >> 1) & 0b1) != 0;
        let bg_enable = ((byte >> 0) & 0b1) != 0;

        LCDCFlags {
            lcd_enable,
            window_tile_map,
            window_enable,
            bg_window_tile_data,
            bg_tile_map,
            sprite_size,
            sprite_enable,
            bg_enable,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct STATFlags {
    pub lyc_selected: bool,
    pub mode2_selected: bool,
    pub mode1_selected: bool,
    pub mode0_selected: bool,
    pub lyc_eq_ly: bool,
    pub ppu_mode: u8,
}

impl STATFlags {
    fn new() -> Self {
        Self {
            lyc_selected: false,
            mode2_selected: false,
            mode1_selected: false,
            mode0_selected: false,
            lyc_eq_ly: false,
            ppu_mode: 2,
        }
    }
}

impl std::convert::From<STATFlags> for u8 {
    fn from(flag: STATFlags) -> u8 {
        (if flag.lyc_selected { 1 } else { 0 }) << 6
            | (if flag.mode2_selected { 1 } else { 0 }) << 5
            | (if flag.mode1_selected { 1 } else { 0 }) << 4
            | (if flag.mode0_selected { 1 } else { 0 }) << 3
            | (if flag.lyc_eq_ly { 1 } else { 0 }) << 2
            | (if flag.ppu_mode == 0 { 1 } else { 0 }) << 1
    }
}

impl std::convert::From<u8> for STATFlags {
    fn from(byte: u8) -> Self {
        let lyc_selected = ((byte >> 6) & 0b1) != 0;
        let mode2_selected = ((byte >> 5) & 0b1) != 0;
        let mode1_selected = ((byte >> 4) & 0b1) != 0;
        let mode0_selected = ((byte >> 3) & 0b1) != 0;
        let lyc_eq_ly = ((byte >> 2) & 0b1) != 0;
        let ppu_mode = (byte >> 1) & 0b11;

        STATFlags {
            lyc_selected,
            mode2_selected,
            mode1_selected,
            mode0_selected,
            lyc_eq_ly,
            ppu_mode,
        }
    }
}

#[derive(PartialEq)]
pub enum MemoryBusClient {
    CPU,
    PPU,
}

pub struct MemoryBus {
    pub memory: [u8; 0x10000],
    pub interupt_enable: InteruptsFlags,
    pub interupt_flags: InteruptsFlags,
    pub joypad_flags: JoypadFlags,
    pub lcdc_flags: LCDCFlags,
    pub stat_flags: STATFlags,
    pub client: MemoryBusClient,
    pub lock_region: HashSet<(u16, u16)>,
}

impl MemoryBus {
    pub fn new() -> Self {
        Self {
            memory: [0; 0x10000],
            interupt_enable: InteruptsFlags::new(),
            interupt_flags: InteruptsFlags::new(),
            joypad_flags: JoypadFlags::new(),
            lcdc_flags: LCDCFlags::new(),
            stat_flags: STATFlags::new(),
            client: MemoryBusClient::CPU,
            lock_region: HashSet::new(),
        }
    }

    pub fn lock_region(&mut self, start: u16, end: u16) {
        println!("Locking region {:#06x?} - {:#06x?}", start, end);
        self.lock_region.insert((start, end));
    }

    pub fn unlock_region(&mut self, start: u16, end: u16) {
        println!("Unlocking region {:#06x?} - {:#06x?}", start, end);
        self.lock_region.remove(&(start, end));
    }

    #[inline]
    pub fn read_byte(&self, address: u16) -> u8 {
        // Return FF in locked regions
        if self.client == MemoryBusClient::CPU
            && self.lock_region.iter().any(|(start, end)| {
                if address >= *start && address <= *end {
                    return true;
                }
                false
            })
        {
            println!("Trying to read locked region {:#06x?}", address);
            return 0xFF;
        }
        match address {
            // ROM
            0x0000..=0x3FFF => self.memory[address as usize],
            // Banked ROM
            0x4000..=0x7FFF => match self.read_byte(0x0147) {
                0x00 => self.memory[address as usize],
                _ => todo!("Banked ROM not implemented"),
            },
            // VRAM
            0x8000..=0x9FFF => self.memory[address as usize],
            // External RAM
            0xA000..=0xBFFF => self.memory[address as usize],
            // Work RAM
            0xC000..=0xCFFF => self.memory[address as usize],
            // Banked Work RAM
            0xD000..=0xDFFF => self.memory[address as usize],
            // ECHO RAM
            0xE000..=0xFDFF => self.read_byte(address - 0x2000),
            // OAM
            0xFE00..=0xFE9F => self.memory[address as usize],
            // Unusable
            0xFEA0..=0xFEFF => self.memory[address as usize],
            // IO Registers
            0xFF00 => self.read_joypad(),
            0xFF01 => todo!("SB: Serial transfer data"),
            0xFF02 => todo!("SC: Serial transfer control"),
            0xFF04 => todo!("DIV: Divider register"),
            0xFF05 => todo!("TIMA: Time counter"),
            0xFF06 => todo!("TMA: Timer Module"),
            0xFF07 => todo!("TAC: Timer control"),
            0xFF0F => self.interupt_flags.into(),
            0xFF40 => self.lcdc_flags.into(),
            0xFF41 => self.stat_flags.into(),
            0xFF42..=0xFF44 => self.memory[address as usize],
            0xFF45 => self.memory[address as usize],
            // High RAM
            0xFF80..=0xFFFE => self.memory[address as usize],
            // Interupts enable
            0xFFFF => self.interupt_enable.into(),
            address => {
                eprintln!(
                    "Trying to read address, {:#06x?} probably not implemented corrctly",
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
        // Ignore write in locked regions
        if self.client == MemoryBusClient::CPU
            && self.lock_region.iter().any(|(start, end)| {
                if addr >= *start && addr <= *end {
                    return true;
                }
                false
            })
        {
            println!("Trying to write to locked region {:#06x?}", addr);
            return;
        }

        match addr {
            // ROM
            0x0000..=0x3FFF => panic!("Trying to write to ROM"),
            // Banked ROM
            0x4000..=0x7FFF => panic!("Trying to write to Banked ROM"),
            // VRAM
            0x8000..=0x9FFF => self.memory[addr as usize] = byte,
            // External RAM
            0xA000..=0xBFFF => self.memory[addr as usize] = byte,
            // Work RAM
            0xC000..=0xCFFF => self.memory[addr as usize] = byte,
            // Banked Work RAM
            0xD000..=0xDFFF => self.memory[addr as usize] = byte,
            // ECHO RAM
            0xE000..=0xFDFF => self.write_byte(addr - 0x2000, byte),
            // OAM
            0xFE00..=0xFE9F => self.memory[addr as usize] = byte,
            // Unusable
            0xFEA0..=0xFEFF => self.memory[addr as usize] = byte,
            // IO Registers
            0xFF00 => self.write_joypad(byte),
            0xFF04 => todo!("DIV: Divider register"),
            0xFF05 => todo!("TIMA: Time counter"),
            0xFF06 => todo!("TMA: Timer Module"),
            0xFF07 => todo!("TAC: Timer control"),
            0xFF0F => self.interupt_flags = InteruptsFlags::from(byte),
            0xFF40 => self.lcdc_flags = LCDCFlags::from(byte),
            0xFF41 => self.stat_flags = STATFlags::from(byte),
            0xFF42..=0xFF44 => self.memory[addr as usize] = byte,
            0xFF45 => self.memory[addr as usize] = byte,
            // High RAM
            0xFF80..=0xFFFE => self.memory[addr as usize] = byte,
            // Interupts enable
            0xFFFF => self.interupt_enable = InteruptsFlags::from(byte),
            addr => {
                eprintln!(
                    "trying to write address, {:#06x?} probably not implemented corrctly",
                    addr
                );
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
