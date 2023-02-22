
#![allow(unused)]  // FIXME

use crate::model::Area;
use palette::{LabHue, Hue, Lch, Hsl};

fn point_in_triangle(px: f64, py: f64, r: f64, angle: f64) -> bool {
    let phi = angle;
    let x1 = r * phi.to_radians().cos();
    let y1 = r * phi.to_radians().sin();
    let x2 = r * (phi + 120.0).to_radians().cos();
    let y2 = r * (phi + 120.0).to_radians().sin();
    let x3 = r * (phi + 240.0).to_radians().cos();
    let y3 = r * (phi + 240.0).to_radians().sin();

    let denom = (y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3);
    let alpha = ((y2 - y3) * (px - x3) + (x3 - x2) * (py - y3)) / denom;
    let beta = ((y3 - y1) * (px - x3) + (x1 - x3) * (py - y3)) / denom;
    let gamma = 1.0 - alpha - beta;
    alpha >= 0.0 && beta >= 0.0 && gamma >= 0.0
}

impl Area {
    pub fn triangle(&mut self) -> &mut Self {

        let angle = self.color.hue.to_degrees() as f64;
        for i in 0..self.height {
            for j in 0..self.width {

                let cols2 = self.width as f64 / 2.0;
                let rows2 = self.height as f64 / 2.0;

                let x = (j as f64 - cols2 + 0.5) * self.factorx;
                let y = i as f64 - rows2 + 0.5;

                let within = point_in_triangle(x, y, self.inner_radius, angle);

                if within {
                    self.grid[i][j] = Some(self.color)
                }

            }
        }
        self
    }
}


