
pub struct Area {
    pub width: usize,
    pub height: usize,
    pub radius: f64,
    pub factorx: f64,
    pub color: palette::Lch,
    pub grid: Vec<Vec<Option<palette::Lch>>>,
}
