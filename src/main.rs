mod color;
mod framebuffer;
mod bm;
mod line;

use color::Color;
use framebuffer::FrameBuffer;
use bm::write_bmp_file;
use line::Line;

fn main() {
    let background_color = Color::new(0, 0, 0); // Color de fondo negro
    let foreground_color = Color::new(0, 0, 0); // Color de primer plano negro
    let mut framebuffer = FrameBuffer::new(800, 600, background_color, foreground_color);

    framebuffer.clear(); // Limpiar el framebuffer con el color de fondo

    let red = Color::new(255, 0, 0); // Color rojo
    framebuffer.point(400, 300, red); // Establecer el color del píxel en el centro

    // Obtener el color del píxel en el centro y manejar la opción
    match framebuffer.get_pixel(400, 300) {
        Some(pixel_color) => println!("Color del píxel en (400, 300): {:?}", pixel_color),
        None => println!("El píxel en (400, 300) está fuera de los límites."),
    }

    // Dibujar una línea
    let line_color = Color::new(0, 255, 0); // Color verde
    let line = Line::new(100, 100, 700, 500, line_color);
    line.draw(&mut framebuffer);

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
