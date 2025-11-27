use image::ImageReader;

pub fn run(input: &str, output: &str) -> std::io::Result<()> {
    println!("🖥️ CLI mode active.");
    println!("Reading: {}", input);

    // 1. Open
    let img = ImageReader::open(input)
        .map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Unable to open {}: {}", input, e),
            )
        })?
        .with_guessed_format()
        .map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unknown format: {}", e),
            )
        })?
        .decode()
        .map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Decoding error: {}", e),
            )
        })?;

    println!("Image loaded. Converting to: {}", output);

    // 2. Save (the library infers the format from the output file extension)
    img.save(output).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, format!("Save error: {}", e))
    })?;

    println!("✅ Success! Image saved to {}", output);
    Ok(())
}
