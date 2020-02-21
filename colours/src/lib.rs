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

/// Color types aliases parametrized with `u8` type
pub mod byte {
    type Channel = u8;

    /// 24-bit RGB color (8-bits per channel)
    pub type Rgb = super::Rgb<Channel>;

    /// 32-bit RGB color with alpha (8-bits per channel)
    pub type Rgba = super::Rgba<Channel>;

    /// 24-bit HSL color (8-bits per channel)
    pub type Hls = super::Hsl<Channel>;

    /// 32-bit HSL color with alpha (8-bits per channel)
    pub type Hlsa = super::Hsla<Channel>;

    /// 24-bit HSV color (8-bits per channel)
    pub type Hlv = super::Hsv<Channel>;

    /// 32-bit HSV color with alpha (8-bits per channel)
    pub type Hlva = super::Hsva<Channel>;
}

/// Color types aliases parametrized with `f32` type
pub mod float {
    type Channel = f32;

    /// 32x3-bit RGB color (32-bits per channel)
    pub type Rgb = super::Rgb<Channel>;

    /// 32x4-bit RGB color with alpha (32-bits per channel)
    pub type Rgba = super::Rgba<Channel>;

    /// 32x3-bit HSL color (32-bits per channel)
    pub type Hls = super::Hsl<Channel>;

    /// 32x4-bit HSL color with alpha (32-bits per channel)
    pub type Hlsa = super::Hsla<Channel>;

    /// 32x3-bit HSV color (32-bits per channel)
    pub type Hlv = super::Hsv<Channel>;

    /// 32x4-bit HSV color with alpha (32-bits per channel)
    pub type Hlva = super::Hsva<Channel>;
}
