use self::{
    audio::{AudioRegisters, WavePattern},
    joypad::JoypadRegister,
    lcd::LCDRegisters,
    serial::SerialRegister,
    time_divider::TimeDividerRegister,
};

pub mod audio;
pub mod joypad;
pub mod lcd;
pub mod oam;
pub mod serial;
pub mod time_divider;

#[derive(Default, Debug, Clone, Copy)]
pub struct InteruptFlags {
    pub joypad: bool,
    pub serial: bool,
    pub timer: bool,
    pub lcd_stat: bool,
    pub v_blank: bool,
}

impl std::convert::From<InteruptFlags> for u8 {
    fn from(flags: InteruptFlags) -> u8 {
        let mut value = 0;
        if flags.joypad {
            value |= 0b0000_0001;
        }
        if flags.serial {
            value |= 0b0000_0010;
        }
        if flags.timer {
            value |= 0b0000_0100;
        }
        if flags.lcd_stat {
            value |= 0b0000_1000;
        }
        if flags.v_blank {
            value |= 0b0001_0000;
        }
        value
    }
}

impl std::convert::From<u8> for InteruptFlags {
    fn from(value: u8) -> Self {
        InteruptFlags {
            joypad: value & 0b0000_0001 != 0,
            serial: value & 0b0000_0010 != 0,
            timer: value & 0b0000_0100 != 0,
            lcd_stat: value & 0b0000_1000 != 0,
            v_blank: value & 0b0001_0000 != 0,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct IORegisters {
    pub joypad: JoypadRegister,
    pub serial: SerialRegister,
    pub timer_divider: TimeDividerRegister,
    pub audio: AudioRegisters,
    pub wave_pattern: WavePattern,
    pub lcd: LCDRegisters,
    // TODO: CGB registers
    pub vram_bank: u8,
    pub disable_boot_rom: u8,
    pub _vram_dma: u8,
    pub _bg_obj_palletes: u8,
    pub wram_bank: u8,
}

impl Memory for IORegisters {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF00 => self.joypad.into(),
            0xFF01..=0xFF02 => self.serial.read_byte(address),
            0xFF04..=0xFF07 => self.timer_divider.read_byte(address),
            0xFF10..=0xFF26 => self.audio.read_byte(address),
            0xFF30..=0xFF3F => self.wave_pattern.read_byte(address),
            0xFF40..=0xFF4B => self.lcd.read_byte(address),
            0xFF4F => self.vram_bank,
            0xFF50 => self.disable_boot_rom,
            0xFF51..=0xFF55 => 0,
            0xFF68..=0xFF6B => 0,
            0xFF70 => self.wram_bank,
            _ => panic!("Invalid read from IORegisters address: {:04X}", address),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF00 => self.joypad.write_byte(value),
            0xFF01..=0xFF02 => self.serial.write_byte(address, value),
            0xFF04..=0xFF07 => self.timer_divider.write_byte(address, value),
            0xFF10..=0xFF26 => self.audio.write_byte(address, value),
            0xFF30..=0xFF3F => self.wave_pattern.write_byte(address, value),
            0xFF40..=0xFF4B => self.lcd.write_byte(address, value),
            0xFF4F => self.vram_bank = value,
            0xFF50 => self.disable_boot_rom = value,
            0xFF51..=0xFF55 => (),
            0xFF68..=0xFF6B => (),
            0xFF70 => self.wram_bank = value,
            _ => panic!("Invalid write to IORegisters address: {:04X}", address),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Bus {
    pub memory_lock: MemoryLock,
    pub current_owner: MemoryLockOwner,

    pub rom: [u8; 0x3FFF],
    pub banked_rom: Vec<[u8; 0x3FFF]>,
    pub vram: ([u8; 0x1FFF], [u8; 0x1FFF]),
    pub external_ram: Vec<[u8; 0x1FFF]>,
    pub wram: [u8; 0x0FFF],
    pub external_wram: [[u8; 0x0FFF]; 7],
    pub oam: [u8; 0x009F],
    pub interupt_flags: InteruptFlags,
    pub io: IORegisters,
    pub hram: [u8; 0x007F],
    pub interupt_enable: InteruptFlags,
}

impl Default for Bus {
    fn default() -> Self {
        Self {
            memory_lock: MemoryLock::default(),
            current_owner: MemoryLockOwner::CPU,

            rom: [0; 0x3FFF],
            banked_rom: vec![[0; 0x3FFF]],
            vram: ([0; 0x1FFF], [0; 0x1FFF]),
            external_ram: vec![[0; 0x1FFF]],
            wram: [0; 0x0FFF],
            external_wram: [[0; 0x0FFF]; 7],
            oam: [0; 0x009F],
            interupt_flags: InteruptFlags::default(),
            io: IORegisters::default(),
            hram: [0; 0x007F],
            interupt_enable: InteruptFlags::default(),
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum MemoryLockOwner {
    #[default]
    CPU,
    PPU,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct MemoryLock {
    vram: Option<MemoryLockOwner>,
    oam: Option<MemoryLockOwner>,
}

#[derive(Default, Debug, Clone, Copy)]
pub enum MemoryRegion {
    #[default]
    VRAM,
    OAM,
    // TODO: CGB WRAM,
    // CGBPalette,
}

impl Bus {
    pub fn lock(&mut self, region: MemoryRegion) {
        match region {
            MemoryRegion::VRAM => {
                self.memory_lock.vram = Some(self.current_owner);
            }
            MemoryRegion::OAM => {
                self.memory_lock.oam = Some(self.current_owner);
            }
        }
    }

    pub fn unlock(&mut self, region: MemoryRegion) {
        match region {
            MemoryRegion::VRAM => {
                self.memory_lock.vram = None;
            }
            MemoryRegion::OAM => {
                self.memory_lock.oam = None;
            }
        }
    }

    pub fn locked(&self, address: u16) -> bool {
        match address {
            0x8000..=0x9FFF => {
                self.memory_lock.vram.is_some()
                    && self.memory_lock.vram != Some(MemoryLockOwner::PPU)
            }
            0xFE00..=0xFE9F => {
                self.memory_lock.oam.is_some() && self.memory_lock.oam != Some(MemoryLockOwner::PPU)
            }
            _ => false,
        }
    }

    pub fn load_boot_rom(&mut self, boot_rom: &[u8]) {
        self.rom[..boot_rom.len()].copy_from_slice(boot_rom);
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.rom[0x0100..(rom.len() + 0x0100)].copy_from_slice(rom);
    }
}

impl Memory for Bus {
    fn read_byte(&self, address: u16) -> u8 {
        if self.locked(address) {
            return 0xFF;
        }
        match address {
            0x0000..=0x3FFF => self.rom[address as usize],
            0x4000..=0x7FFF => self.banked_rom[0][address as usize - 0x4000],
            0x8000..=0x9FFF => self.vram.0[address as usize - 0x8000],
            0xA000..=0xBFFF => self.external_ram[0][address as usize - 0xA000],
            0xC000..=0xCFFF => self.wram[address as usize - 0xC000],
            0xD000..=0xDFFF => self.external_wram[0][address as usize - 0xD000],
            0xE000..=0xFDFF => self.wram[address as usize - 0xE000],
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00],
            0xFEA0..=0xFEFF => 0,
            0xFF00..=0xFF0E => self.io.read_byte(address),
            0xFF0F => self.interupt_flags.into(),
            0xFF10..=0xFF7F => self.io.read_byte(address),
            0xFF80..=0xFFFE => self.hram[address as usize - 0xFF80],
            0xFFFF => self.interupt_enable.into(),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        if self.locked(address) {
            return;
        }

        match address {
            0x0000..=0x7FFF => panic!("Cannot write to ROM"),
            0x8000..=0x9FFF => self.vram.0[address as usize - 0x8000] = value,
            0xA000..=0xBFFF => self.external_ram[0][address as usize - 0xA000] = value,
            0xC000..=0xCFFF => self.wram[address as usize - 0xC000] = value,
            0xD000..=0xDFFF => self.external_wram[0][address as usize - 0xD000] = value,
            0xE000..=0xFDFF => self.wram[address as usize - 0xE000] = value,
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00] = value,
            0xFEA0..=0xFEFF => (),
            0xFF00..=0xFF0E => self.io.write_byte(address, value),
            0xFF0F => self.interupt_flags = value.into(),
            0xFF10..=0xFF7F => self.io.write_byte(address, value),
            0xFF80..=0xFFFE => self.hram[address as usize - 0xFF80] = value,
            0xFFFF => self.interupt_enable = value.into(),
        }
    }
}

pub trait Memory {
    fn read_byte(&self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, value: u8);

    fn read_word(&self, address: u16) -> u16 {
        let low = self.read_byte(address);
        let high = self.read_byte(address + 1);
        u16::from_le_bytes([low, high])
    }

    fn write_word(&mut self, address: u16, value: u16) {
        let [low, high] = value.to_le_bytes();
        self.write_byte(address, low);
        self.write_byte(address + 1, high);
    }
}
