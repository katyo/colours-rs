pub trait IsColor {
    type Channel: IsColorChannel;
}

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

pub trait HasAlpha: IsColor + Sized {
    type Alphaless: HasntAlpha + Sized;
    fn split_alpha(self) -> (Self::Alphaless, Self::Channel);
    fn without_alpha(self) -> Self::Alphaless {
        self.split_alpha().0
    }
    fn only_alpha(self) -> Self::Channel {
        self.split_alpha().1
    }
}

pub trait HasntAlpha: IsColor {
    type Alphaful: HasAlpha;
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
