use smart_leds_trait::RGB8;

use crate::segment::{EffectConfig, EffectState};

pub fn rainbow_cycle(pixels: &mut [RGB8], state: &mut EffectState, _config: &EffectConfig) {
    let len = pixels.len() as u32;
    for (i, pixel) in pixels.iter_mut().enumerate() {
        let hue = state.counter.wrapping_add(i as u32 * 256 / len) as u8;
        *pixel = wheel(hue);
    }
    state.counter = state.counter.wrapping_add(1);
}

/// Maps 0–255 to a position on the RGB color wheel.
fn wheel(pos: u8) -> RGB8 {
    let pos = 255 - pos;
    if pos < 85 {
        RGB8 {
            r: 255 - pos * 3,
            g: 0,
            b: pos * 3,
        }
    } else if pos < 170 {
        let pos = pos - 85;
        RGB8 {
            r: 0,
            g: pos * 3,
            b: 255 - pos * 3,
        }
    } else {
        let pos = pos - 170;
        RGB8 {
            r: pos * 3,
            g: 255 - pos * 3,
            b: 0,
        }
    }
}
