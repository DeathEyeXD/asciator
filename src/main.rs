use colored::*;
use image::{GenericImageView, Pixel};
use std::error::Error;
use std::io::{self, Write};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: asciator <image> <1 if you want to invert>(optional)");
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
    // scale_down(&mut img);
    if invert {
        img.invert();
    }
    // let rows = map_image(img);
    // for row in rows {
    //     // println!("{}", row.truecolor(14, 181, 59));
    //     println!("{}", row);
    // }
    //
    map_image(img);

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

fn scale_down(img: &mut image::DynamicImage) {
    let (width, height) = img.dimensions();
    let scale = 120.0 / height as f64;
    let new_width = (width as f64 * scale).round();
    let new_height = (height as f64 * scale).round();

    *img = img.resize(
        new_width as u32,
        new_height as u32,
        image::imageops::FilterType::Nearest,
    )
}

fn map_image(img: image::DynamicImage) {
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    for (x, _, rgb) in img.into_rgb8().enumerate_pixels() {
        let brightness = rgb.to_luma()[0];
        if x == 0 {
            handle.write_all(b"\n").unwrap();
        }
        let char = convert_pixel(brightness);
        // print!("{}{}", char, char);
        // if char == ' ' {
        //     handle
        //         .write_all("@@".on_truecolor(rgb[0], rgb[1], rgb[2]).as_bytes())
        //         .unwrap();
        // } else {
        handle
            .write_all(
                format!("{}{}", char, char)
                    .truecolor(rgb[0], rgb[1], rgb[2])
                    .to_string()
                    .as_bytes(),
            )
            .unwrap();
        // }
    }
}
