use super::{IsColorChannel, IsColor, HasAlpha, HasntAlpha, Hsl, Hsva, Rgba};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hsla<T> {
    pub hue: T,
    pub saturation: T,
    pub lightness: T,
    pub alpha: T,
}

impl<T: IsColorChannel> IsColor for Hsla<T> {
    type Channel = T;
}

impl<T: IsColorChannel> HasAlpha for Hsla<T> {
    type Alphaless = Hsl<T>;

    fn split_alpha(self) -> (Self::Alphaless, Self::Channel) {
        let Hsla { hue, saturation, lightness, alpha } = self;
        (Hsl { hue, saturation, lightness }, alpha)
    }
}

impl<T> Hsla<T> {
    fn new(hue: T, saturation: T, lightness: T, alpha: T) -> Self {
        Self { hue, saturation, lightness, alpha }
    }
}

impl<T: IsColorChannel> Default for Hsla<T> {
    fn default() -> Self {
        Self::new(T::MIN, T::MIN, T::MIN, T::MAX)
    }
}

impl<T: IsColorChannel> From<Hsl<T>> for Hsla<T> {
    fn from(hsl: Hsl<T>) -> Self {
        hsl.with_alpha(T::MAX)
    }
}

impl From<Hsla<u8>> for Hsla<f32> {
    fn from(Hsla { hue, saturation, lightness, alpha }: Hsla<u8>) -> Self {
        Self::new(
            hue as f32 / 255.0,
            saturation as f32 / 255.0,
            lightness as f32 / 255.0,
            alpha as f32 / 255.0,
        )
    }
}

impl From<Hsla<f32>> for Hsla<u8> {
    fn from(Hsla { hue, saturation, lightness, alpha }: Hsla<f32>) -> Self {
        Self::new(
            (hue.clamp_channel() * 255.0).round() as u8,
            (saturation.clamp_channel() * 255.0).round() as u8,
            (lightness.clamp_channel() * 255.0).round() as u8,
            (alpha.clamp_channel() * 255.0).round() as u8,
        )
    }
}

impl From<Hsva<f32>> for Hsla<f32> {
    fn from(hsva: Hsva<f32>) -> Self {
        let (hsv, alpha) = hsva.split_alpha();
        Hsl::from(hsv).with_alpha(alpha)
    }
}

impl From<Rgba<f32>> for Hsla<f32> {
    fn from(rgba: Rgba<f32>) -> Self {
        let (rgb, alpha) = rgba.split_alpha();
        Hsl::from(rgb).with_alpha(alpha)
    }
}
