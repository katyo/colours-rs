use super::{IsColorChannel, IsColor, HasAlpha, HasntAlpha, Hsva, Hsl, Rgb};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hsv<T> {
    pub hue: T,
    pub saturation: T,
    pub value: T,
}

impl<T: IsColorChannel> IsColor for Hsv<T> {
    type Channel = T;
}

impl<T: IsColorChannel> HasntAlpha for Hsv<T> {
    type Alphaful = Hsva<T>;

    fn with_alpha(self, alpha: Self::Channel) -> Self::Alphaful {
        let Hsv { hue, saturation, value } = self;
        Hsva { hue, saturation, value, alpha }
    }
}

impl<T> Hsv<T> {
    pub fn new(hue: T, saturation: T, value: T) -> Self {
        Self { hue, saturation, value }
    }
}

impl<T: IsColorChannel> Default for Hsv<T> {
    fn default() -> Self {
        Self::new(T::MIN, T::MIN, T::MIN)
    }
}

impl<T: IsColorChannel> From<Hsva<T>> for Hsv<T> {
    fn from(hsva: Hsva<T>) -> Self {
        hsva.without_alpha()
    }
}

impl From<Hsv<u8>> for Hsv<f32> {
    fn from(Hsv { hue, saturation, value }: Hsv<u8>) -> Self {
        Self::new(
            hue as f32 / 255.0,
            saturation as f32 / 255.0,
            value as f32 / 255.0,
        )
    }
}

impl From<Hsv<f32>> for Hsv<u8> {
    fn from(Hsv { hue, saturation, value }: Hsv<f32>) -> Self {
        Self::new(
            (hue.clamp_channel() * 255.0).round() as u8,
            (saturation.clamp_channel() * 255.0).round() as u8,
            (value.clamp_channel() * 255.0).round() as u8,
        )
    }
}

impl From<Hsl<f32>> for Hsv<f32> {
    fn from(Hsl { hue, saturation, lightness }: Hsl<f32>) -> Self {
        let lightness = lightness * 2.0;
        let saturation = saturation * if lightness > 1.0 { 2.0 - lightness } else { lightness };

        let lightness_plus_saturation = lightness + saturation;

        let value = lightness_plus_saturation * 0.5;
        let saturation = 2.0 * saturation / lightness_plus_saturation;

        Self { hue, saturation, value }
    }
}

impl From<Rgb<f32>> for Hsv<f32> {
    fn from(Rgb { red, green, blue }: Rgb<f32>) -> Self {
        let max = 0.0f32.max(red).max(green).max(blue);
        let min = 1.0f32.min(red).min(green).min(blue);

        if max > 0.0 {
            let value = max;
            let delta = max - min;
            let saturation = delta / max;

            let hue = if delta != 0.0 {
                (
                    if red == max {
                        (green - blue) / delta
                    } else if green == max {
                        2.0 + (blue - red) / delta
                    } else {
                        4.0 + (red - green) / delta
                    }
                    * 1.0 / 6.0
                ).unwind_channel()
            } else {
                0.0
            };

            Self { hue, saturation, value }
        } else {
            Self { hue: 0.0, saturation: 0.0, value: 0.0 }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_rgb() {
        assert_eq!(Hsv::<u8>::from(Hsv::from(Rgb::<f32>::from(Rgb::new(84u8, 37, 181)))), Hsv::new(184u8, 203, 181));
    }

    #[test]
    fn from_hsl() {
        assert_eq!(Hsv::<u8>::from(Hsv::from(Hsl::<f32>::from(Hsl::new(184u8, 169, 109)))), Hsv::new(184u8, 203, 181));
    }
}
