use clap::Parser;

mod model;
use model::Area;

mod loriini;
use loriini::Loriini;

mod circle;
mod clipboard;
mod draw;
mod triangle;
mod sliders;
mod helpers;
mod messages;

mod editmode;
use editmode::{EditMode, Mode};


const DEFAULT_RADIUS: f64 = 6.0;


#[derive(Parser, Debug)]
#[clap(
    author = "Kolja Wilcke",
    version = env!("CARGO_PKG_VERSION"),
    about = "A console color picker"
)]
struct Cli {
    #[arg(short = 's', value_name = "Size")]
    size: Option<usize>,

    #[arg(short = 'x', value_name = "factorx (float)", default_value_t = 0.5)]
    factorx: f64,

    #[arg(short = 'r', value_name = "outer radius")]
    radius: Option<f64>,

    #[arg(short = 'i', long, value_name = "inner radius")]
    inner_radius: Option<f64>,

    #[arg(short = 'c', value_name = "color input (hex)", default_value_t = String::from("FF0000"))]
    color: String,

    #[arg(short = 'p', long, value_name = "pipe", )]
    pipe: Option<String>,
}

fn dims(size: Option<usize>, radius: Option<f64>, factorx: f64) -> (usize, usize, f64) {
    let radius = radius.unwrap_or_else(|| match size {
        Some(s) => s as f64 / 2.0,
        None => DEFAULT_RADIUS,
    });
    let height = size.unwrap_or_else(|| (2.0 * radius).ceil() as usize);
    let width = (height as f64 / factorx).ceil() as usize;
    (width, height, radius)
}

fn main() {

    let args = Cli::parse();
    let color = helpers::hex_to_hsl(&args.color);

    let (width, height, radius) = dims(args.size, args.radius, args.factorx);
    let inner_radius = args.inner_radius.unwrap_or(radius * 0.7);

    let area = Area {
        width,
        height,
        radius,
        inner_radius,
        factorx: args.factorx,
        color,
        show_info: true,
        pipe: args.pipe,
        edit_mode: EditMode { modes: vec![Mode::Hue, Mode::Lightness, Mode::Saturation] },
        grid: vec![vec![None; width]; height],
        sliders: Vec::new()
    };

    let mut loriini = Loriini::new(area);
    loriini.keyboard_input();
}
