use smart_leds_trait::RGB8;

use crate::pixel::Pixel;

/// Convenience constructor — shorter than writing `RGB8 { r, g, b }` inline.
pub const fn rgb(r: u8, g: u8, b: u8) -> RGB8 {
    RGB8 { r, g, b }
}

pub const BLACK: RGB8 = RGB8 { r: 0, g: 0, b: 0 };
pub const WHITE: RGB8 = RGB8 {
    r: 255,
    g: 255,
    b: 255,
};
pub const RED: RGB8 = RGB8 { r: 255, g: 0, b: 0 };
pub const GREEN: RGB8 = RGB8 { r: 0, g: 255, b: 0 };
pub const BLUE: RGB8 = RGB8 { r: 0, g: 0, b: 255 };
pub const PURPLE: RGB8 = RGB8 {
    r: 128,
    g: 0,
    b: 128,
};
pub const ORANGE: RGB8 = RGB8 {
    r: 255,
    g: 165,
    b: 0,
};

/// Maps 0–255 to a position on the RGB hue wheel.
pub(crate) fn color_wheel(pos: u8) -> RGB8 {
    let pos = 255 - pos;
    if pos < 85 {
        RGB8 {
            r: 255 - pos * 3,
            g: 0,
            b: pos * 3,
        }
    } else if pos < 170 {
        let pos = pos - 85;
        RGB8 {
            r: 0,
            g: pos * 3,
            b: 255 - pos * 3,
        }
    } else {
        let pos = pos - 170;
        RGB8 {
            r: pos * 3,
            g: 255 - pos * 3,
            b: 0,
        }
    }
}

/// Maps 0–255 to a full sine cycle, output 0–255.
/// Integer-only, so it stays cheap on cores without an FPU.
pub(crate) fn sine8(pos: u8) -> u8 {
    lib8tion::sin8(pos)
}

/// `base` scaled by `intensity`, where [`Params::DEFAULT_INTENSITY`] leaves it
/// unchanged. Saturates at 255.
///
/// [`Params::DEFAULT_INTENSITY`]: crate::Params::DEFAULT_INTENSITY
pub(crate) fn scaled(base: u8, intensity: u8) -> u8 {
    let value =
        u16::from(base) * u16::from(intensity) / u16::from(crate::Params::DEFAULT_INTENSITY);
    value.min(255) as u8
}

/// Linearly interpolate between two colors.
/// `blend = 0` → pure `c1`, `blend = 255` → pure `c2`.
pub(crate) fn color_blend(c1: RGB8, c2: RGB8, blend: u8) -> RGB8 {
    let b = blend as u16;
    let ib = 255 - b;
    RGB8 {
        r: ((c1.r as u16 * ib + c2.r as u16 * b) / 255) as u8,
        g: ((c1.g as u16 * ib + c2.g as u16 * b) / 255) as u8,
        b: ((c1.b as u16 * ib + c2.b as u16 * b) / 255) as u8,
    }
}

/// `color` at brightness `level`, where `255` leaves it unchanged.
pub(crate) fn dim(color: RGB8, level: u8) -> RGB8 {
    let channel = |c: u8| ((u16::from(c) * (u16::from(level) + 1)) >> 8) as u8;
    RGB8 {
        r: channel(color.r),
        g: channel(color.g),
        b: channel(color.b),
    }
}

/// A phase that advances with the clock at `rate`: its low byte runs once
/// around every two seconds at the default rate, every second at full rate,
/// and all but stops at zero. Take bytes from it rather than dividing it, so
/// the phase stays continuous when the clock wraps.
pub(crate) fn phase(now_ms: u32, rate: u8) -> u32 {
    now_ms.wrapping_mul(u32::from(rate) + 1) >> 10
}

/// How many pixels a pattern moving at `rate` has travelled by `now_ms`: about
/// 8 a second at the default rate and 16 at full rate. Computed in 64 bits, so
/// it only restarts when the clock itself wraps.
pub(crate) fn travel(now_ms: u32, rate: u8) -> u64 {
    (u64::from(now_ms) * (u64::from(rate) + 1)) >> 14
}

/// Where pixel `index` falls within a repeating pattern of `period` pixels once
/// the pattern has moved `travel` pixels forward.
pub(crate) fn pattern_position(index: usize, travel: u64, period: usize) -> usize {
    let period = period.max(1) as u64;
    ((index as u64 + period - travel % period) % period) as usize
}

/// A well-mixed hash of `x`: for a color or rhythm that must stay the same for
/// a given pixel or zone.
pub(crate) fn hash32(x: u32) -> u32 {
    let mut z = x.wrapping_add(0x9E37_79B9);
    z = (z ^ (z >> 16)).wrapping_mul(0x21F0_AAAD);
    z = (z ^ (z >> 15)).wrapping_mul(0x735A_2D97);
    z ^ (z >> 15)
}

/// Set every pixel to `color`.
pub(crate) fn fill<P: Pixel>(pixels: &mut [P], color: RGB8) {
    let pixel = P::from_rgb8(color);
    for p in pixels.iter_mut() {
        *p = pixel;
    }
}

/// Set each pixel to `color(index)`.
pub(crate) fn fill_with<P: Pixel>(pixels: &mut [P], color: impl Fn(usize) -> RGB8) {
    for (i, p) in pixels.iter_mut().enumerate() {
        *p = P::from_rgb8(color(i));
    }
}

/// Set every pixel to the primary color, following the palette along the
/// strip when there is one.
pub(crate) fn fill_primary<P: Pixel>(pixels: &mut [P], params: &crate::Params) {
    let len = pixels.len();
    fill_with(pixels, |i| params.primary_at(i, len));
}

/// Blend every pixel toward `target` by `rate/255` each call.
pub(crate) fn fade_out<P: Pixel>(pixels: &mut [P], target: RGB8, rate: u8) {
    for p in pixels.iter_mut() {
        *p = P::from_rgb8(color_blend(p.to_rgb8(), target, rate));
    }
}

/// Advance the PRNG and return a new random `u32`.
/// Store the return value back into `EffectState::aux` to keep the chain going.
/// Seeds itself to a non-zero value if `state` is 0.
pub(crate) fn next_rand(state: u32) -> u32 {
    let seed = if state == 0 {
        0xdead_beef_u64
    } else {
        state as u64 | (state as u64) << 32
    };
    fastrand::Rng::with_seed(seed).u32(..)
}

/// Returns a wheel index at least 42 hue steps away from `current`.
/// Advances the PRNG until a sufficiently distant value is found.
/// Returns `(new_index, updated_rng)`.
pub(crate) fn rand_wheel_index(current: u8, rng: u32) -> (u8, u32) {
    let mut rng = rng;
    loop {
        rng = next_rand(rng);
        let r = rng as u8;
        let x = (current as i16 - r as i16).unsigned_abs() as u8;
        let d = x.min(255u8.saturating_sub(x));
        if d >= 42 {
            return (r, rng);
        }
    }
}
