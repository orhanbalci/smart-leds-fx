use smart_leds_trait::RGB8;

use crate::segment::{EffectConfig, EffectState};
use crate::utils::{BLACK, fade_out, next_rand};

/// Simulates fireworks: pixels fade and random bursts of `colors[0]` appear.
pub fn fireworks(pixels: &mut [RGB8], state: &mut EffectState, config: &EffectConfig) {
    let len = pixels.len() as u32;
    fade_out(pixels, BLACK, 128);

    let num_bursts = ((len / 20).max(1)) as usize;
    let mut rng = state.aux;
    for _ in 0..num_bursts {
        rng = next_rand(rng);
        if rng % 10 == 0 {
            rng = next_rand(rng);
            let idx = (rng % len) as usize;
            pixels[idx] = config.colors[0];
        }
    }
    state.aux = rng;
}

/// Each pixel is independently dimmed by a random amount to simulate a flame.
/// `rev_intensity` controls flicker depth: higher = gentler flicker.
pub fn fire_flicker_intensity(
    pixels: &mut [RGB8],
    state: &mut EffectState,
    config: &EffectConfig,
    rev_intensity: u8,
) {
    let c = config.colors[0];
    let max_lum = c.r.max(c.g).max(c.b);
    let lum = (max_lum / rev_intensity.max(1)).max(1);

    let mut rng = state.aux;
    for pixel in pixels.iter_mut() {
        rng = next_rand(rng);
        let flicker = (rng % lum as u32) as u8;
        *pixel = RGB8 {
            r: c.r.saturating_sub(flicker),
            g: c.g.saturating_sub(flicker),
            b: c.b.saturating_sub(flicker),
        };
    }
    state.aux = rng;
}

pub fn fire_flicker(pixels: &mut [RGB8], state: &mut EffectState, config: &EffectConfig) {
    fire_flicker_intensity(pixels, state, config, 3);
}

pub fn fire_flicker_soft(pixels: &mut [RGB8], state: &mut EffectState, config: &EffectConfig) {
    fire_flicker_intensity(pixels, state, config, 6);
}

pub fn fire_flicker_intense(pixels: &mut [RGB8], state: &mut EffectState, config: &EffectConfig) {
    fire_flicker_intensity(pixels, state, config, 1);
}
