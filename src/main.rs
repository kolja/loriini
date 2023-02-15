#![allow(unused_imports)]
use clap::Parser;
use termion;
use termion::color;
use termion::raw::IntoRawMode;

use palette::{FromColor, Hsl, Hsv, Srgb};
use std::f64;
use std::io::{stdout, Write};
use std::iter::zip;

#[derive(Parser, Debug)]
struct Cli {
    // the number of rows:
    #[arg(short = 's', value_name = "Size", default_value_t = 10)]
    size: usize,

    #[arg(short = 'x', value_name = "factorx (float)", default_value_t = 0.5)]
    factorx: f64,

    #[arg(short = 'r', value_name = "outer radius", default_value_t = 5.0)]
    radius: f64,

    #[arg(short = 'o', value_name = "color offset", default_value_t = 0)]
    offset: i8,
}

fn term_color(color: &i8) -> termion::color::Rgb {
    let hue = ((*color as f64) / 255.0) * 360.0;
    let c = Hsl::new(hue, 1.0, 0.4);
    let srgb = Srgb::from_color(c);
    termion::color::Rgb(
        (srgb.red * 255.0) as u8,
        (srgb.green * 255.0) as u8,
        (srgb.blue * 255.0) as u8,
    )
}

fn mean(a: &i8, b: &i8) -> i8 {
    let a = *a as i16;
    let b = *b as i16;
    let c = (a + b) / 2;
    return c as i8;
}

fn draw(area: Area) {
    let mut stdout = stdout().into_raw_mode().unwrap();

    let circle = area.grid
        .windows(2)
        .map(|rows| {
            zip(rows[0].windows(2), rows[1].windows(2))
                .map(|t| match [t.0, t.1] {
                    [[0, 0], [0, 0]] => format!("{} ", color::Fg(term_color(&0))),
                    [[0, 0], [0, d]] => format!("{}▗", color::Fg(term_color(d))),
                    [[0, 0], [c, 0]] => format!("{}▖", color::Fg(term_color(c))),
                    [[0, 0], [c, d]] => format!("{}▄", color::Fg(term_color(&mean(c,d)))),
                    [[0, b], [0, 0]] => format!("{}▝", color::Fg(term_color(b))),
                    [[0, b], [0, d]] => format!("{}▐", color::Fg(term_color(&mean(b,d)))),
                    [[0, b], [_c, 0]] => format!("{}▞", color::Fg(term_color(b))),
                    [[0, _b], [_c, d]] => format!("{}▟", color::Fg(term_color(d))),
                    [[a, 0], [0, 0]] => format!("{}▘", color::Fg(term_color(a))),
                    [[a, 0], [0, _d]] => format!("{}▚", color::Fg(term_color(a))),
                    [[a, 0], [_c, 0]] => format!("{}▌", color::Fg(term_color(a))),
                    [[_a, 0], [c, _d]] => format!("{}▙", color::Fg(term_color(c))),
                    [[a, b], [0, 0]] => format!("{}▀", color::Fg(term_color(&mean(a,b)))),
                    [[_a, b], [0, _d]] => format!("{}▜", color::Fg(term_color(b))),
                    [[a, _b], [_c, 0]] => format!("{}▛", color::Fg(term_color(a))),
                    [[a, b], [c, d]] => format!(
                        "{}{}▄{}",
                        color::Bg(term_color(&mean(a,b))),
                        color::Fg(term_color(&mean(c,d))),
                        color::Bg(color::Reset)
                    ),
                    _ => format!(" "),
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\r\n");

    write!(stdout, "{}\r\n", circle).expect("`write!` failed");

    // stdout.flush().unwrap();
}

fn point_in_circle(x: f64, y: f64, r: f64) -> bool {
    (x * x) + (y * y) < (r * r)
}

fn circle(mut area: Area) -> Area {

    for i in 0..area.height {
        for j in 0..area.width {
            let cols2 = area.width as f64 / 2.0;
            let rows2 = area.height as f64 / 2.0;

            let x = (j as f64 - cols2 + 0.5) * area.factorx;
            let y = i as f64 - rows2 + 0.5;
            let angle = (f64::atan2(x, y) * 128.0 / f64::consts::PI) as i8;

            let within = point_in_circle(x, y, area.radius);
            // let withininner = point_in_ellipse(inner2, outer2, x, y);

            if within {
                // && !withininner {
                area.grid[i][j] = angle;
            } else {
                area.grid[i][j] = 0;
            }
        }
    }
    area
}

struct Area {
    width: usize,
    height: usize,
    radius: f64,
    factorx: f64,
    offset: i8,
    grid: Vec<Vec<i8>>,
}

fn main() {
    let args = Cli::parse();

    let height = args.size;
    let width = height * 2;

    let radius = args.radius;
    let factorx = args.factorx;
    let offset = args.offset;

    let grid = vec![vec![0; width]; height];
    let area = Area { width, height, radius, factorx, offset, grid };
    draw(circle(area));
}
