use clap::Parser;

mod model;
use model::Area;

mod loriini;
use loriini::Loriini;

mod circle;
mod draw;
mod triangle;
mod sliders;
mod helpers;
mod messages;

mod editmode;
use editmode::{EditMode, Mode};


#[derive(Parser, Debug)]
#[clap(
    author = "Kolja Wilcke",
    version = "0.1.6",
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

    #[arg(short = 'p', long, value_name = "pipe", )]
    pipe: Option<String>,
}

fn main() {

    let args = Cli::parse();
    let color = helpers::hex_to_hsl(&args.color);

    let height = args.size;
    let width = height * 2;

    let radius = args.radius;
    let inner_radius = match args.inner_radius {
        Some(r) => r,
        None => radius * 0.7,
    };
    let factorx = args.factorx;
    let show_info: bool = true;
    let pipe = args.pipe;

    let area = Area {
        width,
        height,
        radius,
        inner_radius,
        factorx,
        color,
        show_info,
        pipe,
        edit_mode: EditMode { modes: vec![Mode::Hue, Mode::Lightness, Mode::Saturation] },
        grid: vec![vec![None; width]; height],
        sliders: Vec::new()
    };

    let mut loriini = Loriini::new(area);
    loriini.keyboard_input();
}
