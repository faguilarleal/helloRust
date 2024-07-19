use crate::framebuffer::Framebuffer;

pub trait Line {
    fn line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize);
}

impl Line for Framebuffer {
    fn line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let mut x1 = x1 as isize;
        let mut y1 = y1 as isize;
        let x2 = x2 as isize;
        let y2 = y2 as isize;

        let dx = (x2 - x1).abs();
        let dy = -(y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            if x1 >= 0 && x1 < self.width() as isize && y1 >= 0 && y1 < self.height() as isize {
                self.point(x1 as usize, y1 as usize);
            }
            if x1 == x2 && y1 == y2 { break; }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x1 += sx;
            }
            if e2 <= dx {
                err += dx;
                y1 += sy;
            }
        }
    }
}
