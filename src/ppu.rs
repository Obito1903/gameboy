use pixels::{Pixels, SurfaceTexture};
use winit::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder};

use crate::bus::{
    lcd::SpriteSize,
    oam::{self, ObjectAttribute},
    Bus, MemoryLockOwner, MemoryRegion,
};

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

struct Object {
    attributes: oam::ObjectAttribute,
    pixels: [Pixel; 64],
}

pub struct PPU {
    dot_counter: u16,
    event_loop: EventLoop<()>,
    window: winit::window::Window,
    pixels: Pixels,

    objects_buffer: Vec<ObjectAttribute>,
    fifo_background: Vec<Pixel>,
    fifo_object: Vec<Pixel>,

    // Mode 2
    oam_progress: u8,
}

const WIDTH: u32 = 160;
const HEIGHT: u32 = 144;

impl PPU {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = {
            let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
            WindowBuilder::new()
                .with_title("Hello Pixels")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .with_resizable(false)
                .build(&event_loop)
                .unwrap()
        };

        let pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
        };

        Self {
            dot_counter: 0,
            event_loop,
            window,
            pixels,
            objects_buffer: Vec::with_capacity(10),
            fifo_background: Vec::with_capacity(16),
            fifo_object: Vec::with_capacity(16),

            oam_progress: 0,
        }
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

    fn switch_to_mode2(&mut self, memory: &mut Bus) {
        self.objects_buffer.clear();
        // push rendered pixels to screen
        if let Err(err) = self.pixels.render() {
            panic!("pixels.render failed: {}", err);
        }
        self.window.request_redraw();

        // Lock OAM
        memory.lock(MemoryRegion::OAM);
        memory.io.lcd.status.stat.ppu_mode = 2;
    }

    pub fn mode2(&mut self, memory: &mut Bus) {
        // Load OAM
        if self.objects_buffer.len() < 10 && self.dot_counter % 2 == 0 {
            let oa = memory.oam.data[(self.oam_progress) as usize];
            self.objects_buffer.push(oa);
            if memory.io.lcd.control.obj_size == SpriteSize::Size8x16 {
                self.oam_progress += 1;
                self.objects_buffer
                    .push(memory.oam.data[(self.oam_progress) as usize]);
            }
            self.oam_progress += 1;
        }

        if self.dot_counter == 80 {
            self.switch_to_mode3(memory);
        }
    }

    fn switch_to_mode3(&mut self, memory: &mut Bus) {
        self.oam_progress = 0;
        // Lock VRAM
        memory.lock(MemoryRegion::VRAM);
        memory.io.lcd.status.stat.ppu_mode = 3;
    }

    pub fn mode3(&mut self, memory: &mut Bus) {
        let frame = self.pixels.frame_mut();
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            // let x = (i % WIDTH as usize) as i16;
            // let y = (i / WIDTH as usize) as i16;

            let rgba = [0x5e, 0x48, 0xe8, 0xff];

            pixel.copy_from_slice(&rgba);
        }
        if self.dot_counter == 252 {
            Self::switch_to_mode0(memory);
        }
    }

    pub fn step(&mut self, memory: &mut Bus) {
        Self::update_lyc(memory);
        Self::update_stat_interupt(memory);

        if memory.io.lcd.status.ly == 144 && self.dot_counter == 0 {
            Self::switch_to_mode1(memory);
        }
        if memory.io.lcd.status.ly == 153 {
            memory.io.lcd.status.ly = 0;
            self.switch_to_mode2(memory);
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
            // Advance dot counter by 4 (4 dot per cycle in single speed mode)
            self.step(memory);
            self.step(memory);
            self.step(memory);
            self.step(memory);
        }
    }
}
