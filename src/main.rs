use clap::Parser;
use hex::FromHex;
use palette::{FromColor, Hsl, Srgb};
use cli_clipboard;

mod model;
use model::{Area, Bar, EditMode};

mod circle;
mod draw;
mod triangle;

use std::io::{stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(Parser, Debug)]
#[clap(
    author = "Kolja Wilcke",
    version = "0.1",
    about = "A console color picker"
)]
struct Cli {
    // the number of rows:
    #[arg(short = 's', value_name = "Size", default_value_t = 12)]
    size: usize,

    #[arg(short = 'x', value_name = "factorx (float)", default_value_t = 0.5)]
    factorx: f64,

    #[arg(short = 'r', value_name = "outer radius", default_value_t = 6.0)]
    radius: f64,

    #[arg(short = 'i', long, value_name = "inner radius")]
    inner_radius: Option<f64>,

    #[arg(short = 'c', value_name = "color input (hex)", default_value_t = String::from("FF0000"))]
    color: String,
}

fn main() {
    let args = Cli::parse();
    let color = match <[u8; 3]>::from_hex(&args.color) {
        Ok([r, g, b]) => Hsl::from_color(Srgb::from_components((
            (r as f32) / 255.0,
            (g as f32) / 255.0,
            (b as f32) / 255.0,
        ))),
        Err(_) => panic!("failed to decode the color {}", args.color),
    };

    let height = args.size;
    let width = height * 2;

    let radius = args.radius;
    let inner_radius = match args.inner_radius {
        Some(r) => r,
        None => radius * 0.7,
    };
    let factorx = args.factorx;
    let mut show_info: bool = false;
    let mut edit_mode: EditMode = EditMode::Hue;

    let grid = vec![vec![None; width]; height];
    let mut area = Area {
        width,
        height,
        radius,
        inner_radius,
        factorx,
        color,
        grid,
    };

    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(
        stdout,
        "{}{}\r\n",
        termion::clear::All,
        area.circle().triangle().draw().join("\r\n")
    )
    .expect("`write!` failed");

    for c in std::io::stdin().keys() {
        let (h, s, l) = area.color.into_components();
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('i') => show_info = !show_info,
            Key::Char('h') => edit_mode = EditMode::Hue,
            Key::Char('a') => edit_mode = EditMode::Alpha,
            Key::Char('l') => edit_mode = EditMode::Lightness,
            Key::Char('s') => edit_mode = EditMode::Saturation,
            Key::Char('y') => {
                let srgb = Srgb::from_color(area.color);
                let hex = format!(
                    "{:02X}{:02X}{:02X}",
                    (srgb.red * 255.0) as u8,
                    (srgb.green * 255.0) as u8,
                    (srgb.blue * 255.0) as u8);
                    cli_clipboard::set_contents(hex).unwrap();
            },
            Key::Char('j') => match edit_mode {
                EditMode::Hue => area.color.hue -= 5.0,
                EditMode::Alpha => todo!(),
                EditMode::Lightness => area.color = Hsl::new(h, s, (l - 0.05).clamp(0.0, 1.0)),
                EditMode::Saturation => area.color = Hsl::new(h, (s - 0.05).clamp(0.0, 1.0), l),
            },
            Key::Char('k') => match edit_mode {
                EditMode::Hue => area.color.hue += 5.0,
                EditMode::Alpha => todo!(),
                EditMode::Lightness => area.color = Hsl::new(h, s, (l + 0.05).clamp(0.0, 1.0)),
                EditMode::Saturation => area.color = Hsl::new(h, (s + 0.05).clamp(0.0, 1.0), l),
            },
            _ => {}
        }
        let out: String = if show_info {
            let circle = area.circle().triangle();
            let circle_strings = circle.draw();
            let bars = circle.info(vec![Bar::Lightness, Bar::Saturation, Bar::Preview], 20);
            circle_strings
                .iter()
                .zip(bars.iter())
                .fold(String::new(), |acc, (c, b)| {
                    if acc.is_empty() {
                        format!("{}{}", c, b)
                    } else {
                        format!("{}{}{}", acc, "\r\n", format!("{}{}", c, b))
                    }
                })
        } else {
            area.circle().triangle().draw().join("\r\n")
        };
        write!(stdout, "{}{}\r\n", termion::clear::All, out).expect("write failed");
    }
}
