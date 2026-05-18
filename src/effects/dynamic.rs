use smart_leds_trait::RGB8;

use crate::segment::{EffectConfig, EffectState};
use crate::utils::{color_blend, color_wheel, next_rand, sine8};

/// All pixels a single random color, changing each call.
pub fn random_color(pixels: &mut [RGB8], state: &mut EffectState, _config: &EffectConfig) {
    state.aux = next_rand(state.aux);
    let color = color_wheel(state.aux as u8);
    for pixel in pixels.iter_mut() {
        *pixel = color;
    }
}

/// One random pixel changes to a random wheel color each step.
/// On first call (counter == 0), fills the whole strip with random colors.
pub fn single_dynamic(pixels: &mut [RGB8], state: &mut EffectState, _config: &EffectConfig) {
    if state.counter == 0 {
        let mut rng = state.aux;
        for p in pixels.iter_mut() {
            rng = next_rand(rng);
            *p = color_wheel(rng as u8);
        }
        state.aux = rng;
    }

    let rng = next_rand(state.aux);
    let idx = (rng % pixels.len() as u32) as usize;
    let rng2 = next_rand(rng);
    pixels[idx] = color_wheel(rng2 as u8);
    state.aux = rng2;
    state.counter = state.counter.wrapping_add(1);
}

/// Every pixel gets a new random wheel color each step.
pub fn multi_dynamic(pixels: &mut [RGB8], state: &mut EffectState, _config: &EffectConfig) {
    let mut rng = state.aux;
    for pixel in pixels.iter_mut() {
        rng = next_rand(rng);
        *pixel = color_wheel(rng as u8);
    }
    state.aux = rng;
}

/// Smooth sine-wave luminance gradient flowing along the strip.
/// Blends `colors[0]` and `colors[1]` per-pixel using a sine envelope.
pub fn running_lights(pixels: &mut [RGB8], state: &mut EffectState, config: &EffectConfig) {
    let len = pixels.len();
    let sine_incr = ((256 / len) as u8).max(1);

    for (i, pixel) in pixels.iter_mut().enumerate() {
        let phase = ((i as u32).wrapping_add(state.counter) * sine_incr as u32) as u8;
        let lum = sine8(phase);
        *pixel = color_blend(config.colors[0], config.colors[1], lum);
    }
    state.counter = state.counter.wrapping_add(1);
}

/// Randomly dissolves pixels one-by-one from `colors[current]` to the next color.
/// Cycles through `colors[0]`, `colors[1]`, `colors[2]`.
pub fn block_dissolve(pixels: &mut [RGB8], state: &mut EffectState, config: &EffectConfig) {
    let len = pixels.len() as u32;
    let color_idx = (state.aux & 0x3) as usize;
    let target = config.colors[color_idx];

    let rng = next_rand(state.aux >> 2);
    let idx = (rng % len) as usize;

    if pixels[idx] != target {
        pixels[idx] = target;
        state.aux = (state.aux & 0x3) | (rng << 2);
        return;
    }

    // Scan for any non-target pixel.
    let start = (rng % len) as usize;
    for offset in 0..pixels.len() {
        let i = (start + offset) % pixels.len();
        if pixels[i] != target {
            pixels[i] = target;
            state.aux = (state.aux & 0x3) | (next_rand(rng) << 2);
            return;
        }
    }

    // All pixels are target — advance to next color.
    let next_idx = (color_idx + 1) % 3;
    state.aux = next_idx as u32 | (next_rand(rng) << 2);
}
