
use crate::EditMode;

pub struct Area {
    pub width: usize,
    pub height: usize,
    pub radius: f64,
    pub inner_radius: f64,
    pub factorx: f64,
    pub color: palette::Hsl,
    pub grid: Vec<Vec<Option<palette::Hsl>>>,
    pub show_info: bool,
    pub edit_mode: EditMode,
    pub sliders: Vec<Slider>,
    pub pipe: Option<String>
}

pub struct SliderData {
    pub colors: Vec<palette::Hsl>,
    pub width: u8,
    pub pos: u8
}

#[allow(dead_code)] // allow for different kinds of color sliders
pub enum Slider {
    // Red
    // Green,
    // Blue,
    Hue(Option<SliderData>),
    Lightness(Option<SliderData>),
    Saturation(Option<SliderData>),
    Alpha(Option<SliderData>),
    Preview(Option<u8>)
}

#[cfg(test)]
mod tests {

    use std::fs;
    use std::io::{stdout, Write};

    use hex::FromHex;
    use palette::{FromColor, Hsl, Srgb};
    use termion::raw::IntoRawMode;

    use crate::model::{Area, Slider};
    use crate::editmode::{EditMode, Mode};

    fn setup(show_info: bool) -> Area {
        let color = match <[u8; 3]>::from_hex(String::from("A1B2C3")) {
            Ok([r, g, b]) => Hsl::from_color(Srgb::from_components((
                (r as f32) / 255.0,
                (g as f32) / 255.0,
                (b as f32) / 255.0,
            ))),
            Err(_) => panic!("parse failed"),
        };

        let height = 12;
        let width = height * 2;
        let radius = 6.0;
        let inner_radius = radius * 0.7;
        let factorx = 0.5;
        let pipe = None;

        let grid = vec![vec![None; width]; height];

        Area {
            width,
            height,
            radius,
            inner_radius,
            factorx,
            color,
            show_info,
            edit_mode: EditMode { modes: vec![Mode::Hue, Mode::Lightness, Mode::Saturation] },
            grid,
            sliders: Vec::new(),
            pipe
        }
    }

    struct TestData {
        ring: String,
        triangle: String,
        sliders: String,
    }

    fn data() -> TestData {
        TestData {
            ring: String::from_utf8(fs::read("testdata/ring.txt").unwrap()).unwrap(),
            triangle: String::from_utf8(fs::read("testdata/triangle.txt").unwrap()).unwrap(),
            sliders: String::from_utf8(fs::read("testdata/sliders.txt").unwrap()).unwrap(),
        }
    }

    #[test]
    #[ignore]
    fn ring() {
        let with_sliders = false;
        let mut area = setup(with_sliders);
        let mut stdout = stdout().into_raw_mode().unwrap();
        let result_raw = format!("{}", area.circle().draw().join("\r\n"));
        let result_escaped = format!("{:?}", area.circle().draw().join("\r\n"));
        write!(stdout, "{}\r\n", result_raw).unwrap();
        assert_eq!(result_escaped, data().ring.trim_end());
    }

    #[test]
    #[ignore]
    fn triangle() {
        let with_sliders = false;
        let mut area = setup(with_sliders);
        let mut stdout = stdout().into_raw_mode().unwrap();
        let result_raw = format!("{}", area.triangle().draw().join("\r\n"));
        let result_escaped = format!("{:?}", area.triangle().draw().join("\r\n"));
        write!(stdout, "{}\r\n", result_raw).unwrap();
        assert_eq!(result_escaped, data().triangle.trim_end());
    }

    #[test]
    #[ignore]
    fn sliders() {
        let with_sliders = true;
        let mut area = setup(with_sliders);
        let mut stdout = stdout().into_raw_mode().unwrap();
        let result_raw = format!("{}", area.sliders(vec![Slider::Lightness(None), Slider::Saturation(None), Slider::Preview(None)], 20).draw().join("\r\n"));
        let result_escaped = format!("{:?}", area.sliders(vec![Slider::Lightness(None), Slider::Saturation(None), Slider::Preview(None)], 20).draw().join("\r\n"));
        write!(stdout, "{}\r\n", result_raw).unwrap();
        assert_eq!(result_escaped, data().sliders.trim_end());
    }
}
