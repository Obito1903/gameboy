use crate::cpu::MemoryBus;
use raylib::prelude::*;

#[derive(Copy, Clone)]
pub struct Pixel {
    pub color: Color,
}

#[derive(Copy, Clone)]
pub struct Tile {
    pub pixels: [Pixel; 8 * 8],
    pub tile_number: u8,
    pub object: bool,
}

impl Tile {
    pub fn new() -> Self {
        Self {
            pixels: [Pixel {
                color: Color::BLACK,
            }; 8 * 8],
            tile_number: 0,
            object: false,
        }
    }

    pub fn read_tile(memory_bus: &MemoryBus, address: u16) -> Self {
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
                    println!(
                        "Pixel at ({}, {}) is {:?} with byte1 {:X} and byte2 {:X}",
                        x, y, color, byte1, byte2
                    );
                }
            }
            i += 2;
        }
        tile
    }
}

pub struct PPU {
    pub image: Image,
    pub tiles: [Tile; 384],
}

impl PPU {
    pub fn new() -> Self {
        Self {
            image: Image::gen_image_color(160, 144, Color::BLACK),
            tiles: [Tile::new(); 384],
        }
    }

    // update the screen memory
    pub fn update(&mut self, memory_bus: &MemoryBus) {
        let lcd_control_flags = memory_bus.lcd_flags;
        // Read the tile data
        for i in 0..384 {
            self.tiles[i] = Tile::read_tile(memory_bus, 0x8000 + i as u16 * 16);

            self.tiles[i].tile_number = if i < 256 { i as u8 } else { 0 };

            if lcd_control_flags.bg_window_tile_data_area && i < 256 {
                self.tiles[i].object = true;
            } else if !lcd_control_flags.bg_window_tile_data_area && i > 128 {
                self.tiles[i].object = true;
                self.tiles[i].tile_number = if i < 256 {
                    i as u8
                } else {
                    255 - i as u8
                };
            }
        }

        let mut tile_numbers1: Vec<u8> = Vec::new();
        let mut tile_numbers2: Vec<u8> = Vec::new();

        // Read first tile map
        let mut i = 0;
        for y in 0..32 {
            for x in 0..32 {
                let tile_number = memory_bus.read_byte(0x9800 + i);
                tile_numbers1.push(tile_number);
                i += 1;
            }
        }

        // Read second tile map
        i = 0;
        for y in 0..32 {
            for x in 0..32 {
                let tile_number = memory_bus.read_byte(0x9C00 + i);
                tile_numbers2.push(tile_number);
                i += 1;
            }
        }

        let scx = memory_bus.read_byte(0xFF43);
        let scy = memory_bus.read_byte(0xFF42);

        let bottom = (scx + 143) % 255;
        let right =  (scy + 159) % 255;

        // Draw the tiles
        let mut i = 0;
        for y in 0..32 {
            for x in 0..32 {
                let tile_number = if lcd_control_flags.bg_window_tile_data_area {
                    tile_numbers2[i]
                } else {
                    tile_numbers1[i]
                };

                let tile = self.tiles[tile_number as usize];
                let mut j = 0;
                for ty in 0..8 {
                    for tx in 0..8 {
                        let pixel = tile.pixels[ty * 8 + tx];
                        self.image.draw_pixel(
                            x as i32 * 8 + tx as i32,
                            y as i32 * 8 + ty as i32,
                            pixel.color,
                        );
                        j += 1;
                    }
                }
                i += 1;
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

    }
}
