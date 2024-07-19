mod framebuffer;
mod line;
mod bm;
mod color;


use framebuffer::Framebuffer;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    // Clear the framebuffer with a white background
    framebuffer.set_background_color(0x000000);

    framebuffer.clear();

    // Set the current drawing color to black
    framebuffer.set_current_color(0xFFFB00);
    framebuffer.set_line_color(0xFFFFFF); 


    // Draw and fill a polygon
    let polygon_points = [
        (165, 380),
         (185, 360),
          (180, 330),
           (207, 345),
            (233, 330),
             (230, 360),
              (250, 380),
               (220, 385),
                (205, 410),
                 (193, 383)
    ];

    let polygon_points2 =[
        (321, 335),
         (288, 286),
          (339, 251),
           (374, 302)
    ];

    println!("Polygon 1  !!");
    framebuffer.draw_polygon(&polygon_points, 0xFFFFFF, 0xFFFB00);

    println!("Polygon 2 !!");
    framebuffer.draw_polygon(&polygon_points2, 0xFFFFFF, 0x0097FF);

    // Save the framebuffer as a BMP file
    framebuffer.render_buffer("output.bmp").expect("Error writing BMP file");

    println!("Framebuffer rendered to output.bmp");
}
