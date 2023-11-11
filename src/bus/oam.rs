use super::Memory;

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

#[derive(Debug, Clone)]
pub struct Oam {
    transfer: bool,
    progress: u8,
    source_upper_byte: u8,
    data: [u8; 0xA0],
}

impl Oam {
    pub fn dma_transfer_step(&mut self) {
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

        self.data[0xFE00 + self.progress as usize] = self.read_byte(source_address);
    }
}

impl Default for Oam {
    fn default() -> Self {
        Self {
            transfer: false,
            progress: 0,
            source_upper_byte: 0,
            data: [0; 0xA0],
        }
    }
}

impl Memory for Oam {
    fn read_byte(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF46 => {
                self.transfer = true;
                self.source_upper_byte = value;
            }
            _ => self.data[address as usize] = value,
        }
    }
}
