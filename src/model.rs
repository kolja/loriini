
pub struct Area {
    pub width: usize,
    pub height: usize,
    pub radius: f64,
    pub inner_radius: f64,
    pub factorx: f64,
    pub color: palette::Hsl,
    pub grid: Vec<Vec<Option<palette::Hsl>>>,
}

#[allow(dead_code)] // allow for different kinds of color sliders
pub enum Bar {
    // Red,
    // Green,
    // Blue,
    Hue,
    Lightness,
    Saturation,
    Alpha,
    Preview
}

pub enum EditMode {
    Hue,
    Alpha,
    Saturation,
    Lightness
}
