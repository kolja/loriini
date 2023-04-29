extern crate neovim_lib;

use palette::{FromColor, Hsl, Srgb};
use cli_clipboard;
use neovim_lib::{Neovim, NeovimApi, Session};

use std::io::{stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::model::{Area, Slider};
use crate::editmode::Mode;

use crate::messages::Messages;


pub struct Loriini {
    pub nvim: Neovim,
    pub area: Area,
}


impl Loriini {
    pub fn new(area: Area) -> Loriini {
        let mut session = Session::new_tcp("127.0.0.1:6666").unwrap();
        // let mut session = Session::new_parent().unwrap();
        let nvim = Neovim::new(session);

        Loriini { nvim, area }
    }

    pub fn open_channel(&mut self) {
        let mut receiver = self.nvim.session.start_event_loop_channel().into_iter();
        let mut area = &mut self.area;
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
            // let (event, values) = &receiver.into_iter().next();

            let (event, values) = receiver.next().unwrap();
            // {
            //     Some(Ok(input)) => input,
            //     _ => break,
            // };
            match Messages::from(event) {
                Messages::Plus => {
                    self.nvim.command(&format!("echo \"Hello Loriini (plus)\"")).unwrap();
                },
                Messages::Minus => {
                    self.nvim.command(&format!("echo \"Hello Loriini (minus)\"")).unwrap();
                },
                Messages::Next => {},
                Messages::Prev => {},
                Messages::Copy => {},
                Messages::Quit => break,
                Messages::Unknown(_s) => {},
            }
        }
    }

    pub fn keyboard_input(&mut self) {

        let mut area = &mut self.area;
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
                Key::Char('h') | Key::Left => match area.edit_mode.active() {
                    Mode::Hue => area.color.hue -= 5.0,
                    // Mode::Alpha => todo!(),
                    Mode::Lightness => area.color = Hsl::new(h, s, (l - 0.05).clamp(0.0, 1.0)),
                    Mode::Saturation => area.color = Hsl::new(h, (s - 0.05).clamp(0.0, 1.0), l),
                },
                Key::Char('l') | Key::Right => match area.edit_mode.active() {
                    Mode::Hue => area.color.hue += 5.0,
                    // Mode::Alpha => todo!(),
                    Mode::Lightness => area.color = Hsl::new(h, s, (l + 0.05).clamp(0.0, 1.0)),
                    Mode::Saturation => area.color = Hsl::new(h, (s + 0.05).clamp(0.0, 1.0), l),
                },
                _ => {}
            }
        }
    }
}
