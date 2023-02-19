
#![allow(unused_imports)]
use termion;
use termion::color;
use termion::raw::IntoRawMode;

use palette::{Pixel, FromColor, IntoColor, Hue, Mix, Hsl, Hsv, Lch, Srgb};
use std::f64;
use std::io::{stdout, Write};
use std::iter::zip;
use itertools::Itertools;

use crate::model::Area;

fn term_color(color: &Hsl) -> termion::color::Rgb {
    let srgb = Srgb::from_color(*color);
    termion::color::Rgb(
        (srgb.red * 255.0) as u8,
        (srgb.green * 255.0) as u8,
        (srgb.blue * 255.0) as u8,
    )
}

impl Area {
    pub fn draw(&self) -> String {
        // let mut stdout = stdout().into_raw_mode().unwrap();

        self.grid.into_iter().tuples()
            .map(|(row1, row2)| {
                zip(row1.into_iter().tuples::<(_,_)>(),
                    row2.into_iter().tuples::<(_,_)>())
                    .map(|(t1,t2)| match [t1, t2] {
                        [(None, None), (None, None)] => String::from(" "),
                        [(None, None), (None, Some(d))] => format!("{}▗", color::Fg(term_color(&d))),
                        [(None, None), (Some(c), None)] => format!("{}▖", color::Fg(term_color(&c))),
                        [(None, None), (Some(c), Some(d))] => format!("{}▄", color::Fg(term_color(&c.mix(&d, 0.5)))),
                        [(None, Some(b)), (None, None)] => format!("{}▝", color::Fg(term_color(&b))),
                        [(None, Some(b)), (None, Some(d))] => format!("{}▐", color::Fg(term_color(&b.mix(&d, 0.5)))),
                        [(None, Some(b)), (_c, None)] => format!("{}▞", color::Fg(term_color(&b))),
                        [(None, _b), (_c, Some(d))] => format!("{}▟", color::Fg(term_color(&d))),
                        [(Some(a), None), (None, None)] => format!("{}▘", color::Fg(term_color(&a))),
                        [(Some(a), None), (None, _d)] => format!("{}▚", color::Fg(term_color(&a))),
                        [(Some(a), None), (_c, None)] => format!("{}▌", color::Fg(term_color(&a))),
                        [(_a, None), (Some(c), _d)] => format!("{}▙", color::Fg(term_color(&c))),
                        [(Some(a), Some(b)), (None, None)] => format!("{}▀", color::Fg(term_color(&a.mix(&b, 0.5)))),
                        [(_a, Some(b)), (None, _d)] => format!("{}▜", color::Fg(term_color(&b))),
                        [(Some(a), _b), (_c, None)] => format!("{}▛", color::Fg(term_color(&a))),
                        [(Some(a), Some(b)), (Some(c), Some(d))] => format!(
                            "{}{}▄{}",
                            color::Bg(term_color(&a.mix(&b, 0.5))),
                            color::Fg(term_color(&c.mix(&d, 0.5))),
                            color::Bg(color::Reset)
                        ),
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\r\n")

        // write!(stdout, "{}\r\n", circle).expect("`write!` failed");

        // stdout.flush().unwrap();
    }
}
