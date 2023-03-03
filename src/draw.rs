use termion;
use termion::color::{Bg, Fg, Reset};

use itertools::Itertools;
use palette::{FromColor, Hsl, Mix, Srgb};
use std::iter::zip;

use crate::model::{Area, Bar};

fn term_color(color: &Hsl) -> termion::color::Rgb {
    let srgb = Srgb::from_color(*color);
    termion::color::Rgb(
        (srgb.red * 255.0) as u8,
        (srgb.green * 255.0) as u8,
        (srgb.blue * 255.0) as u8,
    )
}

fn hex_color(color: &Hsl) -> String {
    let srgb = Srgb::from_color(*color);
    format!(
        "{:02X}{:02X}{:02X}",
        (srgb.red * 255.0) as u8,
        (srgb.green * 255.0) as u8,
        (srgb.blue * 255.0) as u8
    )
}

impl Area {
    pub fn draw(&self) -> Vec<String> {
        self.grid
            .clone()
            .into_iter()
            .tuples()
            .map(|(row1, row2)| {
                zip(
                    row1.into_iter().tuples::<(_, _)>(),
                    row2.into_iter().tuples::<(_, _)>(),
                )
                .map(|(t1, t2)| match [t1, t2] {
                    [(None, None), (None, None)] => String::from(" "),
                    [(None, None), (None, Some(d))] => format!("{}▗", Fg(term_color(&d))),
                    [(None, None), (Some(c), None)] => format!("{}▖", Fg(term_color(&c))),
                    [(None, None), (Some(c), Some(d))] => {
                        format!("{}▄", Fg(term_color(&c.mix(&d, 0.5))))
                    }
                    [(None, Some(b)), (None, None)] => format!("{}▝", Fg(term_color(&b))),
                    [(None, Some(b)), (None, Some(d))] => {
                        format!("{}▐", Fg(term_color(&b.mix(&d, 0.5))))
                    }
                    [(None, Some(b)), (_c, None)] => format!("{}▞", Fg(term_color(&b))),
                    [(None, _b), (_c, Some(d))] => format!("{}▟", Fg(term_color(&d))),
                    [(Some(a), None), (None, None)] => format!("{}▘", Fg(term_color(&a))),
                    [(Some(a), None), (None, _d)] => format!("{}▚", Fg(term_color(&a))),
                    [(Some(a), None), (_c, None)] => format!("{}▌", Fg(term_color(&a))),
                    [(_a, None), (Some(c), _d)] => format!("{}▙", Fg(term_color(&c))),
                    [(Some(a), Some(b)), (None, None)] => {
                        format!("{}▀", Fg(term_color(&a.mix(&b, 0.5))))
                    }
                    [(_a, Some(b)), (None, _d)] => format!("{}▜", Fg(term_color(&b))),
                    [(Some(a), _b), (_c, None)] => format!("{}▛", Fg(term_color(&a))),
                    [(Some(a), Some(b)), (Some(c), Some(d))] => format!(
                        "{}{}▄{}",
                        Bg(term_color(&a.mix(&b, 0.5))),
                        Fg(term_color(&c.mix(&d, 0.5))),
                        Bg(Reset)
                    ),
                })
                .collect::<String>()
            })
            .collect::<Vec<String>>()
    }

    pub fn info(&self, bars: Vec<Bar>, width: u8) -> Vec<String> {
        let lines_before: usize = (self.height as f32 / 2.0).ceil() as usize - (2 * bars.len() - 1);
        let before = vec![String::from(""); lines_before].into_iter();
        let spacer = vec![String::from(""); bars.len() - 1];

        let step = 1.0 / (width * 2) as f32;
        let (h, s, l) = self.color.into_components();
        let marker_color = Hsl::new(h + 180.0, s, (l + 0.2).clamp(0.0,1.0));

        let bars = Itertools::interleave(
            bars.into_iter().map(|b| match b {
                Bar::Hue => {
                    todo!()
                }
                Bar::Alpha => {
                    todo!()
                }
                Bar::Saturation => {
                    let bar = (0..width).map(|i| {
                        let c1 = if (s * width as f32 * 2.0).floor() as u8 == i * 2 {
                            marker_color
                        } else {
                            Hsl::new(h, i as f32 * step * 2.0, l)
                        };
                        let c2 = if (s * width as f32 * 2.0).floor() as u8 + 1 == i * 2 {
                            marker_color
                        } else {
                            Hsl::new(h, i as f32 * step * 2.0 + step, l)
                        };

                        format!(
                            "{}{}▐",
                            Bg(term_color(&c1)),
                            Fg(term_color(&c2))
                        )
                    }).collect::<String>();
                    format!("{}{}", bar, Bg(Reset))
                }
                Bar::Lightness => {
                    let bar = (0..width).map(|i| {
                        let c1 = if (l * width as f32 * 2.0).floor() as u8 == i * 2 {
                            marker_color
                        } else {
                            Hsl::new(h, s, i as f32 * step * 2.0)
                        };
                        let c2 = if (l * width as f32 * 2.0).floor() as u8 + 1 == i * 2 {
                            marker_color
                        } else {
                            Hsl::new(h, s, i as f32 * step * 2.0 + step)
                        };
                        format!(
                            "{}{}▐",
                            Bg(term_color(&c1)),
                            Fg(term_color(&c2))
                        )
                    }).collect::<String>();
                    format!("{}{}", bar, Bg(Reset))
                }
                Bar::Preview => {
                    let mut text_color: Hsl = Hsl::from_color(self.color);
                    if self.color.lightness < 0.3 {
                        text_color.lightness += 0.3;
                    } else {
                        text_color.lightness -= 0.3;
                    }

                    format!(
                        "{}{} #{}{}{}",
                        Bg(term_color(&self.color)),
                        Fg(term_color(&text_color)),
                        hex_color(&self.color),
                        " ".repeat(width as usize - 8),
                        Bg(Reset)
                    )
                }
            }),
            spacer,
        );
        before.chain(bars).collect()
    }
}
