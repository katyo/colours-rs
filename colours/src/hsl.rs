use super::{IsColorChannel, IsColor, HasAlpha, HasntAlpha, Rgb, Hsv, Hsla};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hsl<T> {
    pub hue: T,
    pub saturation: T,
    pub lightness: T,
}

impl<T: IsColorChannel> IsColor for Hsl<T> {
    type Channel = T;
}

impl<T: IsColorChannel> HasntAlpha for Hsl<T> {
    type Alphaful = Hsla<T>;

    fn with_alpha(self, alpha: Self::Channel) -> Self::Alphaful {
        let Hsl { hue, saturation, lightness } = self;
        Hsla { hue, saturation, lightness, alpha }
    }
}

impl<T> Hsl<T> {
    pub fn new(hue: T, saturation: T, lightness: T) -> Self {
        Self { hue, saturation, lightness }
    }
}

impl<T: IsColorChannel> Default for Hsl<T> {
    fn default() -> Self {
        Self::new(T::MIN, T::MIN, T::MIN)
    }
}

impl<T: IsColorChannel> From<Hsla<T>> for Hsl<T> {
    fn from(hsla: Hsla<T>) -> Self {
        hsla.without_alpha()
    }
}

impl From<Hsl<u8>> for Hsl<f32> {
    fn from(Hsl { hue, saturation, lightness }: Hsl<u8>) -> Self {
        Self::new(
            hue as f32 / 255.0,
            saturation as f32 / 255.0,
            lightness as f32 / 255.0,
        )
    }
}

impl From<Hsl<f32>> for Hsl<u8> {
    fn from(Hsl { hue, saturation, lightness }: Hsl<f32>) -> Self {
        Self::new(
            (hue.clamp_channel() * 255.0).round() as u8,
            (saturation.clamp_channel() * 255.0).round() as u8,
            (lightness.clamp_channel() * 255.0).round() as u8,
        )
    }
}

impl From<Hsv<f32>> for Hsl<f32> {
    fn from(Hsv { hue, saturation, value }: Hsv<f32>) -> Self {
        let lightness = (2.0 - saturation) * value;
        let denominator = if lightness > 1.0 {
            2.0 - lightness
        } else {
            lightness
        };
        let saturation = if denominator > 0.0 {
            saturation * value / denominator
        } else {
            0.0
        };
        let lightness = lightness * 0.5;
        Self { hue, saturation, lightness }
    }
}

impl From<Rgb<f32>> for Hsl<f32> {
    fn from(Rgb { red, green, blue }: Rgb<f32>) -> Self {
        let c_max = red.max(green).max(blue);
        let c_min = red.min(green).min(blue);
        let c_delta = c_max - c_min;

        let mut hue = 0.0;
        let mut saturation = 0.0;

        let lightness = (c_max + c_min) / 2.0;

        if c_delta != 0.0 {
            if c_max == red {
                hue = 1.0/6.0 * (((green - blue) / c_delta) % 6.0);
            } else if c_max == green {
                hue = 1.0/6.0 * ((blue - red) / c_delta + 2.0);
            } else if c_max == blue {
                hue = 1.0/6.0 * ((red - green) / c_delta + 4.0);
            }
            saturation = c_delta / (1.0 - (2.0 * lightness - 1.0).abs());
        }

        let hue = hue.unwind_channel();

        Self { hue, saturation, lightness }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_rgb() {
        assert_eq!(Hsl::<u8>::from(Hsl::from(Rgb::<f32>::from(Rgb::new(84u8, 37, 181)))), Hsl::new(184u8, 168, 109));
    }

    #[test]
    fn from_hsv() {
        assert_eq!(Hsl::<u8>::from(Hsl::from(Hsv::<f32>::from(Hsv::new(184u8, 203, 181)))), Hsl::new(184u8, 169, 109));
    }
}
