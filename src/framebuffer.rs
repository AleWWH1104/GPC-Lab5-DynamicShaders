// framebuffer.rs
// use raylib::prelude::*; // Remove raylib import for Color

// Define a simple color type or use u32 directly
type Color = u32;

// Helper function to convert RGB (u8) to u32 (0xRRGGBB)
fn color_to_u32(r: u8, g: u8, b: u8) -> Color {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

// Helper function to convert raylib Color to u32 if needed elsewhere
// fn rl_color_to_u32(rl_color: raylib::prelude::Color) -> Color {
//     color_to_u32(rl_color.r, rl_color.g, rl_color.b)
// }

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>, // Keep buffer as u32
    pub zbuffer: Vec<f32>,
    background_color: Color,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height], // Initialize with black (0x000000)
            zbuffer: vec![f32::INFINITY; width * height],
            background_color: color_to_u32(0, 0, 0), // Black background
        }
    }

    pub fn clear(&mut self) {
        // Use the stored background color value directly
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
        for depth in self.zbuffer.iter_mut() {
            *depth = f32::INFINITY;
        }
    }

    pub fn point(&mut self, x: usize, y: usize, color: Color) { // Accept u32 color
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.buffer[index] = color;
        }
    }

    pub fn point_with_depth(&mut self, x: usize, y: usize, depth: f32, color: Color) { // Accept u32 color
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            if depth < self.zbuffer[index] {
                self.zbuffer[index] = depth;
                self.buffer[index] = color; // Store u32 color
            }
        }
    }

    pub fn set_background_color(&mut self, color: Color) { // Accept u32 color
        self.background_color = color;
    }
}