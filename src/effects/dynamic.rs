use crate::params::Params;
use crate::pixel::Pixel;
use crate::segment::EffectState;
use crate::utils::{color_blend, color_wheel, fill, next_rand, sine8};

/// All pixels a single random color, changing each call.
pub fn random_color<P: Pixel>(pixels: &mut [P], state: &mut EffectState, _params: &Params) {
    state.aux = next_rand(state.aux);
    fill(pixels, color_wheel(state.aux as u8));
}

/// One random pixel changes to a random wheel color each step.
/// On first call (counter == 0), fills the whole strip with random colors.
pub fn single_dynamic<P: Pixel>(pixels: &mut [P], state: &mut EffectState, _params: &Params) {
    if state.counter == 0 {
        let mut rng = state.aux;
        for p in pixels.iter_mut() {
            rng = next_rand(rng);
            *p = P::from_rgb8(color_wheel(rng as u8));
        }
        state.aux = rng;
    }

    let rng = next_rand(state.aux);
    let idx = (rng % pixels.len() as u32) as usize;
    let rng2 = next_rand(rng);
    pixels[idx] = P::from_rgb8(color_wheel(rng2 as u8));
    state.aux = rng2;
    state.counter = state.counter.wrapping_add(1);
}

/// Every pixel gets a new random wheel color each step.
pub fn multi_dynamic<P: Pixel>(pixels: &mut [P], state: &mut EffectState, _params: &Params) {
    let mut rng = state.aux;
    for pixel in pixels.iter_mut() {
        rng = next_rand(rng);
        *pixel = P::from_rgb8(color_wheel(rng as u8));
    }
    state.aux = rng;
}

/// Smooth sine-wave luminance gradient flowing along the strip.
/// Blends `colors[0]` and `colors[1]` per-pixel using a sine envelope.
pub fn running_lights<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    let len = pixels.len();
    let sine_incr = ((256 / len) as u8).max(1);

    for (i, pixel) in pixels.iter_mut().enumerate() {
        let phase = ((i as u32).wrapping_add(state.counter) * sine_incr as u32) as u8;
        let lum = sine8(phase);
        *pixel = P::from_rgb8(color_blend(params.colors[0], params.colors[1], lum));
    }
    state.counter = state.counter.wrapping_add(1);
}

/// Randomly dissolves pixels one-by-one from `colors[current]` to the next color.
/// Cycles through `colors[0]`, `colors[1]`, `colors[2]`.
pub fn block_dissolve<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    let len = pixels.len() as u32;
    // The low two bits hold the colour index; a seeded `aux` may hold 3.
    let color_idx = ((state.aux & 0x3) % 3) as usize;
    let target = params.colors[color_idx];

    let rng = next_rand(state.aux >> 2);
    let idx = (rng % len) as usize;

    if pixels[idx].to_rgb8() != target {
        pixels[idx] = P::from_rgb8(target);
        state.aux = color_idx as u32 | (rng << 2);
        return;
    }

    // Scan for any non-target pixel.
    let start = (rng % len) as usize;
    for offset in 0..pixels.len() {
        let i = (start + offset) % pixels.len();
        if pixels[i].to_rgb8() != target {
            pixels[i] = P::from_rgb8(target);
            state.aux = color_idx as u32 | (next_rand(rng) << 2);
            return;
        }
    }

    // All pixels are target — advance to next color.
    let next_idx = (color_idx + 1) % 3;
    state.aux = next_idx as u32 | (next_rand(rng) << 2);
}
