use smart_leds_trait::RGB8;

use crate::segment::{EffectConfig, EffectState};
use crate::utils::{WHITE, color_blend, color_wheel, next_rand};

pub fn static_color(pixels: &mut [RGB8], _state: &mut EffectState, config: &EffectConfig) {
    for pixel in pixels.iter_mut() {
        *pixel = config.colors[0];
    }
}

pub fn blink(pixels: &mut [RGB8], state: &mut EffectState, config: &EffectConfig) {
    let color = if state.counter % 2 == 0 {
        config.colors[0]
    } else {
        config.colors[1]
    };
    for pixel in pixels.iter_mut() {
        *pixel = color;
    }
    state.counter = state.counter.wrapping_add(1);
}

/// Like blink but the "on" color cycles through the hue wheel each cycle.
pub fn blink_rainbow(pixels: &mut [RGB8], state: &mut EffectState, config: &EffectConfig) {
    let color = if state.counter % 2 == 0 {
        color_wheel((state.counter << 2) as u8)
    } else {
        config.colors[1]
    };
    for pixel in pixels.iter_mut() {
        *pixel = color;
    }
    state.counter = state.counter.wrapping_add(1);
}

pub fn breath(pixels: &mut [RGB8], state: &mut EffectState, config: &EffectConfig) {
    let level = breath_level(state.counter as u8);
    let c = config.colors[0];
    let color = RGB8 {
        r: (c.r as u16 * level as u16 / 255) as u8,
        g: (c.g as u16 * level as u16 / 255) as u8,
        b: (c.b as u16 * level as u16 / 255) as u8,
    };
    for pixel in pixels.iter_mut() {
        *pixel = color;
    }
    state.counter = state.counter.wrapping_add(1);
}

/// All LEDs cycle through a single solid hue.
pub fn rainbow(pixels: &mut [RGB8], state: &mut EffectState, _config: &EffectConfig) {
    let color = color_wheel(state.counter as u8);
    for pixel in pixels.iter_mut() {
        *pixel = color;
    }
    state.counter = state.counter.wrapping_add(1);
}

/// Smooth fade between `colors[1]` and `colors[0]`.
pub fn fade(pixels: &mut [RGB8], state: &mut EffectState, config: &EffectConfig) {
    let lum = state.counter as u16;
    let lum = if lum > 255 { 511 - lum } else { lum };
    let color = color_blend(config.colors[1], config.colors[0], lum as u8);
    for pixel in pixels.iter_mut() {
        *pixel = color;
    }
    state.counter = (state.counter + 4) % 512;
}

/// Background `colors[0]` with 8 random white sparkles per frame.
pub fn hyper_sparkle(pixels: &mut [RGB8], state: &mut EffectState, config: &EffectConfig) {
    for pixel in pixels.iter_mut() {
        *pixel = config.colors[0];
    }
    let mut rng = state.aux;
    for _ in 0..8 {
        rng = next_rand(rng);
        let idx = (rng % pixels.len() as u32) as usize;
        pixels[idx] = WHITE;
    }
    state.aux = rng;
}

/// Strobe with N rapid flashes then a pause. N is derived from `config.speed`.
pub fn multi_strobe(pixels: &mut [RGB8], state: &mut EffectState, config: &EffectConfig) {
    let count = 2 * (config.speed as u32 / 100 + 1);
    if state.counter < count {
        let color = if state.counter % 2 == 0 {
            config.colors[0]
        } else {
            config.colors[1]
        };
        for pixel in pixels.iter_mut() {
            *pixel = color;
        }
    } else {
        for pixel in pixels.iter_mut() {
            *pixel = config.colors[1];
        }
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
