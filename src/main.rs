mod framebuffer;
mod line;
mod bm;
mod color;

use framebuffer::Framebuffer;
use line::Line;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    // Clear the framebuffer with a white background
    framebuffer.set_background_color(0xFFFFFF);
    framebuffer.clear();

    // Set the current drawing color to black
    framebuffer.set_current_color(0x000000);

    // Draw some lines using Bresenham's algorithm
    framebuffer.line(100, 100, 700, 500);
    framebuffer.line(700, 100, 100, 500);
    framebuffer.line(400, 50, 400, 550);
    framebuffer.line(50, 300, 750, 300);

    // Draw and fill a polygon
    let polygon_points = [
        (100, 100),
        (700, 100),
        (700, 500),
        (100, 500),
    ];
    framebuffer.draw_polygon(&polygon_points);

    // Save the framebuffer as a BMP file
    framebuffer.render_buffer("output.bmp").expect("Error writing BMP file");

    println!("Framebuffer rendered to output.bmp");
}
