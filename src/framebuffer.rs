use crate::bm::write_bmp_file;
use crate::color::Color;

pub struct Framebuffer {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    background_color: u32,
    current_color: u32,
    line_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Framebuffer {
        let buffer = vec![0; width * height];
        Framebuffer {
            width,
            height,
            buffer,
            background_color: 0x000000, // Default background color (black)
            current_color: 0x000000, // Default drawing color (black)
            line_color: 0x000000, // Default line color (black)
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    pub fn set_line_color(&mut self, color: u32) {
        self.line_color = color;
    }
    
    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            // Invertir el eje Y
            let inverted_y = self.height - 1 - y;
            self.buffer[inverted_y * self.width + x] = self.current_color;
        }
    }

    pub fn render_buffer(&self, file_path: &str) -> std::io::Result<()> {
        let buffer: Vec<Color> = self.buffer.iter()
            .map(|&color_value| {
                let red = ((color_value >> 16) & 0xFF) as u8;
                let green = ((color_value >> 8) & 0xFF) as u8;
                let blue = (color_value & 0xFF) as u8;
                Color::new(red, green, blue)
            })
            .collect();
        write_bmp_file(file_path, &buffer, self.width, self.height)
    }
}
