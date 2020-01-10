use super::{IsColorChannel, IsColor, HasAlpha, HasntAlpha, Rgba, Hsl, Hsv};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb<T> {
    pub red: T,
    pub green: T,
    pub blue: T,
}

impl<T: IsColorChannel> IsColor for Rgb<T> {
    type Channel = T;
}

impl<T: IsColorChannel> HasntAlpha for Rgb<T> {
    type Alphaful = Rgba<T>;

    fn with_alpha(self, alpha: Self::Channel) -> Self::Alphaful {
        let Rgb { red, green, blue } = self;
        Rgba { red, green, blue, alpha }
    }
}

impl<T> Rgb<T> {
    pub fn new(red: T, green: T, blue: T) -> Self {
        Self { red, green, blue }
    }
}

impl<T: IsColorChannel> Default for Rgb<T> {
    fn default() -> Self {
        Self::new(T::MIN, T::MIN, T::MIN)
    }
}

impl<T: IsColorChannel> From<Rgba<T>> for Rgb<T> {
    fn from(rgba: Rgba<T>) -> Self {
        rgba.without_alpha()
    }
}

impl From<Rgb<u8>> for Rgb<f32> {
    fn from(Rgb { red, green, blue }: Rgb<u8>) -> Self {
        Self::new(
            red as f32 / 255.0,
            green as f32 / 255.0,
            blue as f32 / 255.0,
        )
    }
}

impl From<Rgb<f32>> for Rgb<u8> {
    fn from(Rgb { red, green, blue }: Rgb<f32>) -> Self {
        Self::new(
            (red.clamp_channel() * 255.0).round() as u8,
            (green.clamp_channel() * 255.0).round() as u8,
            (blue.clamp_channel() * 255.0).round() as u8,
        )
    }
}

impl From<Hsl<f32>> for Rgb<f32> {
    fn from(Hsl { hue, saturation, lightness }: Hsl<f32>) -> Self {
        let hue = hue.unwind_channel();
        let saturation = saturation.clamp_channel();
        let lightness = lightness.clamp_channel();

        let c = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
        let x = c * (1.0 - ((hue * 6.0) % 2.0 - 1.0).abs());
        let m = lightness - c / 2.0;

        hue_cxm_to_rgb(hue, c, x, m)
    }
}

impl From<Hsv<f32>> for Rgb<f32> {
    fn from(Hsv { hue, saturation, value }: Hsv<f32>) -> Self {
        let hue = hue.unwind_channel();
        let saturation = saturation.clamp_channel();
        let value = value.clamp_channel();

        let c = value * saturation;
        let x = c * (1.0 - ((hue * 6.0) % 2.0 - 1.0).abs());
        let m = value - c;

        hue_cxm_to_rgb(hue, c, x, m)
    }
}

fn hue_cxm_to_rgb(hue: f32, c: f32, x: f32, m: f32) -> Rgb<f32> {
    let mut red = 0.0;
    let mut green = 0.0;
    let mut blue = 0.0;

    if (hue >= 0.0 && hue < 1.0/6.0) || hue >= 1.0 {
        red = c;
        green = x;
    } else if hue >= 1.0/6.0 && hue < 1.0/3.0 {
        red = x;
        green = c;
    } else if hue >= 1.0/3.0 && hue < 0.5 {
        green = c;
        blue = x;
    } else if hue >= 0.5 && hue < 2.0/3.0 {
        green = x;
        blue = c;
    } else if hue >= 2.0/3.0 && hue < 5.0/6.0 {
        red = x;
        blue = c;
    } else if hue >= 5.0/6.0 && hue < 1.0 {
        red = c;
        blue = x;
    }

    red += m;
    green += m;
    blue += m;

    Rgb::new(red, green, blue)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_u8() {
        assert_eq!(Rgb::<f32>::from(Rgb::<u8>::new(84, 37, 181)),
                   Rgb::new(84.0/255.0, 37.0/255.0, 181.0/255.0));
    }

    #[test]
    fn from_f32() {
        assert_eq!(Rgb::<u8>::from(Rgb::<f32>::new(84.0/255.0, 37.0/255.0, 181.0/255.0)),
                   Rgb::new(84, 37, 181));
    }

    #[test]
    fn from_hsl() {
        assert_eq!(Rgb::<u8>::from(Rgb::from(Hsl::<f32>::from(Hsl::new(184u8, 168, 109)))),
                   Rgb::new(84u8, 37, 181));
    }

    #[test]
    fn from_hsv() {
        assert_eq!(Rgb::<u8>::from(Rgb::from(Hsv::<f32>::from(Hsv::new(12u8, 107, 66)))),
                   Rgb::new(66u8, 46, 38));
    }
}
