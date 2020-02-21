# Color types with conversions

[![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg)](https://opensource.org/licenses/MIT)
[![Travis-CI Status](https://travis-ci.com/katyo/colours-rs.svg?branch=master)](https://travis-ci.com/katyo/colours-rs)
[![Crates.io Package](https://img.shields.io/crates/v/colours.svg?style=popout)](https://crates.io/crates/colours)
[![Docs.rs API Docs](https://docs.rs/colours/badge.svg)](https://docs.rs/colours)

This library implements color types for some widely used color models
such as RGB, HSL, HSV and variants with alpha channel.
The support for other models may be added in future.

Each type parameterized using channel type: `u8` or `f32`.
The channel value range for `u8` is from _0_ to _255_,
and for `f32` is from _0.0_ to _1.0_.
