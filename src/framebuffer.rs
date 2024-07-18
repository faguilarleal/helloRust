use super::color::Color;

pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    background_color: Color,
    foreground_color: Color,
}

impl FrameBuffer {
    // Constructor para crear un nuevo framebuffer
    pub fn new(width: usize, height: usize, background_color: Color, foreground_color: Color) -> FrameBuffer {
        let buffer = vec![0; width * height];
        let mut fb = FrameBuffer { width, height, buffer, background_color, foreground_color };
        fb.clear(); // Inicializar el framebuffer con el color de fondo
        fb
    }

    // Método para obtener el índice de un píxel en el buffer
    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    // Método para convertir un Color a u32
    fn color_to_u32(color: &Color) -> u32 {
        (color.red() as u32) << 16 | (color.green() as u32) << 8 | (color.blue() as u32)
    }

    // Método para limpiar el framebuffer con el color de fondo
    pub fn clear(&mut self) {
        let color_value = FrameBuffer::color_to_u32(&self.background_color);
        for pixel in self.buffer.iter_mut() {
            *pixel = color_value;
        }
    }

    // Método para establecer el color de un píxel
    pub fn point(&mut self, x: usize, y: usize, color: Color) {
        let index = self.get_index(x, y);
        let color_value = FrameBuffer::color_to_u32(&color);
        self.buffer[index] = color_value;
    }

    // Método para obtener el color de un píxel
    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        let index = self.get_index(x, y);
        let color_value = self.buffer[index];
        let red = ((color_value >> 16) & 0xFF) as u8;
        let green = ((color_value >> 8) & 0xFF) as u8;
        let blue = (color_value & 0xFF) as u8;
        Color::new(red, green, blue)
    }

    // Métodos para configurar los colores de fondo y primer plano
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_foreground_color(&mut self, color: Color) {
        self.foreground_color = color;
    }
}
