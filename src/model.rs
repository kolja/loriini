
pub struct Area {
    pub width: usize,
    pub height: usize,
    pub radius: f64,
    pub inner_radius: f64,
    pub factorx: f64,
    pub color: palette::Hsl,
    pub grid: Vec<Vec<Option<palette::Hsl>>>,
    pub show_info: bool,
    pub sliders: Vec<Slider>
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

pub enum EditMode {
    Hue,
    Alpha,
    Saturation,
    Lightness
}
