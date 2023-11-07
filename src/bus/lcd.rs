use super::{InteruptFlags, Memory};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum WindowTileMapArea {
    // 9800–9BFF
    #[default]
    Area0 = 0,
    // 9C00–9FFF
    Area1 = 1,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum BGWindowTileDataArea {
    // 8800–97FF
    #[default]
    Area0 = 0,
    // 8000–8FFF
    Area1 = 1,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum BGTileMapArea {
    // 9800–9BFF
    #[default]
    Area0 = 0,
    // 9C00–9FFF
    Area1 = 1,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum SpriteSize {
    // 8x8
    #[default]
    Size8x8 = 0,
    // 8x16
    Size8x16 = 1,
}

pub struct LCDControl {
    pub lcd_enable: bool,
    pub window_tile_map_area: WindowTileMapArea,
    pub window_enable: bool,
    pub bg_window_tile_data_area: BGWindowTileDataArea,
    pub bg_tile_map_area: BGTileMapArea,
    pub obj_size: SpriteSize,
    pub obj_enable: bool,
    pub bg_window_enable_priority: bool,
}

impl std::convert::From<LCDControl> for u8 {
    fn from(value: LCDControl) -> Self {
        let mut result = 0;
        if value.lcd_enable {
            result |= 0b1000_0000;
        }
        if value.window_tile_map_area == WindowTileMapArea::Area1 {
            result |= 0b0100_0000;
        }
        if value.window_enable {
            result |= 0b0010_0000;
        }
        if value.bg_window_tile_data_area == BGWindowTileDataArea::Area1 {
            result |= 0b0001_0000;
        }
        if value.bg_tile_map_area == BGTileMapArea::Area1 {
            result |= 0b0000_1000;
        }
        if value.obj_size == SpriteSize::Size8x16 {
            result |= 0b0000_0100;
        }
        if value.obj_enable {
            result |= 0b0000_0010;
        }
        if value.bg_window_enable_priority {
            result |= 0b0000_0001;
        }
        result
    }
}

impl std::convert::From<u8> for LCDControl {
    fn from(value: u8) -> Self {
        LCDControl {
            lcd_enable: value & 0b1000_0000 != 0,
            window_tile_map_area: if value & 0b0100_0000 != 0 {
                WindowTileMapArea::Area1
            } else {
                WindowTileMapArea::Area0
            },
            window_enable: value & 0b0010_0000 != 0,
            bg_window_tile_data_area: if value & 0b0001_0000 != 0 {
                BGWindowTileDataArea::Area1
            } else {
                BGWindowTileDataArea::Area0
            },
            bg_tile_map_area: if value & 0b0000_1000 != 0 {
                BGTileMapArea::Area1
            } else {
                BGTileMapArea::Area0
            },
            obj_size: if value & 0b0000_0100 != 0 {
                SpriteSize::Size8x16
            } else {
                SpriteSize::Size8x8
            },
            obj_enable: value & 0b0000_0010 != 0,
            bg_window_enable_priority: value & 0b0000_0001 != 0,
        }
    }
}
#[derive(Default, Debug, Clone, Copy)]
pub struct StatRegister {
    pub lyc_interupt: bool,
    pub mode_2_oam_interrupt: bool,
    pub mode_1_vblank_interrupt: bool,
    pub mode_0_hblank_interrupt: bool,
    pub lyc_ly: bool,
    pub ppu_mode: u8,
}

impl std::convert::From<StatRegister> for u8 {
    fn from(value: StatRegister) -> Self {
        let mut result = 0;
        if value.lyc_interupt {
            result |= 0b0100_0000;
        }
        if value.mode_2_oam_interrupt {
            result |= 0b0010_0000;
        }
        if value.mode_1_vblank_interrupt {
            result |= 0b0001_0000;
        }
        if value.mode_0_hblank_interrupt {
            result |= 0b0000_1000;
        }
        if value.lyc_ly {
            result |= 0b0000_0100;
        }
        result |= value.ppu_mode & 0b0000_0011;
        result
    }
}

impl std::convert::From<u8> for StatRegister {
    fn from(value: u8) -> Self {
        StatRegister {
            lyc_interupt: value & 0b0100_0000 != 0,
            mode_2_oam_interrupt: value & 0b0010_0000 != 0,
            mode_1_vblank_interrupt: value & 0b0001_0000 != 0,
            mode_0_hblank_interrupt: value & 0b0000_1000 != 0,
            lyc_ly: value & 0b0000_0100 != 0,
            ppu_mode: value & 0b0000_0011,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct LCDStatus {
    pub ly: u8,
    pub lyc: u8,
    pub stat: StatRegister,
}

impl Memory for LCDStatus {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF41 => self.stat.into(),
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            _ => panic!("Invalid read from LCDStatus address: {:04X}", address),
        }
    }
    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF41 => self.stat = value.into(),
            0xFF44 => self.ly = value,
            0xFF45 => self.lyc = value,
            _ => panic!("Invalid write to LCDStatus address: {:04X}", address),
        }
    }
}

impl LCDStatus {
    pub fn tick(&self, interupt_flags: &mut InteruptFlags) {
        if self.stat.lyc_interupt {
            if self.ly == self.lyc {
                interupt_flags.lcd_stat = true;
            }
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct LCDPosScroll {
    pub scy: u8,
    pub scx: u8,
    pub wy: u8,
    pub wx: u8,
}

pub enum Color {
    White = 0,
    LightGray = 1,
    DarkGray = 2,
    Black = 3,
}

pub struct ColorPalette {
    pub color3: Color,
    pub color2: Color,
    pub color1: Color,
    pub color0: Color,
}

pub struct LCDPalettes {
    pub bgp: ColorPalette,
    pub obp0: ColorPalette,
    pub obp1: ColorPalette,
    // TODO: CGB palettes
}

pub struct LCDRegisters {
    pub control: LCDControl,
    pub status: LCDStatus,
    pub pos_scroll: LCDPosScroll,
    pub palettes: LCDPalettes,
}

impl Memory for LCDRegisters {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF40 => self.control.into(),
            0xFF41 => self.status.into(),
            0xFF42 => self.pos_scroll.scy,
            0xFF43 => self.pos_scroll.scx,
            0xFF44 => self.status.ly,
            0xFF45 => self.status.lyc,
            0xFF47 => self.palettes.bgp.into(),
            0xFF48 => self.palettes.obp0.into(),
            0xFF49 => self.palettes.obp1.into(),
            0xFF4A => self.pos_scroll.wy,
            0xFF4B => self.pos_scroll.wx,
            _ => panic!("Invalid read from LCDRegisters address: {:04X}", address),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF40 => self.control = value.into(),
            0xFF41 => self.status = value.into(),
            0xFF42 => self.pos_scroll.scy = value,
            0xFF43 => self.pos_scroll.scx = value,
            0xFF44 => self.status.ly = value,
            0xFF45 => self.status.lyc = value,
            0xFF47 => self.palettes.bgp = value.into(),
            0xFF48 => self.palettes.obp0 = value.into(),
            0xFF49 => self.palettes.obp1 = value.into(),
            0xFF4A => self.pos_scroll.wy = value,
            0xFF4B => self.pos_scroll.wx = value,
            _ => panic!("Invalid write to LCDRegisters address: {:04X}", address),
        }
    }
}
