// https://raytracing.github.io/books/RayTracingInOneWeekend.html
// To render an image, run
// cargo run > image.ppm

use rand::Rng;
use std::io::prelude::*;
use std::io::BufWriter;

use renderer::camera::Camera;
use renderer::color::*;
use renderer::hittable::*;
use renderer::hittable_list::*;
use renderer::ray::*;
use renderer::sphere::*;
use renderer::vector::*;

fn ray_color(ray: &Ray, world: &HittableList) -> Color {
    if let Some(hit) = world.hit(ray, 0.0, f32::INFINITY) {
        return 0.5 * (hit.normal + 1.0);
    }
    let direction = unit_vector(&ray.direction);
    let t = 0.5 * (direction.y + 1.0);
    (1.0 - t)
        * Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
        + t * Color {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        }
}

fn main() -> std::io::Result<()> {
    let stdout = std::io::stdout();
    let mut buffer = BufWriter::new(stdout.lock());

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let width: u16 = 400;
    let height: u16 = (width as f32 / aspect_ratio) as u16;
    let samples_per_pixel = 100;

    // World
    let mut world = HittableList { objects: vec![] };
    world.add(Box::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
    }));
    world.add(Box::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
    }));

    // Camera
    let cam = Camera::new();

    // Create a variable to store the buffer size when printing to standard
    // error. Macro `eprint!` doesn't return num bytes so we will get the
    // length of a string to 'hack' how C would use:
    // `std::cerr << "\rfoo" << std::flush;`
    let mut buffer_size;

    // Write out the ppm header to stdout
    write!(buffer, "P3\n{width} {height}\n255\n").expect("Failed to write header...");

    buffer.flush().unwrap();

    let mut rng = rand::thread_rng();

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
            let mut color = Color::zero();

            for _s in 0..samples_per_pixel {
                let u = (i as f32 + rng.gen::<f32>()) / (width as f32 - 1.0);
                let v = (j as f32 + rng.gen::<f32>()) / (height as f32 - 1.0);
                let ray = cam.get_ray(u, v);
                color += ray_color(&ray, &world);
            }
            write_color(buffer.get_mut(), &color, samples_per_pixel);
        }
    }

    eprint!("\nRender Finished\n");

    buffer.flush().unwrap();

    Ok(())
}
