use termion;
use termion::color::{Bg, Fg, Reset};

use itertools::Itertools;
use palette::{FromColor, Hsl, Mix, Srgb};
use std::iter::zip;

use crate::model::{Area, Slider};
use crate::editmode::Mode;

use crate::helpers;

fn term_color(color: &Hsl) -> termion::color::Rgb {
    let srgb = Srgb::from_color(*color);
    termion::color::Rgb(
        (srgb.red * 255.0) as u8,
        (srgb.green * 255.0) as u8,
        (srgb.blue * 255.0) as u8,
    )
}

impl Area {

    fn selected(is_selected: bool, color: Hsl) -> String {
        if is_selected {
            format!("{}▕", Fg(term_color(&color)))
        } else {
            " ".to_string()
        }
    }

    pub fn draw(&mut self) -> Vec<String> {
        let colorwheel = self.draw_colorwheel();
        if self.show_info {
            colorwheel
                .iter()
                .zip(self.draw_sliders().iter())
                .map(|(cw, sl)| { format!("{}{}", cw, sl) })
                .collect::<Vec<String>>()
        } else {
            colorwheel
        }
    }

    fn draw_colorwheel(&mut self) -> Vec<String> {
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

    fn draw_sliders(&mut self) -> Vec<String> {
        let lines_before: usize = self.height / 2 - (2 * self.sliders.len() - 1);
        let before = vec![String::from(""); lines_before].into_iter();
        let spacer = vec![String::from(""); self.sliders.len() - 1];

        let sliders = self.sliders.iter().map(|slider|
            match slider {
                Slider::Hue(Some(_data)) => {
                    todo!()
                },
                Slider::Alpha(Some(_data)) => {
                    todo!()
                },
                Slider::Saturation(Some(data)) => {
                    let bar = data.colors.chunks(2).map(|cnk| {
                        let c1 = cnk[0];
                        let c2 = cnk[1];
                        format!(
                            "{}{}▐",
                            Bg(term_color(&c1)),
                            Fg(term_color(&c2))
                        )}
                    ).collect::<String>();

                    let sel = Area::selected(self.edit_mode.is_active(Mode::Saturation), Hsl::new(1.0,1.0,1.0));
                    format!("{}{}{}", sel, bar, Bg(Reset))
                },
                Slider::Lightness(Some(data)) => {
                    let bar = data.colors.chunks(2).map(|cnk| {
                        let c1 = cnk[0];
                        let c2 = cnk[1];
                        format!(
                            "{}{}▐",
                            Bg(term_color(&c1)),
                            Fg(term_color(&c2))
                        )}
                    ).collect::<String>();

                    let sel = Area::selected(self.edit_mode.is_active(Mode::Lightness), Hsl::new(1.0,1.0,1.0));
                    format!("{}{}{}", sel, bar, Bg(Reset))
                },
                Slider::Preview(Some(width)) => {
                    let mut text_color: Hsl = Hsl::from_color(self.color);
                    if self.color.lightness < 0.3 {
                        text_color.lightness += 0.3;
                    } else {
                        text_color.lightness -= 0.3;
                    }

                    let sel = Area::selected(self.edit_mode.is_active(Mode::Hue), Hsl::new(1.0,1.0,1.0));
                    format!(
                        "{}{}{} #{}{}{}",
                        sel,
                        Fg(term_color(&text_color)),
                        Bg(term_color(&self.color)),
                        helpers::hsl_to_hex(&self.color),
                        " ".repeat(*width as usize - 8),
                        Bg(Reset)
                    )
                }
                _ => "".to_string()
        });

        before.chain(Itertools::interleave(sliders, spacer)).collect()
    }
}
