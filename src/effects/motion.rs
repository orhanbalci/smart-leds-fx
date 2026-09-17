//! Patterns that travel along the strip: bands, streams, spots and wipes.
//!
//! All but Tri Wipe are time-driven: they draw from [`Params::now_ms`] and move
//! at [`Params::rate`], so they move smoothly however often they are stepped.

use lib8tion::{sin8, triwave8};

use crate::params::Params;
use crate::pixel::Pixel;
use crate::segment::EffectState;
use crate::utils::{color_blend, hash32, pattern_position, phase, travel};

/// Sawtooth bands travelling along the strip: each rises from `colors[1]` to
/// the primary color and drops back. `width` sets each band, from 2 to 33
/// pixels.
pub fn saw<P: Pixel>(pixels: &mut [P], _state: &mut EffectState, params: &Params) {
    let len = pixels.len();
    let period = 2 + usize::from(params.width >> 3);
    let moved = travel(params.now_ms, params.rate);
    for (i, pixel) in pixels.iter_mut().enumerate() {
        let at = pattern_position(i, moved, period);
        let level = (at * 255 / (period - 1)) as u8;
        *pixel = P::from_rgb8(color_blend(
            params.colors[1],
            params.primary_at(i, len),
            level,
        ));
    }
}

/// Alternating bands of the primary color and `colors[1]` travelling along the
/// strip. `width` sets each band, from 1 to 16 pixels.
pub fn bands<P: Pixel>(pixels: &mut [P], _state: &mut EffectState, params: &Params) {
    let len = pixels.len();
    let band = 1 + usize::from(params.width >> 4);
    let moved = travel(params.now_ms, params.rate);
    for (i, pixel) in pixels.iter_mut().enumerate() {
        *pixel = P::from_rgb8(if pattern_position(i, moved, band * 2) < band {
            params.primary_at(i, len)
        } else {
            params.colors[1]
        });
    }
}

/// Zones of random colors — from the palette, or the hue wheel — flowing along
/// the strip. `width` sets each zone, from 1 to 16 pixels.
pub fn stream<P: Pixel>(pixels: &mut [P], _state: &mut EffectState, params: &Params) {
    let zone = 1 + u64::from(params.width >> 4);
    let moved = travel(params.now_ms, params.rate);
    for (i, pixel) in pixels.iter_mut().enumerate() {
        // Counted from far ahead, so the zone index never goes below zero.
        let index = (i as u64 + (1 << 32) - moved) / zone;
        *pixel = P::from_rgb8(params.wheel(hash32(index as u32) as u8));
    }
}

/// Zones of four pixels flowing along the strip, each a random red, green and
/// blue of its own rather than a color from the palette.
pub fn stream2<P: Pixel>(pixels: &mut [P], _state: &mut EffectState, params: &Params) {
    const ZONE: u64 = 4;
    let moved = travel(params.now_ms, params.rate);
    for (i, pixel) in pixels.iter_mut().enumerate() {
        // Counted from far ahead, so the zone index never goes below zero.
        let zone = hash32(((i as u64 + (1 << 32) - moved) / ZONE) as u32);
        *pixel = P::from_rgb8(smart_leds_trait::RGB8 {
            r: (zone >> 16) as u8,
            g: (zone >> 8) as u8,
            b: zone as u8,
        });
    }
}

/// A soft spot of the primary color gliding back and forth over `colors[1]`.
/// `spread` sets how far its glow reaches, from a pixel to the whole strip.
pub fn gradient<P: Pixel>(pixels: &mut [P], _state: &mut EffectState, params: &Params) {
    let len = pixels.len();
    let sweep = triwave8(phase(params.now_ms, params.rate) as u8);
    let centre = usize::from(sweep) * (len - 1) / 255;
    let reach = 1 + len * usize::from(params.spread) / 255;
    for (i, pixel) in pixels.iter_mut().enumerate() {
        let level = 255usize.saturating_sub(i.abs_diff(centre) * 255 / reach) as u8;
        *pixel = P::from_rgb8(color_blend(
            params.colors[1],
            params.primary_at(i, len),
            level,
        ));
    }
}

