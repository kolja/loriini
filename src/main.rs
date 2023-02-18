
use clap::Parser;
use hex::FromHex;
use palette::{FromColor, Hsl, Hsv, Lch, Srgb};

mod model;
use model::Area;

mod draw;
use draw::draw;

mod circle;
use circle::circle;

#[derive(Parser, Debug)]
struct Cli {
    // the number of rows:
    #[arg(short = 's', value_name = "Size", default_value_t = 10)]
    size: usize,

    #[arg(short = 'x', value_name = "factorx (float)", default_value_t = 0.5)]
    factorx: f64,

    #[arg(short = 'r', value_name = "outer radius", default_value_t = 5.0)]
    radius: f64,

    #[arg(short = 'c', value_name = "color input (hex)", default_value_t = String::from("FF0000"))]
    color: String,
}

fn main() {
    let args = Cli::parse();
    let color = match <[u8; 3]>::from_hex(&args.color) {
        Ok([r,g,b]) => Lch::from_color(
            Srgb::from_components(( (r as f32)/255.0, (g as f32)/255.0, (b as f32)/255.0))),
        Err(_) => panic!("failed to decode the color {}", args.color )
    };

    let height = args.size;
    let width = height * 2;

    let radius = args.radius;
    let factorx = args.factorx;

    let grid = vec![vec![None; width]; height];
    let area = Area {
        width,
        height,
        radius,
        factorx,
        color,
        grid,
    };

    draw(circle(area));
}
