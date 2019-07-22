extern crate structopt;
extern crate image;
extern crate rand;
extern crate rand_pcg;

use std::path::Path;
use structopt::StructOpt;
use image::{GrayImage, ImageResult, Pixel};
use rand::prelude::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct CliArgs {
    #[structopt(name = "FILE")]
    input: String,
    #[structopt(short, long, default_value = "out.png")]
    output: String,
    #[structopt(short, long, default_value = "20")]
    amplitude: u32,
    #[structopt(short = "w", long, default_value = "128")]
    pattern_width: u32,
    #[structopt(short = "h", long, default_value = "128")]
    pattern_height: u32,
    #[structopt(short, long)]
    cross_eyed: bool,
}

fn load_depth_map(path: &Path) -> ImageResult<GrayImage> {
    image::open(path).map(|image| image.to_luma())
}

fn interp(value: u8, args: &CliArgs) -> u32 {
    (if args.cross_eyed {
        255 - value
    } else {
        value
    } as f64 / 255.0 * args.amplitude as f64) as u32
}

fn main() {
    let master_seed: u64 = random();
    let args = CliArgs::from_args();
    let depth_map = load_depth_map(Path::new(&args.input)).unwrap();
    let (width, height) = depth_map.dimensions();
    let mut stereogram = GrayImage::new(width + args.pattern_width * 2, height);
    let mut pattern = GrayImage::new(args.pattern_width, args.pattern_height);

    for (x, y, pixel) in pattern.enumerate_pixels_mut() {
        let mut rng = rand_pcg::Pcg32::seed_from_u64((master_seed as u32 + y * width + x) as u64);
        pixel[0] = rng.next_u32() as u8;
    }
    
    for y in 0..height {
        for x in 0..(width + args.pattern_width * 2) {
            let pixel = if x < args.pattern_width {
                pattern.get_pixel(x % args.pattern_width, y % args.pattern_height)
            } else if x < width + args.pattern_width {
                let shift = interp(depth_map.get_pixel(x - args.pattern_width, y).channels()[0], &args);
                stereogram.get_pixel(((x + shift) as i32 - args.pattern_width as i32) as u32, y)
            } else {
                stereogram.get_pixel((x as i32 - args.pattern_width as i32) as u32, y)
            }.clone();
            stereogram.put_pixel(x, y, pixel);
        }
    }

    stereogram.save(Path::new(&args.output)).unwrap();
}
