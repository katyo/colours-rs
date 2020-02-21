/*!
# Generic color manipulation library

This crate implements widely used color types of different color models with conversions between it.

Each type can be parametrized by scalar type which will be used as channel or color component.
 */

mod color;
mod rgb;
mod rgba;
mod hsl;
mod hsla;
mod hsv;
mod hsva;

pub use self::color::*;
pub use self::rgb::Rgb;
pub use self::rgba::Rgba;
pub use self::hsl::Hsl;
pub use self::hsla::Hsla;
pub use self::hsv::Hsv;
pub use self::hsva::Hsva;
