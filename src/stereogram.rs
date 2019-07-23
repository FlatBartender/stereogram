use image::{GrayImage, Pixel, GenericImageView};
use rand::prelude::*;
use derive_builder::Builder;

#[derive(Builder)]
#[builder(build_fn(skip))]
pub struct StereogramBuilder {
    amplitude: f64,
    pattern_width: u32,
    pattern_height: u32,
    cross_eyed: bool,
    margins: bool,
}

impl Default for StereogramBuilder {
    fn default() -> StereogramBuilder {
        StereogramBuilder {
            amplitude: 20.0,
            pattern_width: 128,
            pattern_height: 128,
            cross_eyed: false,
            margins: true,
        }
    }
}

impl StereogramBuilder {
    pub fn new() -> StereogramBuilder {
        Self::default()
    }

    pub fn render(&self, depth_map: &GrayImage) -> GrayImage {
        let mut rng = thread_rng();

        let margins_shift = if self.margins {
            self.pattern_width
        } else {
            0
        };
        let (width, height) = depth_map.dimensions();
        let width = width + margins_shift * 2;
        let mut stereogram = GrayImage::new(width, height);
        let mut pattern = GrayImage::new(self.pattern_width, self.pattern_height);
        
        pattern.pixels_mut().for_each(|pixel| pixel[0] = rng.gen());
        
        for y in 0..height {
            for x in 0..width {
                let pixel = if x < self.pattern_width {
                    pattern.get_pixel(x , y % self.pattern_height)
                } else {
                    let depth = if depth_map.in_bounds(x - margins_shift, y) {
                        depth_map.get_pixel(x - margins_shift, y).channels()[0] as f64 / 255.0
                    } else {
                        0.0
                    };
                    let depth = if self.cross_eyed {
                        1.0 - depth
                    } else {
                        depth
                    };
                    let shift = (depth * self.amplitude) as u32;
                    stereogram.get_pixel(((x + shift) as i32 - self.pattern_width as i32) as u32, y)
                }.clone();

                stereogram.put_pixel(x, y, pixel);
            }
        }

        stereogram
    }
}
