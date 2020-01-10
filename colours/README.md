# Color types with conversions

This library implements color types for some widely used color models
such as RGB, HSL, HSV and variants with alpha channel.
The support for other models may be added in future.

Each type parameterized using channel type: `u8` or `f32`.
The channel value range for `u8` is from _0_ to _255_,
and for `f32` is from _0.0_ to _1.0_.
