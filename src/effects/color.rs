use smart_leds_trait::RGB8;

use crate::params::Params;
use crate::pixel::Pixel;
use crate::segment::EffectState;
use crate::utils::{WHITE, color_blend, fill, fill_primary, fill_with, next_rand};

pub fn static_color<P: Pixel>(pixels: &mut [P], _state: &mut EffectState, params: &Params) {
    fill_primary(pixels, params);
}

pub fn blink<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    if state.counter % 2 == 0 {
        fill_primary(pixels, params);
    } else {
        fill(pixels, params.colors[1]);
    }
    state.counter = state.counter.wrapping_add(1);
}

/// Like blink but the "on" color cycles through the hue wheel each cycle.
pub fn blink_rainbow<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    let color = if state.counter % 2 == 0 {
        params.wheel((state.counter << 2) as u8)
    } else {
        params.colors[1]
    };
    fill(pixels, color);
    state.counter = state.counter.wrapping_add(1);
}

pub fn breath<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    let level = breath_level(state.counter as u8);
    let len = pixels.len();
    fill_with(pixels, |i| {
        let c = params.primary_at(i, len);
        RGB8 {
            r: (c.r as u16 * level as u16 / 255) as u8,
            g: (c.g as u16 * level as u16 / 255) as u8,
            b: (c.b as u16 * level as u16 / 255) as u8,
        }
    });
    state.counter = state.counter.wrapping_add(1);
}

/// All LEDs cycle through a single solid hue.
pub fn rainbow<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    fill(pixels, params.wheel(state.counter as u8));
    state.counter = state.counter.wrapping_add(1);
}

/// Smooth fade between `colors[1]` and `colors[0]`.
pub fn fade<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    let lum = state.counter as u16;
    let lum = if lum > 255 { 511 - lum } else { lum };
    let len = pixels.len();
    fill_with(pixels, |i| {
        color_blend(params.colors[1], params.primary_at(i, len), lum as u8)
    });
    state.counter = (state.counter + 4) % 512;
}

/// A single-step flash of `colors[0]` over `colors[1]`, then a pause that
/// shortens as intensity rises.
pub fn strobe<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    let period = strobe_period(params.intensity);
    if state.counter % period == 0 {
        fill_primary(pixels, params);
    } else {
        fill(pixels, params.colors[1]);
    }
    state.counter = (state.counter + 1) % period;
}

/// [`strobe`] whose flash advances around the hue wheel.
/// `state.aux` holds the hue of the next flash.
pub fn strobe_rainbow<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    let period = strobe_period(params.intensity);
    if state.counter % period == 0 {
        fill(pixels, params.wheel(state.aux as u8));
        state.aux = (state.aux + 16) & 0xFF;
    } else {
        fill(pixels, params.colors[1]);
    }
    state.counter = (state.counter + 1) % period;
}

/// Steps per flash cycle: 2 at full intensity, 9 at zero.
fn strobe_period(intensity: u8) -> u32 {
    2 + u32::from(255 - intensity) / 32
}

/// Background `colors[0]` with random white sparkles: 8 per frame at default
/// intensity.
pub fn hyper_sparkle<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    fill_primary(pixels, params);
    let mut rng = state.aux;
    for _ in 0..(params.intensity / 16).max(1) {
        rng = next_rand(rng);
        let idx = (rng % pixels.len() as u32) as usize;
        pixels[idx] = P::from_rgb8(WHITE);
    }
    state.aux = rng;
}

/// Strobe with N rapid flashes then a pause. N is derived from `params.speed`.
pub fn multi_strobe<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    let count = 2 * (params.speed as u32 / 100 + 1);
    if state.counter < count && state.counter % 2 == 0 {
        fill_primary(pixels, params);
    } else {
        fill(pixels, params.colors[1]);
    }
    state.counter = (state.counter + 1) % (count + 1);
}

/// Triangle wave: 0→254 over steps 0–127, 255→1 over steps 128–255.
fn breath_level(step: u8) -> u8 {
    if step < 128 {
        step * 2
    } else {
        255u8.saturating_sub((step - 128) * 2)
    }
}
