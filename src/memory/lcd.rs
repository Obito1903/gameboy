use super::InteruptFlags;

pub enum WindowTileMapArea {
    // 9800–9BFF
    Area0 = 0,
    // 9C00–9FFF
    Area1 = 1,
}

pub enum BGWindowTileDataArea {
    // 8800–97FF
    Area0 = 0,
    // 8000–8FFF
    Area1 = 1,
}

pub enum BGTileMapArea {
    // 9800–9BFF
    Area0 = 0,
    // 9C00–9FFF
    Area1 = 1,
}

pub enum SpriteSize {
    // 8x8
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

pub struct StatRegister {
    pub lyc_interupt: bool,
    pub mode_2_oam_interrupt: bool,
    pub mode_1_vblank_interrupt: bool,
    pub mode_0_hblank_interrupt: bool,
    pub lyc_ly: bool,
    pub ppu_mode: u8,
}

pub struct LCDStatus {
    pub ly: u8,
    pub lyc: u8,
    pub stat: StatRegister,
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
