use glob::glob;
use image::imageops::thumbnail;
use image::io::Reader as ImageReader;
use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    for entry in glob(&args[1])? {
        match entry {
            Ok(path) => {
                if path.is_file() {
                    let img = ImageReader::open(path.clone())?.decode()?;
                    let resize_img = thumbnail(&img, 200, 200);
                    resize_img.save(path)?;
                }
            }
            Err(e) => panic!("path error {}", e),
        }
    }
    Ok(())
}
