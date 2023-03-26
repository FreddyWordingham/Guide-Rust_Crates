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

    #[arg(short, long, default_value = "1")]
    ss_power: u8,

    #[arg(short, long, default_value = "1920")]
    width: usize,

    #[arg(short, long, default_value = "1080")]
    height: usize,

    #[arg(short, long, default_value = "0.99")]
    rate: f64,

    #[arg(short, long, default_value = "100")]
    frames: u16,

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

    let mut scale = args.scale;
    let padding = ((args.frames - 1) as f64).log10() as usize + 1;
    for n in 0..args.frames {
        let data = sample::area(
            args.real,
            args.imag,
            scale,
            [args.width, args.height],
            args.max_iters,
            args.ss_power,
        );
        let img = colour::image(data, &cmap, args.max_iters);
        colour::encode(&img)
            .save(output_dir.join(format!("mandy_{:0padding$}.png", n, padding = padding)))
            .unwrap();
        scale *= args.rate;
    }
}
