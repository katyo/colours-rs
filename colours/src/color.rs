/// Something which can be treated as color
pub trait IsColor {
    type Channel: IsColorChannel;
}

/// Something which can be color channel
pub trait IsColorChannel: Sized + Copy + PartialOrd {
    const MIN: Self;
    const MAX: Self;

    #[inline]
    fn clamp_channel(self) -> Self {
        if self < Self::MIN {
            Self::MIN
        } else if self > Self::MAX {
            Self::MAX
        } else {
            self
        }
    }

    fn unwind_channel(self) -> Self;
}

/// The color which has an alpha component
pub trait HasAlpha: IsColor + Sized {
    /// The color of same kind but without alpha component
    type Alphaless: HasntAlpha + Sized;

    /// Split alpha component from color
    fn split_alpha(self) -> (Self::Alphaless, Self::Channel);

    /// Remove alpha component of color
    fn without_alpha(self) -> Self::Alphaless {
        self.split_alpha().0
    }

    /// Extract alpha component from color
    fn only_alpha(self) -> Self::Channel {
        self.split_alpha().1
    }
}

/// The color which has not alpha component
pub trait HasntAlpha: IsColor {
    /// The color of same kind but with alpha component
    type Alphaful: HasAlpha;

    /// Append alpha component to the color
    fn with_alpha(self, alpha: Self::Channel) -> Self::Alphaful;
}

impl IsColorChannel for u8 {
    const MIN: Self = 0;
    const MAX: Self = 255;

    #[inline]
    fn unwind_channel(self) -> Self {
        self
    }
}

impl IsColorChannel for f32 {
    const MIN: Self = 0.0;
    const MAX: Self = 1.0;

    #[inline]
    fn unwind_channel(self) -> Self {
        if self >= 0.0 && self <= 1.0 {
            self
        } else {
            ((self % 1.0) + 1.0) % 1.0
        }
    }
}
