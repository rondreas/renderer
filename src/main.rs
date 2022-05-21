// https://raytracing.github.io/books/RayTracingInOneWeekend.html

use std::fs::File;
use std::io::prelude::*; // TODO: Figure out which actual imports we use from this,

use std::ops::{Neg, AddAssign, MulAssign, DivAssign};

// Seems we can 'derive' and get some traits for free,
// Debug here is used for printing the Vec3 in formatting
// and PartialEq for when we make asserts_eq in tests.
#[derive(Debug, PartialEq)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}

// Implement operator traits,
impl Neg for Vec3 {
    type Output = Self; // TODO: figure out this standard,

    fn neg(self) -> Self {
        return Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        };
    }
}

// Overloading implementations for different types, here we impl for both Vec3 and f32
impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl AddAssign<f32> for Vec3 {
    fn add_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        };
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        };
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        };
    }
}

trait Dot {
    fn dot(&self, other: &Self) -> f32;
}

impl Dot for Vec3 {
    fn dot(&self, other: &Self) -> f32 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }
}

// Can't find how to extend .len() so doing this,
trait Length {
    fn length(&self) -> f32;
}

impl Length for Vec3 {
    fn length(&self) -> f32 {
        return self.dot(self).sqrt();
    }
}

fn main() -> std::io::Result<()> {
    // File::create will return a Result, if it raises an error, the error
    // will be returned by the whole function. So guess we will exit main
    // and return the error if that happens.
    let mut file = File::create("image.ppm")?;

    // Define size,
    let width: u16 = 256;
    let height: u16 = 256;

    // Create a variable to store the buffer size when printing to standard
    // error. Macro `eprint!` doesn't return num bytes so we will get the
    // length of a string to 'hack' how C would use:
    // `std::cerr << "\rfoo" << std::flush;`
    let mut buffer_size = 0;
    
    // Fill content with header from formatted string.
    let mut contents = format!("P3\n{width} {height}\n255\n");

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

            // Get formatted string to represent the color for this pixel
            let colors = format!("{red} {green} {blue}\n");
            contents.push_str(&colors);
        }
    }

    // Write contents to file,
    write!(file, "{}", contents)?;

    eprint!("\nRender Finished\n");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_neg() {
        let a = Vec3{x: 1.0, y: 0.0, z: -3.0};
        assert_eq!(-a, Vec3{x: -1.0, y: -0.0, z: 3.0});
    }

    #[test]
    fn test_vec3_add_assign_vec3() {
        let mut a = Vec3{x:1.0, y:2.0, z:3.0};
        let b = Vec3{x:1.0, y:1.0, z:1.0};

        a += b;

        assert_eq!(a,Vec3{x: 2.0, y: 3.0, z: 4.0});
    }

    #[test]
    fn test_vec3_add_assign_f32() {
        let mut a = Vec3{x:1.0, y:2.0, z:3.0};

        a += 1.0;

        assert_eq!(a,Vec3{x: 2.0, y: 3.0, z: 4.0});
    }

    #[test]
    fn test_vec3_mul_assign() {
        let mut a = Vec3{x: 1.0, y: 2.0, z: 3.0};
        let b = Vec3{x: 1.0, y: 2.0, z: 0.5};

        a *= b;

        assert_eq!(a, Vec3{x: 1.0, y: 4.0, z: 1.5});
    }

    #[test]
    fn test_vec3_div_assign() {
        let mut a = Vec3{x: 1.0, y: 2.0, z: 3.0};
        let b = Vec3{x: 2.0, y: 3.0, z: 0.8};

        a /= b;

        assert_eq!(a,Vec3{x: 0.5, y: 0.6666667, z: 3.75});
    }

    #[test]
    fn test_vec3_dot() {
        let a = Vec3{x: 1.0, y: 2.0, z: 3.0};
        let b = Vec3{x: 2.0, y: 3.0, z: 4.0};

        let dot_product = a.dot(&b);

        assert_eq!(dot_product, 20.0);
    }
}