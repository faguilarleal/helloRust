use crate::framebuffer::Framebuffer;

pub trait Line {
    fn line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize);
    fn draw_polygon(&mut self, points: &[(usize, usize)], border_color: u32, fill_color: u32);
    fn fill_polygon(&mut self, points: &[(usize, usize)], border_color: u32);
}

impl Framebuffer {
    pub fn line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, thickness: usize) {
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
                // Draw the line with thickness
                for i in -(thickness as isize) / 2..=thickness as isize / 2 {
                    for j in (-(thickness as isize) / 2)..=(thickness as isize / 2) {
                        let nx = x1 + i;
                        let ny = y1 + j;
                        if nx >= 0 && nx < self.width() as isize && ny >= 0 && ny < self.height() as isize {
                            self.point(nx as usize, ny as usize);
                        }
                    }
                }
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

    pub fn draw_polygon(&mut self, points: &[(usize, usize)], border_color: u32, fill_color: u32) {
        let n = points.len();
        if n < 3 {
            eprintln!("A polygon requires at least 3 points.");
            return;
        }

        // Set border color and draw the edges of the polygon with thickness
        self.set_current_color(border_color);
        let border_thickness = 2; // Adjust thickness as needed
        for i in 0..n {
            let (x1, y1) = points[i];
            let (x2, y2) = points[(i + 1) % n];
            self.line(x1, y1, x2, y2, border_thickness);
        }

        // Fill the polygon with the fill color
        self.set_current_color(fill_color);
        self.fill_polygon(points, fill_color);
    }

    pub fn fill_polygon(&mut self, points: &[(usize, usize)], fill_color: u32) {
        let min_y = points.iter().map(|(_, y)| *y).min().unwrap() as isize;
        let max_y = points.iter().map(|(_, y)| *y).max().unwrap() as isize;

        for y in min_y..=max_y {
            let mut intersections = Vec::new();
            for i in 0..points.len() {
                let (x1, y1) = points[i];
                let (x2, y2) = points[(i + 1) % points.len()];

                if (y1 <= y.try_into().unwrap() && y2 > y.try_into().unwrap()) || (y2 <= y.try_into().unwrap() && y1 > y.try_into().unwrap()) {
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
                        self.point(x as usize, y as usize);
                    }
                }
            }
        }
    }
}
