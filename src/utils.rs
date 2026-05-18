use micromath::F32Ext;
use smart_leds_trait::RGB8;

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
pub fn color_wheel(pos: u8) -> RGB8 {
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
/// Uses micromath for a smooth approximation suitable for no_std.
pub fn sine8(pos: u8) -> u8 {
    let angle = pos as f32 * core::f32::consts::TAU / 256.0;
    (angle.sin() * 127.5 + 127.5) as u8
}

/// Linearly interpolate between two colors.
/// `blend = 0` → pure `c1`, `blend = 255` → pure `c2`.
pub fn color_blend(c1: RGB8, c2: RGB8, blend: u8) -> RGB8 {
    let b = blend as u16;
    let ib = 255 - b;
    RGB8 {
        r: ((c1.r as u16 * ib + c2.r as u16 * b) / 255) as u8,
        g: ((c1.g as u16 * ib + c2.g as u16 * b) / 255) as u8,
        b: ((c1.b as u16 * ib + c2.b as u16 * b) / 255) as u8,
    }
}

/// Blend every pixel toward `target` by `rate/255` each call.
pub fn fade_out(pixels: &mut [RGB8], target: RGB8, rate: u8) {
    for p in pixels.iter_mut() {
        *p = color_blend(*p, target, rate);
    }
}

/// Advance the PRNG and return a new random `u32`.
/// Store the return value back into `EffectState::aux` to keep the chain going.
/// Seeds itself to a non-zero value if `state` is 0.
pub fn next_rand(state: u32) -> u32 {
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
pub fn rand_wheel_index(current: u8, rng: u32) -> (u8, u32) {
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
