use super::{IsColorChannel, IsColor, HasAlpha, HasntAlpha, Hsv, Hsla, Rgba};

/// Generic HSV color with alpha component
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hsva<T> {
    pub hue: T,
    pub saturation: T,
    pub value: T,
    pub alpha: T,
}

impl<T: IsColorChannel> IsColor for Hsva<T> {
    type Channel = T;
}

impl<T: IsColorChannel> HasAlpha for Hsva<T> {
    type Alphaless = Hsv<T>;

    fn split_alpha(self) -> (Self::Alphaless, Self::Channel) {
        let Hsva { hue, saturation, value, alpha } = self;
        (Hsv { hue, saturation, value }, alpha)
    }
}

impl<T> Hsva<T> {
    pub fn new(hue: T, saturation: T, value: T, alpha: T) -> Self {
        Self { hue, saturation, value, alpha }
    }
}

impl<T: IsColorChannel> Default for Hsva<T> {
    fn default() -> Self {
        Self::new(T::MIN, T::MIN, T::MIN, T::MAX)
    }
}

impl<T: IsColorChannel> From<Hsv<T>> for Hsva<T> {
    fn from(hsv: Hsv<T>) -> Self {
        hsv.with_alpha(T::MAX)
    }
}

impl From<Hsva<u8>> for Hsva<f32> {
    fn from(Hsva { hue, saturation, value, alpha }: Hsva<u8>) -> Self {
        Self::new(
            hue as f32 / 255.0,
            saturation as f32 / 255.0,
            value as f32 / 255.0,
            alpha as f32 / 255.0,
        )
    }
}

impl From<Hsva<f32>> for Hsva<u8> {
    fn from(Hsva { hue, saturation, value, alpha }: Hsva<f32>) -> Self {
        Self::new(
            (hue.clamp_channel() * 255.0).round() as u8,
            (saturation.clamp_channel() * 255.0).round() as u8,
            (value.clamp_channel() * 255.0).round() as u8,
            (alpha.clamp_channel() * 255.0).round() as u8,
        )
    }
}

impl From<Hsla<f32>> for Hsva<f32> {
    fn from(hsla: Hsla<f32>) -> Self {
        let (hsl, alpha) = hsla.split_alpha();
        Hsv::from(hsl).with_alpha(alpha)
    }
}

impl From<Rgba<f32>> for Hsva<f32> {
    fn from(rgba: Rgba<f32>) -> Self {
        let (rgb, alpha) = rgba.split_alpha();
        Hsv::from(rgb).with_alpha(alpha)
    }
}
