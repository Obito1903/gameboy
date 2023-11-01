use crate::cpu::MemoryBus;
use raylib::prelude::*;

#[derive(Copy, Clone)]
pub struct Pixel {
    pub color: Color,
}

#[derive(Copy, Clone)]
pub struct Tile {
    pub pixels: [Pixel; 8 * 8],
}

impl Tile {
    pub fn new() -> Self {
        Self {
            pixels: [Pixel {
                color: Color::BLACK,
            }; 8 * 8],
        }
    }

    pub fn read(memory_bus : &MemoryBus, address: u16) -> Self {
        let mut tile = Self::new();
        let mut i = 0;

        // println!("Reading tile at address {:X}", address);
        for y in 0..8 {
            let byte1 = memory_bus.read_byte(address + i);
            let byte2 = memory_bus.read_byte(address + i + 1);
            for x in 0..8 {
                let color = match ((byte1 >> (7 - x)) & 1) + (((byte2 >> (7 - x)) & 1) << 1) {
                    0 => Color::BLACK,
                    1 => Color::DARKGRAY,
                    2 => Color::GRAY,
                    3 => Color::WHITE,
                    _ => Color::BLACK,
                };
               
                tile.pixels[y * 8 + x] = Pixel { color };

                if address == 0x8000 {
                    println!("Pixel at ({}, {}) is {:?} with byte1 {:X} and byte2 {:X}", x, y, color, byte1, byte2);
                }
            }
            i += 2;

        
        }
        tile
    }
}

pub struct LCDScreen {
    pub image: Image,
    pub tiles: [Tile; 384],
}

impl LCDScreen {
    pub fn new() -> Self {
        Self {
            image: Image::gen_image_color(160, 144, Color::BLACK),
            tiles: [Tile::new(); 384],
        }
    }

    // update the screen memory
    pub fn update(&mut self, memory_bus: &MemoryBus) {
        for i in 0..384 {
            let tile = Tile::read(memory_bus, 0x8000 + i as u16 * 16);
            self.tiles[i] = tile;
        }

        self.image = Image::gen_image_color(160, 144, Color::BLACK);
        for y in 0..18 {
            for x in 0..20 {
                let tile = self.tiles[y * 20 + x];
                for i in 0..8 {
                    for j in 0..8 {
                        self.image.draw_pixel(
                            x as i32 * 8 + i as i32,
                            y as i32 * 8 + j as i32,
                            tile.pixels[i + j * 8].color,
                        );
                    }
                }
            }
        }
    }
    

    pub fn draw(&mut self, rl: &mut raylib::RaylibHandle, thread: &raylib::RaylibThread) {
        let scale = 3;

        // Convert the image to a texture
        let texture = rl.load_texture_from_image(thread, &self.image).unwrap();

        // Calculate the destination rectangle for drawing
        let dest_rect = Rectangle {
            x: 0.0,
            y: 0.0,
            width: (self.image.width * scale) as f32,
            height: (self.image.height * scale) as f32,
        };

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // Draw the scaled texture
        d.draw_texture_pro(
            &texture,
            Rectangle::new(0.0, 0.0, self.image.width as f32, self.image.height as f32),
            dest_rect,
            Vector2::default(),
            0.0,
            Color::WHITE,
        );

        // d.draw_text(&format!("Gameboy Emulator"), 10, 10, 20, Color::WHITE);
    }
}
