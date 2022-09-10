// https://raytracing.github.io/books/RayTracingInOneWeekend.html

use std::io::prelude::*; // TODO: Figure out which actual imports we use from this,
use std::io::BufWriter;

// Get the Color struct and function for printing colors,
use renderer::vector::*;
use renderer::color::*;
use renderer::ray::*;

use Vec3 as Point3;

fn ray_color(ray: &Ray) -> Color {
    let t = 0.5 * (ray.direction.y + 1.0);
    return (1.0 - t) * Color{x: 1.0, y: 1.0, z: 1.0} + t * Color{x: 0.5, y: 0.7, z: 1.0};
}

fn main() -> std::io::Result<()> {
    let stdout = std::io::stdout();
    let mut buffer = BufWriter::new(stdout.lock());

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let width: u16 = 400;
    let height: u16 = (width as f32 / aspect_ratio) as u16;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let origin = Point3{x: 0.0, y: 0.0, z: 0.0};
    let horizontal = Vec3{x: viewport_width, y: 0.0, z: 0.0};
    let vertical = Vec3{x: 0.0, y: viewport_height, z: 0.0};
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3{x: 0.0, y: 0.0, z: focal_length};

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
    for j in (0..height).rev() {
        // Output progress for scanlines, to give us feedback in case the
        // render freezes...
        let buf = format!("\rScanlines remaining: {j}");
        buffer_size = buf.len() + 1;
        eprint!("{:buffer_size$}", buf);

        for i in 0..width {
            let u = i as f32 / (width as f32 - 1.0);
            let v = j as f32 / (height as f32 - 1.0);

            let ray = Ray{origin, direction: lower_left_corner + u*horizontal + v*vertical - origin};
            let pixel_color = ray_color(&ray);
            write_color(buffer.get_mut(), &pixel_color);
        }
    }

    eprint!("\nRender Finished\n");

    Ok(())
}