/// A head sweeping from the start of the strip to the end, over and over, the
/// primary color fading into `colors[1]` behind it. `spread` sets how long
/// the tail is.
pub fn loading<P: Pixel>(pixels: &mut [P], _state: &mut EffectState, params: &Params) {
    let len = pixels.len();
    let head = usize::from(phase(params.now_ms, params.rate) as u8) * len / 256;
    let tail = 1 + len * usize::from(params.spread) / 255;
    for (i, pixel) in pixels.iter_mut().enumerate() {
        let behind = (head + len - i) % len;
        let level = 255usize.saturating_sub(behind * 255 / tail) as u8;
        *pixel = P::from_rgb8(color_blend(
            params.colors[1],
            params.primary_at(i, len),
            level,
        ));
    }
}

/// Two sine waves running through each other in opposite directions over
/// `colors[1]`: one in the primary color, one in `colors[2]` — or the primary
/// again while that is black. `scale` sets how tightly the waves repeat.
pub fn running_dual<P: Pixel>(pixels: &mut [P], _state: &mut EffectState, params: &Params) {
    let len = pixels.len();
    let offset = phase(params.now_ms, params.rate) as u8;
    let step = u32::from(params.scale / 16 + 1);
    let is_black = |c: smart_leds_trait::RGB8| c.r == 0 && c.g == 0 && c.b == 0;
    for (i, pixel) in pixels.iter_mut().enumerate() {
        let angle = (i as u32 * step) as u8;
        let left = params.primary_at(i, len);
        let right = if is_black(params.colors[2]) {
            left
        } else {
            params.colors[2]
        };
        let forward = sin8(angle.wrapping_add(offset));
        let backward = sin8(angle.wrapping_sub(offset));
        let color = color_blend(params.colors[1], left, forward);
        *pixel = P::from_rgb8(color_blend(color, right, backward / 2));
    }
}

/// Wipes the primary color, then `colors[1]`, then `colors[2]` across the
/// strip, each over the last, one pixel per step.
pub fn tri_wipe<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    let len = pixels.len();
    let position = state.counter as usize % (len * 3);
    let (stage, reached) = (position / len, position % len);
    let color = |stage: usize, i: usize| match stage % 3 {
        0 => params.primary_at(i, len),
        slot => params.colors[slot],
    };
    for (i, pixel) in pixels.iter_mut().enumerate() {
        // Ahead of the wipe, the previous color still shows.
        let shown = if i <= reached { stage } else { stage + 2 };
        *pixel = P::from_rgb8(color(shown, i));
    }
    state.counter = state.counter.wrapping_add(1);
}

/// The palette, or the hue wheel, flowing along the strip in zones that
/// alternate direction. `count` sets how many zones, from 1 to 8.
pub fn flow<P: Pixel>(pixels: &mut [P], _state: &mut EffectState, params: &Params) {
    let len = pixels.len();
    let zones = 1 + usize::from(params.count >> 5);
    let zone_len = len.div_ceil(zones).max(1);
    let offset = phase(params.now_ms, params.rate) as u8;
    for (i, pixel) in pixels.iter_mut().enumerate() {
        let along = ((i % zone_len) * 255 / zone_len) as u8;
        let index = if (i / zone_len) % 2 == 0 {
            along.wrapping_add(offset)
        } else {
            along.wrapping_sub(offset)
        };
        *pixel = P::from_rgb8(params.wheel(index));
    }
}

/// Every other pixel crossfading between the primary color and `colors[1]`,
/// like the lights at a level crossing. `smoothness` sets how gradual the
/// crossover is: a sharp swap at `0`, a slow fade at `255`.
pub fn railway<P: Pixel>(pixels: &mut [P], _state: &mut EffectState, params: &Params) {
    let len = pixels.len();
    let wave = i32::from(triwave8(phase(params.now_ms, params.rate) as u8));
    // The less smooth, the steeper the swing through the middle.
    let steepness = 1 + i32::from(255 - params.smoothness) / 16;
    let level = (128 + (wave - 128) * steepness).clamp(0, 255) as u8;
    for (i, pixel) in pixels.iter_mut().enumerate() {
        let level = if i % 2 == 0 { level } else { 255 - level };
        *pixel = P::from_rgb8(color_blend(
            params.colors[1],
            params.primary_at(i, len),
            level,
        ));
    }
}
