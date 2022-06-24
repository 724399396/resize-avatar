use image::imageops::{crop, thumbnail};
use image::io::Reader as ImageReader;
use std::cmp;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut img = ImageReader::open(&args[1])?.decode()?;
    let width = img.width();
    let height = img.height();
    let min_edge = cmp::min(width, height);
    if width > height {
        let resize_img = crop(&mut img, (width - min_edge) / 2, 0, min_edge, min_edge);
        let resize_img = thumbnail(&resize_img.to_image(), 200, 200);
        resize_img.save(&args[1])?;
    } else {
        let resize_img = crop(&mut img, 0, 0, min_edge, min_edge);
        let resize_img = thumbnail(&resize_img.to_image(), 200, 200);
        resize_img.save(&args[1])?;
    }
    Ok(())
}
