
#![allow(unused)]  // FIXME

use crate::model::Area;
use palette::{LabHue, Hue, Lch, Hsl};

struct Triangle {
    p1: (f64, f64),
    p2: (f64, f64),
    p3: (f64, f64),
}

fn triangle(r: f64, phi: f64) -> Triangle {
    Triangle {
        p1: (r * phi.to_radians().cos(), r * phi.to_radians().sin()),
        p2: (r * (phi + 120.0).to_radians().cos(), r * (phi + 120.0).to_radians().sin()),
        p3: (r * (phi + 240.0).to_radians().cos(), r * (phi + 240.0).to_radians().sin())
    }
}

fn point_in_triangle(px: f64, py: f64, t: &Triangle) -> bool {

    let (x1, y1) = t.p1;
    let (x2, y2) = t.p2;
    let (x3, y3) = t.p3;

    let denom = (y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3);
    let alpha = ((y2 - y3) * (px - x3) + (x3 - x2) * (py - y3)) / denom;
    let beta = ((y3 - y1) * (px - x3) + (x1 - x3) * (py - y3)) / denom;
    let gamma = 1.0 - alpha - beta;

    alpha >= 0.0 && beta >= 0.0 && gamma >= 0.0
}

impl Area {
    pub fn triangle(&mut self) -> &mut Self {

        let angle = self.color.hue.to_degrees() as f64;
        let t = &triangle( self.inner_radius, angle );

        for i in 0..self.height {
            for j in 0..self.width {

                let cols2 = self.width as f64 / 2.0;
                let rows2 = self.height as f64 / 2.0;

                let x = (j as f64 - cols2 + 0.5) * self.factorx;
                let y = i as f64 - rows2 + 0.5;

                let within = point_in_triangle(x, y, t);

                if within {
                    self.grid[i][j] = Some(self.color)
                }

            }
        }
        self
    }
}


