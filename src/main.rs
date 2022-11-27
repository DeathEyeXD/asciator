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

fn run() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::parse();
    let asciator = Asciator::from_args(args)?;
    asciator.run();
    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct CliArgs {
    /// Path to the image
    #[arg(value_hint = clap::ValueHint::FilePath)]
    path: PathBuf,

    /// Scale the image to this width.
    /// Cannot be used with scale
    #[arg(long, default_value = "100", group = "_scale")]
    scale_px: u32,

    /// Scale the image by this factor.
    /// Cannot be used with scale-px
    #[arg(long, group = "_scale")]
    scale: Option<f64>,

    /// Print the resulted ASCII art in color.
    /// Note: this will only work if the terminal supports ANSI colors
    #[arg(long, short)]
    colorize: bool,

    /// Controls how bright pixels have to be to be converted to ascii
    /// Lower values mean even dark pixels will be converted to ascii
    #[arg(short, long, default_value = "1")]
    brightness_threshold: usize,
}

struct Asciator {
    scale_px: u32,
    scale: Option<f64>,
    colorize: bool,
    density_map: Vec<char>,
    image: image::DynamicImage,
}

impl Asciator {
    const DEFAULT_DENSITY_MAP: [char; 10] = ['.', ',', ';', '!', 'v', 'l', 'L', 'F', 'E', '$'];
    fn from_args(args: CliArgs) -> Result<Self, Box<dyn Error>> {
        let mut density_map = vec![' '; args.brightness_threshold];
        density_map.extend(Self::DEFAULT_DENSITY_MAP);

        Ok(Self {
            scale_px: args.scale_px,
            scale: args.scale,
            colorize: args.colorize,
            image: image::open(&args.path)?,
            density_map,
        })
    }

    pub fn run(mut self) {
        self.scale_image();
        self.print_ascii_art();
    }

    fn scale_image(&mut self) {
        let (width, height) = self.image.dimensions();
        let (new_width, new_height) = match self.scale {
            Some(scale) => {
                let new_width = (width as f64 * scale) as u32;
                let new_height = (height as f64 * scale) as u32;
                (new_width, new_height)
            }
            None => {
                let scale = self.scale_px as f64 / width as f64;
                let new_height = (height as f64 * scale) as u32;
                (self.scale_px, new_height)
            }
        };

        self.image = self
            .image
            .resize(new_width, new_height, image::imageops::FilterType::Nearest)
    }

    fn convert_pixel(&self, brightness: u8) -> char {
        let index = (brightness as f64 / 255.0 * (self.density_map.len() - 1) as f64) as usize;
        self.density_map[index]
    }

    fn print_ascii_art(self) {
        let mut stdout = io::stdout().lock();

        for (x, _, rgb) in self.image.pixels() {
            if x == 0 {
                writeln!(stdout).unwrap();
            }
            let brightness = rgb.to_luma().0[0];
            let c = self.convert_pixel(brightness);
            if c != ' ' && self.colorize {
                // let color = format!("\x1b[38;2;{};{};{}m", rgb[0], rgb[1], rgb[2]);
                write!(
                    stdout,
                    "\x1b[38;2;{};{};{}m{}{}",
                    rgb[0], rgb[1], rgb[2], c, c
                )
                .unwrap();
                // write!(
                //     stdout,
                //     "{}",
                //     format!("{}{}", c, c).truecolor(rgb[0] as u8, rgb[1] as u8, rgb[2] as u8)
                // )
                // .unwrap();
            } else {
                write!(stdout, "{}{}", c, c).unwrap();
            }
        }

        writeln!(stdout).unwrap();
    }
}
// const GRAYSCALE_CHARS: &str = " `.,^\":;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
// const GRAYSCALE_CHARS: &str = "  .,;!vlLFE$";
// const DENSITY_CHARS: [char; 13] = [
//     ' ', ' ', ' ', '.', ',', ';', '!', 'v', 'l', 'L', 'F', 'E', '$',
// ];
// // const DENSITY_CHARS: [char; 8] = [' ', ' ', '.', ':', '░', '▒', '▓', '█'];
// // const GRAYSCALE_CHARS: &str = r#"@MBHENR#KWXDFPQASUZbdehx*8Gm&04LOVYkpq5Tagns69owz$CIu23Jcfry%1v7l+it[] {}?j|()=~!-/<>\"^_';,:`. "#;
