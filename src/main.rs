use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // Create the image file,
    let mut file = File::create("image.ppm")?;

    // Define size,
    let width: u16 = 256;
    let height: u16 = 256;
    
    // Fill content with header,
    let mut contents = format!("P3\n{width} {height}\n255\n");    

    for u in 0..height {
        for v in 0..width {
            // Get colors, range 0 .. 255
            let red: u8 = ((u as f32 / (width as f32 - 1.0)) * 255.99) as u8;
            let green: u8 = ((v as f32 / (height as f32 - 1.0)) * 255.99) as u8;
            let blue: u8 = (0.25 * 255.99) as u8;

            // Get formatted string to represent the color for this pixel
            let colors = format!("{red} {green} {blue}\n");
            contents.push_str(&colors);
        }
    }

    // Write contents to file,
    write!(file, "{}", contents);

    Ok(())
}
