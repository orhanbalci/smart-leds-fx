//! Highlights over a background: dots, spots, flashes and sparkles.
//!
//! Each of these draws something small over a background, so each reads
//! [`Params::overlay`]: with it set they leave the background black, for a
//! caller that blends the result onto something else.

use lib8tion::sin8;
use smart_leds_trait::RGB8;

use crate::params::Params;
use crate::pixel::Pixel;
use crate::segment::EffectState;
use crate::utils::{BLACK, WHITE, color_blend, dim, fill_with, hash32, phase, travel};

/// Two dots travelling along the strip, one in the primary color and one in
/// `colors[1]`, half a strip apart, over `colors[2]`. `width` sets each dot,
/// from 1 to 16 pixels.
pub fn two_dots<P: Pixel>(pixels: &mut [P], _state: &mut EffectState, params: &Params) {
    let len = pixels.len();
    let background = if params.overlay {
        BLACK
    } else {
        params.colors[2]
    };
    fill_with(pixels, |_| background);

    let width = 1 + usize::from(params.width >> 4);
    let first = (travel(params.now_ms, params.rate) % len as u64) as usize;
    let second = (first + len / 2) % len;
    for step in 0..width.min(len) {
        let at = (first + step) % len;
        pixels[at] = P::from_rgb8(params.primary_at(at, len));
        let at = (second + step) % len;
        pixels[at] = P::from_rgb8(params.colors[1]);
    }
}

/// Brief flashes of the primary color striking at random places over
/// `colors[1]`. `intensity` sets how often they strike and `rate` how quickly
/// each passes.
pub fn lightning<P: Pixel>(pixels: &mut [P], _state: &mut EffectState, params: &Params) {
    let len = pixels.len();
    let background = if params.overlay {
        BLACK
    } else {
        params.colors[1]
    };
    fill_with(pixels, |_| background);

    // Each strike lasts one tick of the clock, so which one is showing — if
    // any — is a question the clock answers.
    let tick = phase(params.now_ms, params.rate) >> 3;
    let strike = hash32(tick);
    if (strike & 0xFF) as u8 > params.intensity / 2 {
        return;
    }
    let start = (strike >> 8) as usize % len;
    let length = 1 + (strike >> 16) as usize % (len / 2 + 1);
    let level = 128 + (strike >> 24) as u8 / 2;
    let end = (start + length).min(len);
    for (step, pixel) in pixels[start..end].iter_mut().enumerate() {
        *pixel = P::from_rgb8(dim(params.primary_at(start + step, len), level));
    }
}

/// Evenly spaced spots of the primary color over `colors[1]`. `spread` sets
/// how far apart they sit, from 2 to 33 pixels, and `width` how wide each is.
pub fn spots<P: Pixel>(pixels: &mut [P], _state: &mut EffectState, params: &Params) {
    lit_spots(pixels, params, 255);
}

/// [`spots`], with every spot fading in and out together.
pub fn spots_fade<P: Pixel>(pixels: &mut [P], _state: &mut EffectState, params: &Params) {
    // Spread and width take this effect's sliders, so the fade keeps its own
    // steady pace.
    lit_spots(pixels, params, sin8(phase(params.now_ms, 96) as u8));
}

fn lit_spots<P: Pixel>(pixels: &mut [P], params: &Params, level: u8) {
    let len = pixels.len();
    let background = if params.overlay {
        BLACK
    } else {
        params.colors[1]
    };
    let spacing = 2 + usize::from(params.spread >> 3);
    let width = 1 + usize::from(params.width >> 4);
    for (i, pixel) in pixels.iter_mut().enumerate() {
        *pixel = P::from_rgb8(if i % spacing < width {
            color_blend(background, params.primary_at(i, len), level)
        } else {
            background
        });
    }
}

/// Sparkles of `colors[2]` — white while that is black — over `colors[0]`, a
/// new set with every step. `intensity` sets how many.
pub fn solid_glitter<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    fill_with(pixels, |_| params.colors[0]);
    scatter(pixels, params, state.counter);
    state.counter = state.counter.wrapping_add(1);
}

/// Sparkles of `colors[2]` — white while that is black — scattered over the
/// palette. `intensity` sets how many and `rate` how fast they move.
pub fn glitter<P: Pixel>(pixels: &mut [P], _state: &mut EffectState, params: &Params) {
    let len = pixels.len();
    let overlay = params.overlay;
    fill_with(pixels, |i| {
        if overlay {
            BLACK
        } else {
            params.primary_at(i, len)
        }
    });

    // A new set of sparkles every tick of the clock.
    scatter(pixels, params, phase(params.now_ms, params.rate) >> 4);
}

/// Scatters `intensity`-many sparkles of `colors[2]` — white while that is
/// black — over `pixels`, in the places `tick` picks.
fn scatter<P: Pixel>(pixels: &mut [P], params: &Params, tick: u32) {
    let len = pixels.len();
    let is_black = |c: RGB8| c.r == 0 && c.g == 0 && c.b == 0;
    let color = if is_black(params.colors[2]) {
        WHITE
    } else {
        params.colors[2]
    };
    let count = 1 + len * usize::from(params.intensity) / 512;
    for sparkle in 0..count {
        let at = hash32(tick ^ (sparkle as u32).rotate_left(16)) as usize % len;
        pixels[at] = P::from_rgb8(color);
    }
}
