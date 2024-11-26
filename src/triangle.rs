
use crate::model::Area;
use palette::Hsl;
use std::f64::consts::PI;

#[allow(dead_code)] // apparently, a0 is never used
struct Triangle {
    a0: f64,
    a1: f64,
    a2: f64,
    p0: (f64, f64),
    p1: (f64, f64),
    p2: (f64, f64),
    side: f64,
    height: f64,
    r: f64
}

impl Triangle {
    fn new(r: f64, phi: f64) -> Triangle {
        let a0 = phi;
        let a1 = phi + 120.0;
        let a2 = phi + 240.0;
        let p0 = (r * a0.to_radians().cos(), r * a0.to_radians().sin());
        let p1 = (r * a1.to_radians().cos(), r * a1.to_radians().sin());
        let p2 = (r * a2.to_radians().cos(), r * a2.to_radians().sin());
        let side = p0.diff(p1).sum_of_squares().sqrt();
        let height = (side * 3.0_f64.sqrt()) / 2.0;

        Triangle {
            a0, a1, a2, p0, p1, p2, side, height, r
        }
    }
}

trait Point {
    fn in_triangle(&self, t: &Triangle) -> bool;
    fn s_and_l(&self, t: &Triangle) -> (f64, f64);
    fn sum_of_squares(&self) -> f64;
    fn diff(&self, other: (f64, f64)) -> (f64, f64);
    fn angle(&self, other: (f64, f64)) -> f64;
}

impl Point for (f64, f64) {
    fn in_triangle(&self, t: &Triangle) -> bool {
        let (x1, y1) = t.p0;
        let (x2, y2) = t.p1;
        let (x3, y3) = t.p2;
        let (px, py) = (self.0, self.1);

        let denom = (y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3);
        let alpha = ((y2 - y3) * (px - x3) + (x3 - x2) * (py - y3)) / denom;
        let beta = ((y3 - y1) * (px - x3) + (x1 - x3) * (py - y3)) / denom;
        let gamma = 1.0 - alpha - beta;

        alpha >= 0.0 && beta >= 0.0 && gamma >= 0.0
    }

    // fn in_triangle_alternative(&self, t: &Triangle) -> bool {
    //     let curve = t.r - (t.a0 + self.angle((0.0, 0.0)) * 1.5).to_radians().sin().abs() * (t.r / 2.0);
    //     self.sum_of_squares().sqrt() < curve
    // }

    fn s_and_l(&self, t: &Triangle) -> (f64, f64) {

        let dark: bool = self.diff(t.p1).sum_of_squares() < self.diff(t.p2).sum_of_squares();

        let a: f64 = if dark {
            let self_angle = self.angle(t.p1);
            let total = self_angle + t.a1 - 120.0;
            (t.side / 2.0) * total.to_radians().tan()
        } else {
            let self_angle = self.angle(t.p2);
            let total = t.a2 + self_angle + 120.0;
            (t.side / 2.0) * total.to_radians().tan()
        };

        let sat = (a / t.height).abs();

        let one = ((a * a) + ((t.side * t.side) / 4.0)).sqrt() * 2.0;
        let lightness:f64 = if dark {
            let two = self.diff(t.p1).sum_of_squares().sqrt();
            two / one
        } else {
            let two = self.diff(t.p2).sum_of_squares().sqrt();
            1.0 - (two / one)
        };

        (sat, lightness)
    }
    fn sum_of_squares(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1
    }
    fn diff(&self, other: (f64, f64)) -> (f64, f64) {
        (self.0 - other.0, self.1 - other.1)
    }
    fn angle(&self, other: (f64, f64)) -> f64 {
        let (x, y) = self.diff(other);
        f64::atan2(x, y) * 180.0 / PI
    }
}

impl Area {
    pub fn triangle(&mut self) -> &mut Self {
        let angle = self.color.hue.into_degrees() as f64;
        let t = &Triangle::new(self.inner_radius, angle);

        for i in 0..self.height {
            for j in 0..self.width {
                let cols2 = self.width as f64 / 2.0;
                let rows2 = self.height as f64 / 2.0;

                let point = (
                    (j as f64 - cols2 + 0.5) * self.factorx,
                    i as f64 - rows2 + 0.5,
                );

                if point.in_triangle(t) {
                    let (sat, lightness) = point.s_and_l(t);
                    self.grid[i][j] = Some(Hsl::new(angle as f32, sat as f32, lightness as f32));
                }
            }
        }
        self
    }
}
