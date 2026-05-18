use smart_leds_trait::RGB8;

use crate::segment::{EffectConfig, EffectState};

pub fn static_color(pixels: &mut [RGB8], _state: &mut EffectState, config: &EffectConfig) {
    for pixel in pixels.iter_mut() {
        *pixel = config.colors[0];
    }
}

pub fn blink(pixels: &mut [RGB8], state: &mut EffectState, config: &EffectConfig) {
    let color = if state.counter % 2 == 0 {
        config.colors[0]
    } else {
        RGB8 { r: 0, g: 0, b: 0 }
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

/// Triangle wave: 0→255 over the first 128 steps, 255→0 over the next 128.
fn breath_level(step: u8) -> u8 {
    if step < 128 {
        step.saturating_mul(2)
    } else {
        (255u16 - step as u16 * 2).min(255) as u8
    }
}
