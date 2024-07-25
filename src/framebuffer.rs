// src/framebuffer.rs

use minifb::{Window, WindowOptions};

pub struct Framebuffer {
    width: usize,
    height: usize,
    pixels: Vec<u32>, // Use u32 for RGBA color values
    current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            pixels: vec![0x000000; width * height], // Initialize to black
            current_color: 0x000000, // Default color
        }
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    pub fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize) {
        for row in y..(y + height) {
            for col in x..(x + width) {
                if row < self.height && col < self.width {
                    self.pixels[row * self.width + col] = self.current_color;
                }
            }
        }
    }

    // Add getter methods for width and height
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn display(&self) {
        let mut window = Window::new("Maze", self.width, self.height, WindowOptions {
            resize: true,
            scale: minifb::Scale::X2,
            ..WindowOptions::default()
        }).expect("Unable to create window");
    
        while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
            window.update_with_buffer(&self.pixels, self.width, self.height)
                .expect("Failed to update buffer");
        }
    }
}
