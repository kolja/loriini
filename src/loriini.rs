use palette::{FromColor, Hsl, Srgb};
use cli_clipboard;

use std::io::{stdout, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::model::{Area, Slider};
use crate::editmode::Mode;

use std::fs::{File, OpenOptions};
use crate::helpers;

pub struct Loriini {
    pub area: Area,
}

impl Loriini {
    pub fn new(area: Area) -> Loriini {
        Loriini {  area }
    }

    fn respond(file: &mut File, payload: String) -> Result<(), std::io::Error> {
        let mut color: String = payload.clone();
        color.push_str("\n");
        file.write_all(color.as_bytes()).unwrap();
        file.flush().unwrap();
        Ok(())
    }

    pub fn keyboard_input(&mut self) {

        let area = &mut self.area;
        
        let mut file = OpenOptions::new()
            .write(true)
            .open("/tmp/loriini")
            .expect("Failed to open named pipe for writing");

        let mut stdout = stdout().into_raw_mode().unwrap();
        loop {
            write!(
                stdout,
                "{}{}\r\n",
                termion::clear::All,
                area.circle()
                    .triangle()
                    .sliders(vec![Slider::Lightness(None), Slider::Saturation(None), Slider::Preview(None)], 20)
                    .draw()
                    .join("\r\n")
            ).expect("`write!` failed");

            let (h, s, l) = area.color.into_components();
            let key = match std::io::stdin().keys().next() {
                Some(Ok(input)) => input,
                _ => break,
            };
            match key {
                Key::Char('q') => break,
                Key::Char('i') => area.show_info = !area.show_info,
                Key::Char('j') | Key::Down => area.edit_mode.next(),
                Key::Char('k') | Key::Up => area.edit_mode.previous(),
                Key::Char('y') => {
                    let srgb = Srgb::from_color(area.color);
                    let hex = format!(
                        "{:02X}{:02X}{:02X}",
                        (srgb.red * 255.0) as u8,
                        (srgb.green * 255.0) as u8,
                        (srgb.blue * 255.0) as u8);
                        cli_clipboard::set_contents(hex).unwrap();
                },
                Key::Char('h') | Key::Left => {
                    match area.edit_mode.active() {
                        Mode::Hue => area.color.hue -= 5.0,
                        // Mode::Alpha => todo!(),
                        Mode::Lightness => area.color = Hsl::new(h, s, (l - 0.05).clamp(0.0, 1.0)),
                        Mode::Saturation => area.color = Hsl::new(h, (s - 0.05).clamp(0.0, 1.0), l),
                    }
                }
                ,
                Key::Char('l') | Key::Right => {
                    match area.edit_mode.active() {
                        Mode::Hue => area.color.hue += 5.0,
                        // Mode::Alpha => todo!(),
                        Mode::Lightness => area.color = Hsl::new(h, s, (l + 0.05).clamp(0.0, 1.0)),
                        Mode::Saturation => area.color = Hsl::new(h, (s + 0.05).clamp(0.0, 1.0), l),
                    }
                }
                _ => {}
            }

            Self::respond(&mut file, helpers::hsl_to_hex(&area.color)).unwrap();
        }
    }
}
