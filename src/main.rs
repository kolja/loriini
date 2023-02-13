#![allow(unused_imports)]
use clap::Parser;
use termion;
use termion::color;
use termion::raw::IntoRawMode;

use palette::{FromColor, Hsv, Hsl, Srgb};
use std::f64;
use std::io::{stdout, Write};
use std::iter::zip;

#[derive(Parser, Debug)]
struct Cli {
    /// the number of rows:
    size: usize
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

const ROW_COUNT: usize = 12;
const COL_COUNT: usize = ROW_COUNT * 2;

fn draw(matrix: [[i8; COL_COUNT]; ROW_COUNT]) {

    let mut stdout = stdout().into_raw_mode().unwrap();

    let circle = matrix
        .windows(2)
        .map(|rows| {
            zip(rows[0].windows(2), rows[1].windows(2))
                .map(|t| match [t.0, t.1] {
                    [[0, 0], [0, 0]]   => format!("{} ", color::Fg(term_color(&0))),
                    [[0, 0], [0, d]]   => format!("{}▗", color::Fg(term_color(d))),
                    [[0, 0], [c, 0]]   => format!("{}▖", color::Fg(term_color(c))),
                    [[0, 0], [c, _d]]  => format!("{}▄", color::Fg(term_color(c))),
                    [[0, b], [0, 0]]   => format!("{}▝", color::Fg(term_color(b))),
                    [[0, _b], [0, d]]  => format!("{}▐", color::Fg(term_color(d))),
                    [[0, b], [_c, 0]]  => format!("{}▞", color::Fg(term_color(b))),
                    [[0, _b], [_c, d]] => format!("{}▟", color::Fg(term_color(d))),
                    [[a, 0], [0, 0]]   => format!("{}▘", color::Fg(term_color(a))),
                    [[a, 0], [0, _d]]  => format!("{}▚", color::Fg(term_color(a))),
                    [[a, 0], [_c, 0]]  => format!("{}▌", color::Fg(term_color(a))),
                    [[a, 0], [_c, _d]] => format!("{}▙", color::Fg(term_color(a))),
                    [[a, _b], [0, 0]]  => format!("{}▀", color::Fg(term_color(a))),
                    [[a, _b], [0, _d]] => format!("{}▜", color::Fg(term_color(a))),
                    [[a, _b], [_c, 0]] => format!("{}▛", color::Fg(term_color(a))),
                    [[a, _b], [_c, d]] => format!("{}{}▄{}",
                                                    color::Bg(term_color(a)),
                                                    color::Fg(term_color(d)),
                                                    color::Bg(color::Reset)),
                    _ => format!(" "),
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\r\n");

    write!(stdout, "{}\r\n", circle).expect("`write!` failed");

    // stdout.flush().unwrap();
}

fn point_in_ellipse(a: f64, b: f64, x: f64, y: f64) -> bool {
    (x * x) / (a * a) + (y * y) / (b * b) < 1.0
}


fn circle() -> [[i8; COL_COUNT]; ROW_COUNT] {

    let mut matrix = [[0; COL_COUNT]; ROW_COUNT];

    for i in 0..ROW_COUNT {
        for j in 0..COL_COUNT {
            let inner = ROW_COUNT as f64 / 2.0;
            let outer = COL_COUNT as f64 / 2.0;

            // let inner2 = (ROW_COUNT - 3) as f64 / 2.0;
            // let outer2 = (COL_COUNT - 5) as f64 / 2.0;

            let x = i as f64 - inner + 0.5;
            let y = j as f64 - outer + 0.5;
            let angle = (f64::atan2(x,y) * 128.0 / f64::consts::PI) as i8;

            let within = point_in_ellipse(inner, outer, x, y);
            // let withininner = point_in_ellipse(inner2, outer2, x, y);

            if within {// && !withininner {
                matrix[i][j] = angle;
            } else {
                matrix[i][j] = 0;
            }
        }
    }
    matrix
}

struct Area {
    width: usize,
    height: usize,
}

fn main() {
    let args = Cli::parse();
    let width = args.size * 2;
    let height = args.size;
    // Todo: don't rely on globals. Pass in Area struct instead.
    // Also: store Array data on that very struct.
    let _area = Area { width, height };
    draw(circle());
}

