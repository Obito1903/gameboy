use self::{
    audio::{AudioRegisters, WavePattern},
    joypad::{JoypadButtons, JoypadRegister},
    lcd::LCDRegisters,
    serial::SerialRegister,
    time_divider::TimeDividerRegister,
};

pub mod audio;
pub mod joypad;
pub mod lcd;
pub mod serial;
pub mod time_divider;

pub struct InteruptFlags {
    v_blank: bool,
    lcd_stat: bool,
    timer: bool,
    serial: bool,
    joypad: bool,
}

pub struct IORegisters {
    joypad: JoypadRegister,
    serial: SerialRegister,
    timer_divider: TimeDividerRegister,
    audio: AudioRegisters,
    wave_pattern: WavePattern,
    lcd: LCDRegisters,
    // TODO: CGB registers
    vram_bank: u8,
    disable_boot_rom: u8,
    vram_dma: u8,
    bg_obj_palletes: u8,
    wram_bank: u8,
}

impl Memory for IORegisters {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF00 => self.joypad.into(),
            0xFF01..=0xFF02 => self.serial.read_byte(address),
            0xFF04..=0xFF07 => self.timer_divider.read_byte(address),
            0xFF10..=0xFF26 => self.audio.read_byte(address),
            0xFF30..=0xFF3F => self.wave_pattern.read_byte(address),
            // 0xFF40..=0xFF4B => self.lcd.read_byte(address),
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
            // 0xFF40..=0xFF4B => self.lcd.write_byte(address, value),
            0xFF4F => self.vram_bank = value,
            0xFF50 => self.disable_boot_rom = value,
            0xFF51..=0xFF55 => (),
            0xFF68..=0xFF6B => (),
            0xFF70 => self.wram_bank = value,
            _ => panic!("Invalid write to IORegisters address: {:04X}", address),
        }
    }
}

pub struct MemoryBus {
    pub memory_lock: MemoryLock,
    pub current_owner: MemoryLockOwner,

    pub rom: [u8; 0x3FFF],
    pub banked_rom: Vec<[u8; 0x3FFF]>,
    pub vram: ([u8; 0x1FFF], [u8; 0x1FFF]),
    pub external_ram: Vec<[u8; 0x1FFF]>,
    pub wram: [u8; 0x0FFF],
    pub external_wram: [[u8; 0x0FFF]; 7],
    pub oam: [u8; 0x009F],
    pub interupts_flags: InteruptFlags,
    pub io: IORegisters,
    pub hram: [u8; 0x007F],
    pub interupts_enable: InteruptFlags,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemoryLockOwner {
    CPU,
    PPU,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct MemoryLock {
    vram: Option<MemoryLockOwner>,
    oam: Option<MemoryLockOwner>,
}

impl MemoryBus {
    fn locked(&self, address: u16) -> bool {
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
}

impl Memory for MemoryBus {
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
            0xFF00..=0xFF7F => self.io.read_byte(address),
            0xFF80..=0xFFFE => self.hram[address as usize - 0xFF80],
            0xFFFF => 0, /* self.interupts_enable */
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
            0xFF00..=0xFF7F => self.io.write_byte(address, value),
            0xFF80..=0xFFFE => self.hram[address as usize - 0xFF80] = value,
            0xFFFF => (), /* self.interupts_enable */
        }
    }
}

pub trait Memory {
    fn read_byte(&self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, value: u8);
}
