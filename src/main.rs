use image::ImageFormat;
use image::ImageReader;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Ok(());
    }
    let input_path = &args[1];
    let output_path = &args[2];
    let img_reader = ImageReader::open(input_path)?.decode()?;
    let output_format = ImageFormat::from_path(output_path)?;
    img_reader.save_with_format(output_path, output_format)?;
    Ok(())
}
