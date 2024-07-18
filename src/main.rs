mod color;
mod framebuffer;
mod bm;

use color::Color;
use framebuffer::FrameBuffer;
use bm::write_bmp_file;

fn main() {
    let background_color = Color::new(255, 255, 255); // Color de fondo blanco
    let foreground_color = Color::new(0, 0, 0); // Color de primer plano negro
    let mut framebuffer = FrameBuffer::new(800, 600, background_color, foreground_color);

    framebuffer.clear(); // Limpiar el framebuffer con el color de fondo

    let red = Color::new(255, 0, 0); // Color rojo
    framebuffer.point(400, 300, red); // Establecer el color del píxel en el centro

    let pixel_color = framebuffer.get_pixel(400, 300); // Obtener el color del píxel en el centro
    println!("Color del píxel en (400, 300): {:?}", pixel_color); // Imprimir el color del píxel

    // Guardar el framebuffer como un archivo BMP
    let buffer: Vec<Color> = framebuffer.buffer.iter()
        .map(|&color_value| {
            let red = ((color_value >> 16) & 0xFF) as u8;
            let green = ((color_value >> 8) & 0xFF) as u8;
            let blue = (color_value & 0xFF) as u8;
            Color::new(red, green, blue)
        })
        .collect();

    write_bmp_file("output.bmp", &buffer, framebuffer.width, framebuffer.height).expect("Error writing BMP file");
}
