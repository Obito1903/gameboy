pub enum DMGPalette {
    OBP0,
    OBP1,
}

pub enum VRAMBank {
    Bank0,
    Bank1,
}

pub enum CGBPalette {
    Palette0,
    Palette1,
    Palette2,
    Palette3,
    Palette4,
    Palette5,
    Palette6,
    Palette7,
}

pub struct ObjectAttributeFlags {
    priority: bool,
    y_flip: bool,
    x_flip: bool,
    dmg_palette: DMGPalette,
    bank: VRAMBank,
    cgb_palette: CGBPalette,
}

pub struct ObjectAttribute {
    y: u8,
    x: u8,
    index: u8,
    flags: ObjectAttributeFlags,
}

pub struct Oam {
    data: [u8; 0xA0],
}
