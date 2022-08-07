// https://raytracing.github.io/books/RayTracingInOneWeekend.html

use std::io::prelude::*; // TODO: Figure out which actual imports we use from this,
use std::io::BufWriter;

fn main() -> std::io::Result<()> {
    let stdout = std::io::stdout();
    let mut buffer = BufWriter::new(stdout.lock());

    // Define size of resulting image,
    let width: u16 = 256;
    let height: u16 = 256;

    // Create a variable to store the buffer size when printing to standard
    // error. Macro `eprint!` doesn't return num bytes so we will get the
    // length of a string to 'hack' how C would use:
    // `std::cerr << "\rfoo" << std::flush;`
    let mut buffer_size;
    
    // Write out the ppm header to stdout
    write!(buffer, "P3\n{width} {height}\n255\n");

    // The render loop, we will iterate over the image from top to bottom,
    // then from left to right along the pixel row, row will be called a
    // "scanline" from now,
    for u in (0..height).rev() {
        // Output progress for scanlines, to give us feedback in case the
        // render freezes...
        let buf = format!("\rScanlines remaining: {u}");
        buffer_size = buf.len() + 1;
        eprint!("{:buffer_size$}", buf);

        for v in 0..width {
            // Get colors, range 0 .. 255
            let red: u8 = ((v as f32 / (width as f32 - 1.0)) * 255.99) as u8;
            let green: u8 = ((u as f32 / (height as f32 - 1.0)) * 255.99) as u8;
            let blue: u8 = (0.25 * 255.99) as u8;

            write!(buffer, "{red} {green} {blue}\n");
        }
    }

    eprint!("\nRender Finished\n");

    Ok(())
}
