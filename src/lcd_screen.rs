use crate::cpu::MemoryBus;
use raylib::prelude::*;

pub struct LCDScreen {
    pub image: Image,
}

impl LCDScreen {
    pub fn new() -> Self {
        Self {
            image: Image::gen_image_color(160, 144, Color::BLACK),
        }
    }

    // update the screen memory
    pub fn update(&mut self, memory_bus: &MemoryBus) {
        let mut i = 0;
        for y in 0..144 {
            for x in 0..160 {
                let color = match memory_bus.read_byte(0x8000 + i) {
                    0 => Color::BLACK,
                    1 => Color::DARKGRAY,
                    2 => Color::GRAY,
                    3 => Color::WHITE,
                    _ => Color::BLACK,
                };
                self.image.draw_pixel(x, y, color);
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

        // d.draw_text(&format!("Gameboy Emulator"), 10, 10, 20, Color::WHITE);
    }
}
