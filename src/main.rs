mod color;
mod framebuffer;

use color::Color;
use framebuffer::FrameBuffer;

fn main() {
    let background_color = Color::new(255, 255, 255); // Color de fondo blanco
    let foreground_color = Color::new(0, 0, 0); // Color de primer plano negro
    let mut framebuffer = FrameBuffer::new(800, 600, background_color, foreground_color);

    framebuffer.clear(); // Limpiar el framebuffer con el color de fondo

    let red = Color::new(255, 0, 0); // Color rojo
    framebuffer.point(400, 300, red); // Establecer el color del píxel en el centro

    let pixel_color = framebuffer.get_pixel(400, 300); // Obtener el color del píxel en el centro
    println!("{:?}", pixel_color); // Imprimir el color del píxel

    // Cambiar el color de fondo y limpiar nuevamente
    let green_background = Color::new(0, 255, 0); // Color de fondo verde
    framebuffer.set_background_color(green_background);
    framebuffer.clear();

    let pixel_color = framebuffer.get_pixel(400, 300); // Obtener el color del píxel en el centro
    println!("{:?}", pixel_color); // Debería ser verde ahora
}
