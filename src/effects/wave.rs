//! Effects shaped by the clock and by named settings rather than by a step
//! count.
//!
//! The time-driven ones — Sine, Bpm and Wavesins — draw each frame from
//! [`Params::now_ms`], so they move smoothly however often they are stepped.

use lib8tion::{beatsin8, sin8};
use smart_leds_trait::RGB8;

use crate::params::Params;
use crate::pixel::Pixel;
use crate::segment::EffectState;

/// `color` at brightness `level`, where `255` leaves it unchanged.
fn dim(color: RGB8, level: u8) -> RGB8 {
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
fn phase(now_ms: u32, rate: u8) -> u32 {
    now_ms.wrapping_mul(u32::from(rate) + 1) >> 10
}

/// Bands of the hue wheel, or the palette, ripple along the strip under a sine
/// wave of brightness. `rate` sets how fast they move; `scale` how tightly
/// the bands repeat.
pub fn sine<P: Pixel>(pixels: &mut [P], _state: &mut EffectState, params: &Params) {
    let phase = phase(params.now_ms, params.rate);
    let (offset, drift) = (phase as u8, (phase >> 1) as u8);
    let step = u32::from(params.scale / 16 + 1);
    for (i, pixel) in pixels.iter_mut().enumerate() {
        let angle = (i as u32 * step) as u8;
        let level = sin8(angle.wrapping_add(offset));
        *pixel = P::from_rgb8(dim(params.wheel(angle.wrapping_add(drift)), level));
    }
}

/// The hue wheel, or the palette, spread along the strip while a pulse of
/// brightness travels through it to a beat. `rate` sets the tempo, from 20 to
/// 105 beats a minute.
pub fn bpm<P: Pixel>(pixels: &mut [P], _state: &mut EffectState, params: &Params) {
    let len = pixels.len();
    let tempo = 20 + u16::from(params.rate) / 3;
    let hue = (params.now_ms / 32) as u8;
    for (i, pixel) in pixels.iter_mut().enumerate() {
        let along = (i * 256 / len) as u8;
        let level = beatsin8(tempo, 64, 255, 0, 0u8.wrapping_sub(along), params.now_ms);
        *pixel = P::from_rgb8(dim(params.wheel(hue.wrapping_add(along)), level));
    }
}

/// Lights the share of the strip that `fill` asks for, from none at `0` to all
/// at `255`, growing or shrinking one pixel per step toward it. Lit pixels
/// follow the palette, or show the primary color alone with `one_color`; the
/// rest show `colors[1]`. `state.counter` holds how many are lit.
pub fn percent<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    let len = pixels.len();
    let target = (len * usize::from(params.fill) + 127) / 255;
    let shown = (state.counter as usize).min(len);
    let lit = match shown.cmp(&target) {
        core::cmp::Ordering::Less => shown + 1,
        core::cmp::Ordering::Greater => shown - 1,
        core::cmp::Ordering::Equal => shown,
    };
    for (i, pixel) in pixels.iter_mut().enumerate() {
        *pixel = P::from_rgb8(if i >= lit {
            params.colors[1]
        } else if params.one_color {
            params.colors[0]
        } else {
            params.primary_at(i, len)
        });
    }
    state.counter = lit as u32;
}

/// Colors sway back and forth along the palette, or the hue wheel, while
/// brightness ripples across the strip.
///
/// `palette_start` is where the colors start, `palette_span` how far they
/// sway from there, and `palette_step` how far apart neighbouring pixels are.
/// `variation` sets how deep the brightness ripples, and `rate` how fast
/// everything moves.
pub fn wavesins<P: Pixel>(pixels: &mut [P], _state: &mut EffectState, params: &Params) {
    let phase = phase(params.now_ms, params.rate);
    let (sway, ripple) = (phase as u8, (phase >> 1) as u8);
    let step = u32::from(params.palette_step);
    for (i, pixel) in pixels.iter_mut().enumerate() {
        let angle = sway.wrapping_add((i as u32 * step) as u8);
        let offset = (u16::from(sin8(angle)) * u16::from(params.palette_span) / 255) as u8;
        let color = params.wheel(params.palette_start.wrapping_add(offset));

        let wave = sin8(ripple.wrapping_add((i as u32 * 3) as u8));
        let dip = (u16::from(255 - wave) * u16::from(params.variation) / 255) as u8;
        *pixel = P::from_rgb8(dim(color, 255 - dip));
    }
}

/// Stripes that stand still: `width` sets each stripe of the primary color
/// (following the palette), from 1 to 16 pixels, and `gap` the `colors[1]`
/// between them, from 0 to 15.
pub fn solid_pattern<P: Pixel>(pixels: &mut [P], _state: &mut EffectState, params: &Params) {
    let len = pixels.len();
    let width = 1 + usize::from(params.width >> 4);
    let period = width + usize::from(params.gap >> 4);
    for (i, pixel) in pixels.iter_mut().enumerate() {
        *pixel = P::from_rgb8(if i % period < width {
            params.primary_at(i, len)
        } else {
            params.colors[1]
        });
    }
}
