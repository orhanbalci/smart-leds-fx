//! Pixel types the effects can draw into.

use smart_leds_trait::RGB8;

/// A pixel an effect can draw into and read back.
///
/// Effects compute colours as [`RGB8`] and convert only where they touch the
/// buffer, so the same effect code fills a buffer of any pixel type that
/// implements this — [`RGB8`] for a [`SmartLedsWrite`] driver, or another
/// crate's pixel type without copying frames between buffers.
///
/// [`SmartLedsWrite`]: smart_leds_trait::SmartLedsWrite
pub trait Pixel: Copy {
    /// The pixel for `color`.
    fn from_rgb8(color: RGB8) -> Self;

    /// This pixel's colour.
    fn to_rgb8(self) -> RGB8;
}

impl Pixel for RGB8 {
    #[inline]
    fn from_rgb8(color: RGB8) -> Self {
        color
    }

    #[inline]
    fn to_rgb8(self) -> RGB8 {
        self
    }
}

#[cfg(feature = "color8")]
impl Pixel for color8::Crgb {
    #[inline]
    fn from_rgb8(color: RGB8) -> Self {
        color8::Crgb {
            r: color.r,
            g: color.g,
            b: color.b,
        }
    }

    #[inline]
    fn to_rgb8(self) -> RGB8 {
        RGB8 {
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }
}
