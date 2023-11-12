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

#[derive(Debug, Clone, Copy)]
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

impl Default for LCDControl {
    fn default() -> Self {
        Self {
            lcd_enable: true,
            window_tile_map_area: WindowTileMapArea::Area0,
            window_enable: false,
            bg_window_tile_data_area: BGWindowTileDataArea::Area0,
            bg_tile_map_area: BGTileMapArea::Area0,
            obj_size: SpriteSize::Size8x8,
            obj_enable: false,
            bg_window_enable_priority: false,
        }
    }
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

impl Memory for LCDPosScroll {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            _ => panic!("Invalid read from LCDPosScroll address: {:04X}", address),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF42 => self.scy = value,
            0xFF43 => self.scx = value,
            0xFF4A => self.wy = value,
            0xFF4B => self.wx = value,
            _ => panic!("Invalid write to LCDPosScroll address: {:04X}", address),
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Color {
    #[default]
    White = 0,
    LightGray = 1,
    DarkGray = 2,
    Black = 3,
}

impl std::convert::From<u8> for Color {
    fn from(value: u8) -> Self {
        match value {
            0b00 => Color::White,
            0b01 => Color::LightGray,
            0b10 => Color::DarkGray,
            0b11 => Color::Black,
            _ => panic!("Invalid color value: {:02X}", value),
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct ColorPalette {
    pub color3: Color,
    pub color2: Color,
    pub color1: Color,
    pub color0: Color,
}

impl std::convert::From<ColorPalette> for u8 {
    fn from(value: ColorPalette) -> Self {
        let mut result = 0;
        result |= value.color3 as u8;
        result |= (value.color2 as u8) << 2;
        result |= (value.color1 as u8) << 4;
        result |= (value.color0 as u8) << 6;
        result
    }
}

impl std::convert::From<u8> for ColorPalette {
    fn from(value: u8) -> Self {
        ColorPalette {
            color3: (value & 0b0000_0011).into(),
            color2: ((value & 0b0000_1100) >> 2).into(),
            color1: ((value & 0b0011_0000) >> 4).into(),
            color0: ((value & 0b1100_0000) >> 6).into(),
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct LCDPalettes {
    pub bgp: ColorPalette,
    pub obp0: ColorPalette,
    pub obp1: ColorPalette,
    // TODO: CGB palettes
}

impl Memory for LCDPalettes {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF47 => self.bgp.into(),
            0xFF48 => self.obp0.into(),
            0xFF49 => self.obp1.into(),
            _ => panic!("Invalid read from LCDPalettes address: {:04X}", address),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF47 => self.bgp = value.into(),
            0xFF48 => self.obp0 = value.into(),
            0xFF49 => self.obp1 = value.into(),
            _ => panic!("Invalid write to LCDPalettes address: {:04X}", address),
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
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
            0xFF41 => self.status.read_byte(address),
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
            0xFF41 => self.status.write_byte(address, value),
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
