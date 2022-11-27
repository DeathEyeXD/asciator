use colored::*;
use image::{GenericImageView, Pixel};
use std::error::Error;
use std::io::{self, Write};
use std::path::PathBuf;

use clap::Parser;
fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct CliArgs {
    /// Path to the image
    #[arg(value_hint = clap::ValueHint::FilePath)]
    path: PathBuf,

    /// Scale the image to this width
    /// Default is 120px to have consistent output
    #[arg(long, group = "scale", default_value = "120")]
    scale_px: u32,

    /// Scale the image by this factor
    /// Optional argument instead of scale_px
    #[arg(long, group = "scale")]
    scale: Option<f64>,

    /// Print the resulted ASCII art in color
    /// Note: this will only work if the terminal supports ANSI colors
    #[arg(long, short)]
    colorize: bool,

    /// Controls how bright pixels have to be to be converted to ascii
    /// Lower values mean even dark pixels will be converted to ascii
    #[arg(short, long, default_value = "1")]
    brightness_threshold: u32,
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::parse();
    let mut img = image::open(args.path)?;
    Ok(())
}
// const GRAYSCALE_CHARS: &str = " `.,^\":;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
// const GRAYSCALE_CHARS: &str = "  .,;!vlLFE$";
const DENSITY_CHARS: [char; 13] = [
    ' ', ' ', ' ', '.', ',', ';', '!', 'v', 'l', 'L', 'F', 'E', '$',
];
// const DENSITY_CHARS: [char; 8] = [' ', ' ', '.', ':', '░', '▒', '▓', '█'];
// const GRAYSCALE_CHARS: &str = r#"@MBHENR#KWXDFPQASUZbdehx*8Gm&04LOVYkpq5Tagns69owz$CIu23Jcfry%1v7l+it[] {}?j|()=~!-/<>\"^_';,:`. "#;

fn convert_pixel(brightness: u8) -> char {
    let len = DENSITY_CHARS.len();

    let index = (brightness as f64 / 255.0 * (len - 1) as f64).round() as usize;
    DENSITY_CHARS[index]
}

fn scale_width_px(img: &mut image::DynamicImage, new_width: u32) {
    let (width, height) = img.dimensions();
    let scale = new_width as f64 / width as f64;
    let new_height = (height as f64 * scale).round();

    *img = img.resize(
        new_width,
        new_height as u32,
        image::imageops::FilterType::Nearest,
    )
}

fn scale_down(img: &mut image::DynamicImage, scale: f64) {
    let (width, height) = img.dimensions();
    let new_width = (width as f64 * scale).round();
    let new_height = (height as f64 * scale).round();

    *img = img.resize(
        new_width as u32,
        new_height as u32,
        image::imageops::FilterType::Nearest,
    )
}

fn print_img_as_ascii_colorized(img: image::DynamicImage) {
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    for (x, _, rgb) in img.into_rgb8().enumerate_pixels() {
        let brightness = rgb.to_luma()[0];
        if x == 0 {
            handle.write_all(b"\n").unwrap();
        }
        let char = convert_pixel(brightness);
        handle
            .write_all(
                format!("{}{}", char, char)
                    .truecolor(rgb[0], rgb[1], rgb[2])
                    .to_string()
                    .as_bytes(),
            )
            .unwrap();
    }
}

fn print_img_as_ascii(img: image::DynamicImage) {
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    for (x, _, rgb) in img.pixels() {
        let brightness = rgb.to_luma()[0];
        if x == 0 {
            handle.write_all(b"\n").unwrap();
        }
        let char = convert_pixel(brightness) as u8;
        handle.write_all(&[char, char]).unwrap();
    }
}
