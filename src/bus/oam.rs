use super::{Bus, Memory};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum DMGPalette {
    #[default]
    OBP0,
    OBP1,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum VRAMBank {
    #[default]
    Bank0,
    Bank1,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum CGBPalette {
    #[default]
    Palette0,
    Palette1,
    Palette2,
    Palette3,
    Palette4,
    Palette5,
    Palette6,
    Palette7,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct ObjectAttributeFlags {
    pub priority: bool,
    pub y_flip: bool,
    pub x_flip: bool,
    pub dmg_palette: DMGPalette,
    pub bank: VRAMBank,
    pub cgb_palette: CGBPalette,
}

impl std::convert::From<u8> for ObjectAttributeFlags {
    fn from(byte: u8) -> Self {
        Self {
            priority: byte & 0b1000_0000 != 0,
            y_flip: byte & 0b0100_0000 != 0,
            x_flip: byte & 0b0010_0000 != 0,
            dmg_palette: if byte & 0b0001_0000 != 0 {
                DMGPalette::OBP1
            } else {
                DMGPalette::OBP0
            },
            bank: if byte & 0b0000_1000 != 0 {
                VRAMBank::Bank1
            } else {
                VRAMBank::Bank0
            },
            cgb_palette: match byte & 0b0000_0111 {
                0 => CGBPalette::Palette0,
                1 => CGBPalette::Palette1,
                2 => CGBPalette::Palette2,
                3 => CGBPalette::Palette3,
                4 => CGBPalette::Palette4,
                5 => CGBPalette::Palette5,
                6 => CGBPalette::Palette6,
                7 => CGBPalette::Palette7,
                _ => unreachable!(),
            },
        }
    }
}

impl std::convert::From<ObjectAttributeFlags> for u8 {
    fn from(flags: ObjectAttributeFlags) -> Self {
        let mut value = 0;
        if flags.priority {
            value |= 0b1000_0000;
        }
        if flags.y_flip {
            value |= 0b0100_0000;
        }
        if flags.x_flip {
            value |= 0b0010_0000;
        }
        if flags.dmg_palette == DMGPalette::OBP1 {
            value |= 0b0001_0000;
        }
        if flags.bank == VRAMBank::Bank1 {
            value |= 0b0000_1000;
        }
        value |= match flags.cgb_palette {
            CGBPalette::Palette0 => 0,
            CGBPalette::Palette1 => 1,
            CGBPalette::Palette2 => 2,
            CGBPalette::Palette3 => 3,
            CGBPalette::Palette4 => 4,
            CGBPalette::Palette5 => 5,
            CGBPalette::Palette6 => 6,
            CGBPalette::Palette7 => 7,
        };
        value
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct ObjectAttribute {
    pub y: u8,
    pub x: u8,
    pub index: u8,
    pub flags: ObjectAttributeFlags,
}

impl Memory for ObjectAttribute {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0 => self.y,
            1 => self.x,
            2 => self.index,
            3 => self.flags.into(),
            _ => unreachable!(),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0 => self.y = value,
            1 => self.x = value,
            2 => self.index = value,
            3 => self.flags = value.into(),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Oam {
    transfer: bool,
    progress: u16,
    source_upper_byte: u8,
    pub data: [ObjectAttribute; 40],
}

impl Oam {
    pub fn dma_transfer_step(&mut self, memory: Bus) {
        if !self.transfer {
            return;
        }
        if self.progress == 160 {
            self.progress = 0;
            self.transfer = false;
            return;
        }
        println!("DMA transfer step: {:#04X}", self.progress);

        let source_address = (self.source_upper_byte as u16) << 8 | self.progress as u16;
        self.write_byte(0xFE00 + self.progress, memory.read_byte(source_address));
    }
}

impl Default for Oam {
    fn default() -> Self {
        Self {
            transfer: false,
            progress: 0,
            source_upper_byte: 0,
            data: [ObjectAttribute::default(); 40],
        }
    }
}

impl Memory for Oam {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF46 => self.source_upper_byte,
            _ => {
                let oa_index = ((address - 0xFE00) / 4) as usize;
                let oa_offset = (address - 0xFE00) % 4;
                self.data[oa_index].read_byte(oa_offset)
            }
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF46 => {
                self.transfer = true;
                self.source_upper_byte = value;
            }
            _ => {
                let oa_index = ((address - 0xFE00) / 4) as usize;
                let oa_offset = (address - 0xFE00) % 4;
                self.data[oa_index].write_byte(oa_offset, value);
            }
        }
    }
}
