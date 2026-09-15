use smart_leds_trait::RGB8;

use crate::params::Params;
use crate::pixel::Pixel;
use crate::segment::EffectState;
use crate::utils::{BLACK, fade_out, next_rand, scaled};

// Every scanner fades its trail by `64` at default intensity; higher
// intensity gives shorter trails.

/// Knight Rider / Cylon scanner: one bright dot with a fading trail.
/// `state.counter` = current step (0..seg_len*2-2), `state.aux` = call counter for color.
pub fn larson_scanner<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    fade_out(pixels, BLACK, scaled(64, params.intensity));

    let len = pixels.len();
    let total = (len * 2).saturating_sub(2);
    let step = state.counter as usize % total.max(1);

    let pos = if step < len { step } else { total - step };
    if pos < len {
        pixels[pos] = P::from_rgb8(params.primary_at(pos, len));
    }

    state.counter = state.counter.wrapping_add(1);
}

/// Two Larson scanners moving toward each other from opposite ends.
/// `state.counter` = position, `state.aux` bit 0 = direction.
pub fn dual_larson<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    fade_out(pixels, BLACK, scaled(64, params.intensity));

    let len = pixels.len();
    let pos = state.counter as usize;
    let going_forward = state.aux & 1 == 0;

    if pos < len {
        pixels[pos] = P::from_rgb8(params.primary_at(pos, len));
        let mirror_pos = len.saturating_sub(1).saturating_sub(pos);
        pixels[mirror_pos] = P::from_rgb8(if params.colors[2] != BLACK {
            params.colors[2]
        } else {
            params.primary_at(mirror_pos, len)
        });
    }

    if going_forward {
        state.counter += 1;
        if state.counter as usize >= len - 1 {
            state.aux ^= 1;
        }
    } else {
        state.counter = state.counter.saturating_sub(1);
        if state.counter == 0 {
            state.aux ^= 1;
        }
    }
}

/// Larson scanner with a hue that cycles with each bounce.
/// `state.counter` = position, `state.aux`: bit 0 = direction, bits 8-15 = call count.
pub fn rainbow_larson<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    fade_out(pixels, BLACK, scaled(64, params.intensity));

    let len = pixels.len();
    let pos = state.counter as usize;
    let going_forward = state.aux & 1 == 0;
    let call_count = ((state.aux >> 8) & 0xFF) as u8;

    if pos < len {
        pixels[pos] = P::from_rgb8(params.wheel(call_count.wrapping_mul(16)));
    }

    if going_forward {
        state.counter += 1;
        if state.counter as usize >= len - 1 {
            state.aux = (state.aux ^ 1) | (((call_count.wrapping_add(1)) as u32) << 8);
        }
    } else {
        state.counter = state.counter.saturating_sub(1);
        if state.counter == 0 {
            state.aux = (state.aux ^ 1) | (((call_count.wrapping_add(1)) as u32) << 8);
        }
    }
}

/// Comet: one bright head fires forward, leaving a fading trail.
pub fn comet<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    fade_out(pixels, BLACK, scaled(64, params.intensity));

    let len = pixels.len();
    let pos = state.counter as usize % len;
    pixels[pos] = P::from_rgb8(params.primary_at(pos, len));

    state.counter = state.counter.wrapping_add(1);
}

/// Two independent comets that start randomly and leave fading trails.
/// Packs two u16 comet positions (0xFFFF = inactive) into counter and aux.
pub fn multi_comet<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    fade_out(pixels, BLACK, scaled(64, params.intensity));

    let len = pixels.len() as u32;
    let last = pixels.len() - 1;

    // Comet 0: low 16 bits of counter; 0xFFFF = inactive.
    // Comet 1: high 16 bits of counter; 0xFFFF = inactive.
    // aux = rng seed.
    let mut c0 = state.counter & 0xFFFF;
    let mut c1 = (state.counter >> 16) & 0xFFFF;

    if c0 != 0xFFFF {
        let pos = (c0 as usize).min(last);
        pixels[pos] = P::from_rgb8(params.primary_at(pos, pixels.len()));
        c0 += 1;
        if c0 >= len {
            c0 = 0xFFFF;
        }
    } else {
        let rng = next_rand(state.aux);
        state.aux = rng;
        if rng % len == 0 {
            c0 = 0;
        }
    }

    if c1 != 0xFFFF {
        let pos = (c1 as usize).min(last);
        pixels[pos] =
            P::from_rgb8(params.colors[2].try_or(params.primary_at(pos, pixels.len())));
        c1 += 1;
        if c1 >= len {
            c1 = 0xFFFF;
        }
    } else {
        let rng = next_rand(state.aux.wrapping_add(1));
        state.aux = rng;
        if rng % len == 0 {
            c1 = 0;
        }
    }

    state.counter = c0 | (c1 << 16);
}

trait OrElse {
    fn try_or(self, fallback: Self) -> Self;
}

impl OrElse for RGB8 {
    fn try_or(self, fallback: Self) -> Self {
        if self.r == 0 && self.g == 0 && self.b == 0 {
            fallback
        } else {
            self
        }
    }
}
