use crate::color::Color;
use crate::framebuffer::FrameBuffer;

pub struct Line {
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
    color: Color,
}

impl Line {
    pub fn new(x0: usize, y0: usize, x1: usize, y1: usize, color: Color) -> Line {
        Line { x0, y0, x1, y1, color }
    }

    pub fn draw(&self, framebuffer: &mut FrameBuffer) {
        let mut x0 = self.x0 as isize;
        let mut y0 = self.y0 as isize;
        let x1 = self.x1 as isize;
        let y1 = self.y1 as isize;
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            if x0 >= 0 && x0 < framebuffer.width as isize && y0 >= 0 && y0 < framebuffer.height as isize {
                framebuffer.point(x0 as usize, y0 as usize, self.color);
            }
            if x0 == x1 && y0 == y1 { break; }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }
}
