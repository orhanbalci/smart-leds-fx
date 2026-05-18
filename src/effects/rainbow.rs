use smart_leds_trait::RGB8;

use crate::segment::{EffectConfig, EffectState};
use crate::utils::color_wheel;

pub fn rainbow_cycle(pixels: &mut [RGB8], state: &mut EffectState, _config: &EffectConfig) {
    let len = pixels.len() as u32;
    for (i, pixel) in pixels.iter_mut().enumerate() {
        let hue = state.counter.wrapping_add(i as u32 * 256 / len) as u8;
        *pixel = color_wheel(hue);
    }
    state.counter = state.counter.wrapping_add(1);
}
