pub use crate::vector::Vec3;
use std::io::Write;

// Make an alias for Color
pub use Vec3 as Color;

// Write the translated [0, 255] value of each component
pub fn write_color(mut writer: impl Write, color: &Color, num_samples: u16) {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    let scale = 1.0 / f32::from(num_samples);
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    writeln!(
        writer,
        "{} {} {}",
        (r.clamp(0.0, 0.999) * 255.99) as u8,
        (g.clamp(0.0, 0.999) * 255.99) as u8,
        (b.clamp(0.0, 0.999) * 255.99) as u8
    )
    .expect("We should be allowed to write");
}
