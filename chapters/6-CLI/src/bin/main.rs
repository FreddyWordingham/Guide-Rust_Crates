use clap::Parser;
use std::{fs::create_dir, path::Path};

use mandy::{colour, sample};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    real: f64,

    #[arg(short, long)]
    imag: f64,

    #[arg(short, long)]
    scale: f64,

    #[arg(short, long, default_value = "100")]
    max_iters: u16,

    #[arg(short, long, default_value = "1920")]
    width: usize,

    #[arg(short, long, default_value = "1080")]
    height: usize,

    #[clap(short, long, value_parser, num_args = 2.., value_delimiter = ' ')]
    cmap: Vec<String>,
}

/// Entry point function.
fn main() {
    let args = Args::parse();
    let cmap: Vec<&str> = args.cmap.iter().map(|s| s.as_str()).collect();

    let output_dir = Path::new("output");
    if !output_dir.exists() {
        create_dir("output").unwrap();
    }

    let data = sample::area(
        args.real,
        args.imag,
        args.scale,
        [args.width, args.height],
        args.max_iters,
    );
    let img = colour::image(data, cmap, args.max_iters);
    colour::encode(&img)
        .save(output_dir.join("mandy.png"))
        .unwrap();
}
