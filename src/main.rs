extern crate rand;
extern crate image;

pub mod perlin;

use std::fs::File;
use std::path::Path;

fn main() {
    let width = 512;
    let height = 512;

    let mut buf = image::ImageBuffer::new(width, height);
    let height_map = perlin::random_height_map(width as usize, height as usize);

    for (x, y, pixel) in buf.enumerate_pixels_mut() {
        let height = height_map[y as usize][x as usize];
        let normalised = (height * 256.0) as u8;

        let water = image::Rgb([163,204,255]);
        let land = image::Rgb([204,232,164]);
        let sand = image::Rgb([255,242,186]);
        let mountain = image::Rgb([235,219,200]);

        if normalised < 110 {
            *pixel = water;
        } else if normalised < 115 {
            *pixel = sand;
        } else if normalised > 180 {
            *pixel = mountain;
        } else {
            *pixel = land;
        }
    }

    let ref mut fout = File::create(&Path::new("out.png")).unwrap();
    let _ = image::ImageRgb8(buf).save(fout, image::PNG);
}

