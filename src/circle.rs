
use crate::model::Area;
use palette::{LabHue, Lch};

fn point_in_circle(x: f64, y: f64, r: f64) -> bool {
    (x * x) + (y * y) < (r * r)
}

pub fn circle(mut area: Area) -> Area {

    for i in 0..area.height {
        for j in 0..area.width {
            let cols2 = area.width as f64 / 2.0;
            let rows2 = area.height as f64 / 2.0;

            let x = (j as f64 - cols2 + 0.5) * area.factorx;
            let y = i as f64 - rows2 + 0.5;

            let within = point_in_circle(y, x, area.radius);

            if within {
                let angle = (f64::atan2(y, x) * 180.0 / std::f64::consts::PI) as f32;
                let hue = LabHue::from_degrees(angle);
                area.grid[i][j] = Some(Lch::new(50.0, 100.0, hue));
            } else {
                area.grid[i][j] = None;
            }
        }
    }
    area
}
