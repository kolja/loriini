
use crate::model::Area;
use palette::Hsl;
use std::f64::consts::PI;

fn point_in_circle(x: f64, y: f64, r: f64, ir: f64) -> bool {
    let x2plusy2 = (x * x) + (y * y);
    x2plusy2 < (r * r) && x2plusy2 > (ir * ir)
}

impl Area {
    pub fn circle(&mut self) -> &mut Self {

        for i in 0..self.height {
            for j in 0..self.width {
                let cols2 = self.width as f64 / 2.0;
                let rows2 = self.height as f64 / 2.0;

                let x = (j as f64 - cols2 + 0.5) * self.factorx;
                let y = i as f64 - rows2 + 0.5;

                let within = point_in_circle(x, y, self.radius, self.inner_radius);

                if within {
                    let angle = (f64::atan2(y, x) * 180.0 / PI) as f32;
                    self.grid[i][j] = Some(Hsl::new(angle, 1.0, 0.5));
                } else {
                    self.grid[i][j] = None;
                }
            }
        }
       self
    }
}

