use std::error::Error;

use colored::*;
use image::GenericImageView;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: asciator <image> <invert>(optional)");
        return;
    }
    let path = &args[1];
    let invert = args.len() > 2 && args[2] == "1";

    if let Err(error) = run(path, invert) {
        eprintln!("Error: {}", error);
    }
}

fn run(path: &str, invert: bool) -> Result<(), Box<dyn Error>> {
    let mut img = image::open(path)?;
    scale_down(&mut img);
    if invert {
        img.invert();
    }
    let rows = map_image(img);
    for row in rows {
        // println!("{}", row.truecolor(14, 181, 59));
        println!("{}", row);
    }

    Ok(())
}
const CHAR_MATRIX: &str = "`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

fn convert_pixel(brightness: u8) -> char {
    let len = CHAR_MATRIX.len();

    let index = (brightness as f32 / 255.0 * (len - 1) as f32) as usize;
    return CHAR_MATRIX.chars().nth(index).unwrap();
}

fn scale_down(img: &mut image::DynamicImage) {
    let (width, height) = img.dimensions();
    let scale = 100.0 / width as f64;
    let new_width = (width as f64 * scale) as u32;
    let new_height = (height as f64 * scale) as u32;

    *img = img.resize(new_width, new_height, image::imageops::FilterType::Nearest)
}

fn map_image(mut img: image::DynamicImage) -> Vec<String> {
    let mut rows = vec!["".to_string(); img.height() as usize];
    for (_, y, brightness) in img.into_luma8().enumerate_pixels() {
        rows[y as usize].push(convert_pixel(brightness[0]));
        rows[y as usize].push(convert_pixel(brightness[0]));
    }

    rows
}
