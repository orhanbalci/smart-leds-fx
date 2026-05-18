use smart_leds_trait::RGB8;

use crate::segment::{EffectConfig, EffectState};
use crate::utils::{color_wheel, fade_out, next_rand, rand_wheel_index, BLACK};

pub fn twinkle(pixels: &mut [RGB8], state: &mut EffectState, config: &EffectConfig) {
    let len = pixels.len() as u32;

    if state.counter == 0 {
        for p in pixels.iter_mut() {
            *p = config.colors[1];
        }
        let min = len / 4 + 1;
        let rng = next_rand(state.aux);
        state.aux = rng;
        state.counter = min + (rng % min);
    }

    let rng = next_rand(state.aux);
    state.aux = rng;
    pixels[(rng % len) as usize] = config.colors[0];
    state.counter = state.counter.saturating_sub(1);
}

/// Twinkle with a random wheel color for each new set.
pub fn twinkle_random(pixels: &mut [RGB8], state: &mut EffectState, config: &EffectConfig) {
    if state.counter == 0 {
        // Pick a new random foreground color.
        let rng = next_rand(state.aux);
        state.aux = rng;
        // Store wheel index in high byte of counter (reuse it as color storage).
        // We set counter = packed: low 24 bits = remaining count, high 8 bits = wheel idx.
        let wheel_idx = rng as u8;
        let len = pixels.len() as u32;
        let min = len / 4 + 1;
        let rng2 = next_rand(rng);
        state.aux = rng2;
        let count = min + (rng2 % min);
        state.counter = count | ((wheel_idx as u32) << 24);
        for p in pixels.iter_mut() {
            *p = config.colors[1];
        }
    }

    let wheel_idx = (state.counter >> 24) as u8;
    let fg = color_wheel(wheel_idx);
    let rng = next_rand(state.aux);
    state.aux = rng;
    pixels[(rng % pixels.len() as u32) as usize] = fg;
    let count = state.counter & 0x00FF_FFFF;
    state.counter = (count.saturating_sub(1)) | ((wheel_idx as u32) << 24);
}

pub fn twinkle_fade(pixels: &mut [RGB8], state: &mut EffectState, config: &EffectConfig) {
    fade_out(pixels, BLACK, 64);

    let rng = next_rand(state.aux);
    state.aux = next_rand(rng);
    if rng % 3 == 0 {
        let idx = (state.aux % pixels.len() as u32) as usize;
        pixels[idx] = config.colors[0];
    }
}

/// Twinkle-fade with a random wheel color on each new sparkle.
pub fn twinkle_fade_random(pixels: &mut [RGB8], state: &mut EffectState, _config: &EffectConfig) {
    fade_out(pixels, BLACK, 64);

    let rng = next_rand(state.aux);
    state.aux = next_rand(rng);
    if rng % 3 == 0 {
        let idx = (state.aux % pixels.len() as u32) as usize;
        pixels[idx] = color_wheel(rng as u8);
    }
}

pub fn sparkle(pixels: &mut [RGB8], state: &mut EffectState, config: &EffectConfig) {
    let len = pixels.len() as u32;

    if state.aux == 0 {
        for p in pixels.iter_mut() {
            *p = config.colors[0];
        }
    } else {
        pixels[state.counter as usize] = config.colors[0];
    }

    let rng = next_rand(state.aux);
    state.aux = rng;
    let idx = (rng % len) as usize;
    pixels[idx] = config.colors[1];
    state.counter = idx as u32;
}

/// Background `colors[0]`, sparkle with WHITE.
pub fn flash_sparkle(pixels: &mut [RGB8], state: &mut EffectState, config: &EffectConfig) {
    let len = pixels.len() as u32;

    if state.aux == 0 {
        for p in pixels.iter_mut() {
            *p = config.colors[0];
        }
    } else {
        pixels[state.counter as usize] = config.colors[0];
    }

    let rng = next_rand(state.aux);
    state.aux = rng;
    let idx = (rng % len) as usize;
    pixels[idx] = crate::utils::WHITE;
    state.counter = idx as u32;
}

/// Random wheel color, advance color every full_len / 4 calls.
pub fn sparkle_random(pixels: &mut [RGB8], state: &mut EffectState, _config: &EffectConfig) {
    let len = pixels.len() as u32;

    // aux: low byte = wheel_idx, next byte = sparkle_pos, high 2 bytes = rng
    let wheel_idx = (state.aux & 0xFF) as u8;
    let prev_pos = ((state.aux >> 8) & 0xFF) as usize;
    let rng = state.aux >> 16;

    if state.counter == 0 {
        let (new_idx, new_rng) = rand_wheel_index(wheel_idx, rng);
        // Rebuild background
        let bg = color_wheel(new_idx.wrapping_add(128));
        for p in pixels.iter_mut() { *p = bg; }
        let rng2 = next_rand(new_rng);
        let new_pos = (rng2 % len) as usize;
        pixels[new_pos] = color_wheel(new_idx);
        state.aux = new_idx as u32 | ((new_pos as u32) << 8) | (rng2 << 16);
    } else {
        // Restore previous sparkle
        pixels[prev_pos] = color_wheel(wheel_idx.wrapping_add(128));
        let new_rng = next_rand(rng);
        let new_pos = (new_rng % len) as usize;
        pixels[new_pos] = color_wheel(wheel_idx);
        state.aux = wheel_idx as u32 | ((new_pos as u32) << 8) | (new_rng << 16);
    }
    state.counter = state.counter.wrapping_add(1);
}
