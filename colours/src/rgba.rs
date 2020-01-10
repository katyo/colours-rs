use super::{IsColorChannel, IsColor, HasAlpha, HasntAlpha, Rgb, Hsla, Hsva};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgba<T> {
    pub red: T,
    pub green: T,
    pub blue: T,
    pub alpha: T,
}

impl<T: IsColorChannel> IsColor for Rgba<T> {
    type Channel = T;
}

impl<T: IsColorChannel> HasAlpha for Rgba<T> {
    type Alphaless = Rgb<T>;

    fn split_alpha(self) -> (Self::Alphaless, Self::Channel) {
        let Rgba { red, green, blue, alpha } = self;
        (Rgb { red, green, blue }, alpha)
    }
}

impl<T> Rgba<T> {
    pub fn new(red: T, green: T, blue: T, alpha: T) -> Self {
        Self { red, green, blue, alpha }
    }
}

impl<T: IsColorChannel> Default for Rgba<T> {
    fn default() -> Self {
        Self::new(T::MIN, T::MIN, T::MIN, T::MAX)
    }
}

impl<T: IsColorChannel> From<Rgb<T>> for Rgba<T> {
    fn from(rgb: Rgb<T>) -> Self {
        rgb.with_alpha(T::MAX)
    }
}

impl From<Rgba<u8>> for Rgba<f32> {
    fn from(Rgba { red, green, blue, alpha }: Rgba<u8>) -> Self {
        Self::new(
            red as f32 / 255.0,
            green as f32 / 255.0,
            blue as f32 / 255.0,
            alpha as f32 / 255.0,
        )
    }
}

impl From<Rgba<f32>> for Rgba<u8> {
    fn from(Rgba { red, green, blue, alpha }: Rgba<f32>) -> Self {
        Self::new(
            (red.clamp_channel() * 255.0).round() as u8,
            (green.clamp_channel() * 255.0).round() as u8,
            (blue.clamp_channel() * 255.0).round() as u8,
            (alpha.clamp_channel() * 255.0).round() as u8,
        )
    }
}

impl From<Hsla<f32>> for Rgba<f32> {
    fn from(hsla: Hsla<f32>) -> Self {
        let (hsl, alpha) = hsla.split_alpha();
        Rgb::from(hsl).with_alpha(alpha)
    }
}

impl From<Hsva<f32>> for Rgba<f32> {
    fn from(hsva: Hsva<f32>) -> Self {
        let (hsv, alpha) = hsva.split_alpha();
        Rgb::from(hsv).with_alpha(alpha)
    }
}
