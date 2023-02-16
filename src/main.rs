
use clap::Parser;

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

    #[arg(short = 'o', value_name = "color offset", default_value_t = 0)]
    offset: i16,
}

fn main() {
    let args = Cli::parse();

    let height = args.size;
    let width = height * 2;

    let radius = args.radius;
    let factorx = args.factorx;
    let offset = args.offset;

    let grid = vec![vec![0; width]; height];
    let area = Area {
        width,
        height,
        radius,
        factorx,
        offset,
        grid,
    };

    draw(circle(area));

}
