use crate::framebuffer::Framebuffer;

pub trait Line {
    fn line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize);
    fn draw_polygon(&mut self, points: &[(usize, usize)]);
    fn fill_polygon(&mut self, points: &[(usize, usize)]);
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

    fn draw_polygon(&mut self, points: &[(usize, usize)]) {
        let n = points.len();
        if n < 3 {
            eprintln!("A polygon requires at least 3 points.");
            return;
        }

        for i in 0..n {
            let (x1, y1) = points[i];
            let (x2, y2) = points[(i + 1) % n];
            self.line(x1, y1, x2, y2);
        }

        // Fill the polygon after drawing the outline
        self.fill_polygon(points);
    }

    fn fill_polygon(&mut self, points: &[(usize, usize)]) {
        let min_y = points.iter().map(|&(_, y)| y).min().unwrap_or(0);
        let max_y = points.iter().map(|&(_, y)| y).max().unwrap_or(self.height() as usize - 1);

        for y in min_y..=max_y {
            let mut intersections = Vec::new();
            for i in 0..points.len() {
                let (x1, y1) = points[i];
                let (x2, y2) = points[(i + 1) % points.len()];

                if (y1 <= y && y2 > y) || (y2 <= y && y1 > y) {
                    let t = (y as f32 - y1 as f32) / (y2 as f32 - y1 as f32);
                    let x = x1 as f32 + t * (x2 as f32 - x1 as f32);
                    intersections.push(x);
                }
            }

            intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

            for i in (0..intersections.len()).step_by(2) {
                if i + 1 < intersections.len() {
                    let x1 = intersections[i].round() as isize;
                    let x2 = intersections[i + 1].round() as isize;
                    for x in x1..=x2 {
                        if x >= 0 && x < self.width() as isize {
                            self.point(x as usize, y);
                        }
                    }
                }
            }
        }
    }
}
