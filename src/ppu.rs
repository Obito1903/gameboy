use crate::memory::{self, MemoryBus, MemoryBusClient};

pub struct PPU {
    dot_counter: u16,
}

impl PPU {
    pub fn new() -> Self {
        Self { dot_counter: 0 }
    }

    #[inline]
    fn update_lyc(memory: &mut MemoryBus) {
        // Update LYC == LY
        if memory.read_byte(0xFF44) == memory.read_byte(0xFF45) {
            memory.stat_flags.lyc_eq_ly = true;
        } else {
            memory.stat_flags.lyc_eq_ly = false;
        }
    }

    fn update_stat_interupt(memory: &mut MemoryBus) {
        if memory.stat_flags.lyc_eq_ly && memory.stat_flags.lyc_selected {
            memory.interupt_flags.lcd_stat = true;
        }
    }

    fn switch_to_mode0(memory: &mut MemoryBus) {
        // Unlock VRAM
        memory.unlock_region(0x8000, 0x9FFF);
        // Unlock OAM
        memory.unlock_region(0xFE00, 0xFE9F);
        memory.stat_flags.ppu_mode = 0;
    }

    pub fn mode0(&mut self, memory: &mut MemoryBus) {
        memory.stat_flags.ppu_mode = 0;
    }

    fn switch_to_mode1(memory: &mut MemoryBus) {
        memory.stat_flags.ppu_mode = 1;
    }

    pub fn mode1(&mut self, memory: &mut MemoryBus) {
        memory.interupt_flags.v_blank = true;
    }

    fn switch_to_mode2(memory: &mut MemoryBus) {
        // Lock OAM
        memory.lock_region(0xFE00, 0xFE9F);
        memory.stat_flags.ppu_mode = 2;
    }

    pub fn mode2(&mut self, memory: &mut MemoryBus) {
        if self.dot_counter == 80 {
            Self::switch_to_mode3(memory);
        }
    }

    fn switch_to_mode3(memory: &mut MemoryBus) {
        // Lock VRAM
        memory.lock_region(0x8000, 0x9FFF);
        memory.stat_flags.ppu_mode = 3;
    }

    pub fn mode3(&mut self, memory: &mut MemoryBus) {
        if self.dot_counter == 252 {
            Self::switch_to_mode0(memory);
        }
    }

    pub fn step(&mut self, memory: &mut MemoryBus) {
        Self::update_lyc(memory);
        Self::update_stat_interupt(memory);

        let ly = memory.read_byte(0xFF44);
        if ly == 144 {
            Self::switch_to_mode1(memory);
        }
        if ly == 153 {
            memory.write_byte(0xFF44, 0);
            Self::switch_to_mode2(memory);
        }

        match memory.stat_flags.ppu_mode {
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
            memory.write_byte(0xFF44, ly + 1);
        }
    }

    pub fn run_for(&mut self, memory: &mut MemoryBus, cycles: u8) {
        memory.client = MemoryBusClient::PPU;
        for _ in 0..cycles {
            self.step(memory);
        }
    }
}
