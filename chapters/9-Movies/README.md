# Movies

It would be great to dive into the depths of the Mandelbrot set.
Let's create a wrapper around our imaging code that will zoom the length scale of the image by a given factor each frame.

## Edit Args

Edit [`main.rs`](./src/bin/main.rs) the `Args` struct to accommodate a new `rate` argument:

```rust
...
    #[arg(short, long, default_value = "0.99")]
    rate: f64,

    #[arg(short, long, default_value = "100")]
    frames: u16,
...
```

`rate` will be the factor by which we'll zoom the image each frame.
`frames` will be maximum number of frames we'll generate.

## Edit main

Edit the `main` function in [`main.rs`](./src/bin/main.rs):

```rust
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
```

## Try it

Add the new `rate` and `frames` arguments when you call the program:

```shell
cargo run --bin main --release -- --real 0.3990010000010399 --imag 0.2350015000010228 --scale 1.0e-1 --max-iters 400 --ss-power 2 --width 540 --height 540 --rate 0.97 --frames 1000 --cmap 062B79 16498A 5995B7 FAFBBD FDE050 F1B351 FFBB00 FFFFFF
```

You can then stitch the frames together with the `convert` tool:

```shell
convert -delay 2 -loop 0 *.png output.mov
```

or use `ffmpeg`:

```shell
ffmpeg -framerate 30 -i output/mandy_%06d.png -c:v libx264 -profile:v high -crf 20 -pix_fmt yuv420p output/mandy.mov
```

## Return

[Return to the top-level README](./../../README.md)
