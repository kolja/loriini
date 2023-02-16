
use rand::distributions::{Distribution, Uniform};
use crate::model::Area;

fn point_in_circle(x: f64, y: f64, r: f64) -> bool {
    (x * x) + (y * y) < (r * r)
}

pub fn circle(mut area: Area) -> Area {

    // let mut rng = rand::thread_rng();
    // let die = Uniform::from(1..20);

    for i in 0..area.height {
        for j in 0..area.width {
            let cols2 = area.width as f64 / 2.0;
            let rows2 = area.height as f64 / 2.0;

            let x = (j as f64 - cols2 + 0.5) * area.factorx;
            let y = i as f64 - rows2 + 0.5;
            let angle = (f64::atan2(y, x) * 128.0 / std::f64::consts::PI) as i16;

            let within = point_in_circle(y, x, area.radius);
            // let withininner = point_in_ellipse(inner2, outer2, x, y);

            // let throw = die.sample(&mut rng);

            if within {
                // && !withininner {
                area.grid[i][j] = (angle + area.offset) % 256;
            } else {
                area.grid[i][j] = 0;
            }
        }
    }
    area
}
