use smart_leds_trait::RGB8;

use crate::params::Params;
use crate::pixel::Pixel;
use crate::segment::EffectState;
use crate::utils::{BLACK, fade_out, next_rand, scaled};

/// Simulates fireworks: pixels fade and random bursts of `colors[0]` appear.
/// Higher intensity fades bursts faster.
pub fn fireworks<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    let len = pixels.len() as u32;
    fade_out(pixels, BLACK, scaled(128, params.intensity));

    let num_bursts = ((len / 20).max(1)) as usize;
    let mut rng = state.aux;
    for _ in 0..num_bursts {
        rng = next_rand(rng);
        if rng % 10 == 0 {
            rng = next_rand(rng);
            let idx = (rng % len) as usize;
            pixels[idx] = P::from_rgb8(params.colors[0]);
        }
    }
    state.aux = rng;
}

/// Each pixel is independently dimmed by a random amount to simulate a flame.
/// `rev_intensity` controls flicker depth: higher = gentler flicker. Intensity
/// deepens it further.
pub fn fire_flicker_intensity<P: Pixel>(
    pixels: &mut [P],
    state: &mut EffectState,
    params: &Params,
    rev_intensity: u8,
) {
    let c = params.colors[0];
    let max_lum = c.r.max(c.g).max(c.b);
    let lum = scaled(max_lum / rev_intensity.max(1), params.intensity).max(1);

    let mut rng = state.aux;
    for pixel in pixels.iter_mut() {
        rng = next_rand(rng);
        let flicker = (rng % lum as u32) as u8;
        *pixel = P::from_rgb8(RGB8 {
            r: c.r.saturating_sub(flicker),
            g: c.g.saturating_sub(flicker),
            b: c.b.saturating_sub(flicker),
        });
    }
    state.aux = rng;
}

pub fn fire_flicker<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    fire_flicker_intensity(pixels, state, params, 3);
}

pub fn fire_flicker_soft<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    fire_flicker_intensity(pixels, state, params, 6);
}

pub fn fire_flicker_intense<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    fire_flicker_intensity(pixels, state, params, 1);
}
