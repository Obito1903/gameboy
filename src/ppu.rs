use crate::bus::{Bus, MemoryLockOwner, MemoryRegion};

pub enum PixelColor {
    White,
    LightGray,
    DarkGray,
    Black,
}

pub enum PixelPalette {
    Palette0,
    Palette1,
    Palette2,
}

pub struct Pixel {
    color: PixelColor,
    palette: PixelPalette,
    priority: u8,
    bg_priority: bool,
}

pub struct PPU {
    dot_counter: u16,
}

impl PPU {
    pub fn new() -> Self {
        Self { dot_counter: 0 }
    }

    #[inline]
    fn update_lyc(memory: &mut Bus) {
        // Update LYC == LY
        if memory.io.lcd.status.lyc == memory.io.lcd.status.ly {
            memory.io.lcd.status.stat.lyc_ly = true;
        } else {
            memory.io.lcd.status.stat.lyc_ly = false;
        }
    }

    fn update_stat_interupt(memory: &mut Bus) {
        if memory.io.lcd.status.stat.lyc_ly && memory.io.lcd.status.stat.lyc_interupt {
            memory.interupt_flags.lcd_stat = true;
        }
    }

    fn switch_to_mode0(memory: &mut Bus) {
        // Unlock VRAM
        memory.unlock(MemoryRegion::VRAM);
        // Unlock OAM
        memory.unlock(MemoryRegion::OAM);
        memory.io.lcd.status.stat.ppu_mode = 0;
    }

    pub fn mode0(&mut self, memory: &mut Bus) {
        memory.io.lcd.status.stat.ppu_mode = 0;
    }

    fn switch_to_mode1(memory: &mut Bus) {
        memory.io.lcd.status.stat.ppu_mode = 1;
    }

    pub fn mode1(&mut self, memory: &mut Bus) {
        memory.interupt_flags.v_blank = true;
    }

    fn switch_to_mode2(memory: &mut Bus) {
        // Lock OAM
        memory.lock(MemoryRegion::OAM);
        memory.io.lcd.status.stat.ppu_mode = 2;
    }

    pub fn mode2(&mut self, memory: &mut Bus) {
        if self.dot_counter == 80 {
            Self::switch_to_mode3(memory);
        }
    }

    fn switch_to_mode3(memory: &mut Bus) {
        // Lock VRAM
        memory.lock(MemoryRegion::VRAM);
        memory.io.lcd.status.stat.ppu_mode = 3;
    }

    pub fn mode3(&mut self, memory: &mut Bus) {
        if self.dot_counter == 252 {
            Self::switch_to_mode0(memory);
        }
    }

    pub fn step(&mut self, memory: &mut Bus) {
        Self::update_lyc(memory);
        Self::update_stat_interupt(memory);

        if memory.io.lcd.status.ly == 144 {
            Self::switch_to_mode1(memory);
        }
        if memory.io.lcd.status.ly == 153 {
            memory.io.lcd.status.ly = 0;
            Self::switch_to_mode2(memory);
        }

        match memory.io.lcd.status.stat.ppu_mode {
            0 => self.mode0(memory),
            1 => self.mode1(memory),
            2 => self.mode2(memory),
            3 => self.mode3(memory),
            _ => panic!("Invalid PPU mode"),
        }

        // Advance dot counter
        self.dot_counter += 1;
        if self.dot_counter == 456 {
            self.dot_counter = 0;
            memory.io.lcd.status.ly += 1;
        }
    }

    pub fn run_for(&mut self, memory: &mut Bus, cycles: u8) {
        memory.current_owner = MemoryLockOwner::PPU;
        for _ in 0..cycles {
            self.step(memory);
        }
    }
}
