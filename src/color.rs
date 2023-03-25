pub use crate::vector::Vec3;
use std::io::Write;

// Make an alias for Color
// TODO: Would be nice exposing xyz as rgb
pub use Vec3 as Color;

// Write the translated [0, 255] value of each component
pub fn write_color(mut writer: impl Write, color: &Color) {
    writeln!(
        writer,
        "{} {} {}",
        (color.x * 255.99) as u8,
        (color.y * 255.99) as u8,
        (color.z * 255.99) as u8
    )
    .expect("We should be allowed to write");
}
