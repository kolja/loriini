
#![allow(unused_imports)]
use termion;
use termion::color;
use termion::raw::IntoRawMode;

use palette::{FromColor, Hsl, Hsv, Srgb};
use std::f64;
use std::io::{stdout, Write};
use std::iter::zip;
use itertools::Itertools;

use crate::model::Area;

fn term_color(color: &i16) -> termion::color::Rgb {
    let hue = ((*color as f64) / 255.0) * 360.0;
    let c = Hsl::new(hue, 1.0, 0.4);
    let srgb = Srgb::from_color(c);
    termion::color::Rgb(
        (srgb.red * 255.0) as u8,
        (srgb.green * 255.0) as u8,
        (srgb.blue * 255.0) as u8,
    )
}

fn mean(a: &i16, b: &i16) -> i16 {
    let a = *a;
    let b = *b;
    let c = (a + b) / 2;
    return c as i16;
}

pub fn draw(area: Area) {
    let mut stdout = stdout().into_raw_mode().unwrap();

    let circle = area.grid.into_iter().tuples()
        .map(|(row1, row2)| {
            zip(row1.into_iter().tuples::<(_,_)>(),
                row2.into_iter().tuples::<(_,_)>())
                .map(|(t1,t2)| match [t1, t2] {
                    [(0, 0), (0, 0)] => format!("{} ", color::Fg(term_color(&0))),
                    [(0, 0), (0, d)] => format!("{}▗", color::Fg(term_color(&d))),
                    [(0, 0), (c, 0)] => format!("{}▖", color::Fg(term_color(&c))),
                    [(0, 0), (c, d)] => format!("{}▄", color::Fg(term_color(&mean(&c,&d)))),
                    [(0, b), (0, 0)] => format!("{}▝", color::Fg(term_color(&b))),
                    [(0, b), (0, d)] => format!("{}▐", color::Fg(term_color(&mean(&b,&d)))),
                    [(0, b), (_c, 0)] => format!("{}▞", color::Fg(term_color(&b))),
                    [(0, _b), (_c, d)] => format!("{}▟", color::Fg(term_color(&d))),
                    [(a, 0), (0, 0)] => format!("{}▘", color::Fg(term_color(&a))),
                    [(a, 0), (0, _d)] => format!("{}▚", color::Fg(term_color(&a))),
                    [(a, 0), (_c, 0)] => format!("{}▌", color::Fg(term_color(&a))),
                    [(_a, 0), (c, _d)] => format!("{}▙", color::Fg(term_color(&c))),
                    [(a, b), (0, 0)] => format!("{}▀", color::Fg(term_color(&mean(&a,&b)))),
                    [(_a, b), (0, _d)] => format!("{}▜", color::Fg(term_color(&b))),
                    [(a, _b), (_c, 0)] => format!("{}▛", color::Fg(term_color(&a))),
                    [(a, b), (c, d)] => format!(
                        "{}{}▄{}",
                        color::Bg(term_color(&mean(&a, &b))),
                        color::Fg(term_color(&mean(&c, &d))),
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
