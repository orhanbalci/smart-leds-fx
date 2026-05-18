use smart_leds_trait::RGB8;

use crate::segment::{EffectConfig, EffectState};
use crate::utils::{BLACK, color_blend, color_wheel, fade_out, next_rand, sine8};

/// TwinkleFOX by Mark Kriegsman — deterministic per-LED sine-blended twinkle.
/// Uses a fixed-seed LCG per LED so the pattern is stable without extra state.
pub fn twinkle_fox(pixels: &mut [RGB8], state: &mut EffectState, config: &EffectConfig) {
    let c0 = config.colors[0];
    let c1 = config.colors[1];
    let c2 = config.colors[2];
    let call = state.counter as u16;

    let mut seed: u16 = 0;
    for pixel in pixels.iter_mut() {
        seed = seed.wrapping_mul(2053).wrapping_add(13849);
        let init = ((seed.wrapping_add(seed >> 8)) & 0xFF) as u8;
        seed = seed.wrapping_mul(2053).wrapping_add(13849);
        let incr = (((seed.wrapping_add(seed >> 8)) & 0x07) as u8 + 1) * 2;

        let blend_index = init.wrapping_add((call.wrapping_mul(incr as u16) & 0xFF) as u8);
        let blend_amt = sine8(blend_index);

        let is_black = |c: RGB8| c.r == 0 && c.g == 0 && c.b == 0;
        *pixel = if is_black(c0) {
            color_blend(color_wheel(init), c1, blend_amt)
        } else if !is_black(c2) && init >= 128 {
            color_blend(c2, c1, blend_amt)
        } else {
            color_blend(c0, c1, blend_amt)
        };
    }
    state.counter = state.counter.wrapping_add(1);
}

/// Fireworks + 2-pixel downward shift = falling rain.
/// `colors[0]` and `colors[2]` alternate as raindrop colors.
pub fn rain(pixels: &mut [RGB8], state: &mut EffectState, config: &EffectConfig) {
    let len = pixels.len();
    let rng = next_rand(state.aux);
    state.aux = rng;

    let rain_color = if rng & 1 == 0 {
        config.colors[0]
    } else {
        config.colors[2]
    };

    // Fade and add a random burst like fireworks.
    fade_out(pixels, BLACK, 128);
    if rng % 8 == 0 && len > 2 {
        let rng2 = next_rand(rng);
        state.aux = rng2;
        let idx = (rng2 % (len as u32 - 2) + 1) as usize;
        pixels[idx] = rain_color;
    }

    // Shift everything 2 pixels (rain falls forward).
    if len > 2 {
        pixels.copy_within(0..len - 2, 2);
        pixels[0] = BLACK;
        pixels[1] = BLACK;
    }
}

/// Two "eyes" move toward a random target, pause, then pick a new target.
/// Simplified: skips the blink (variable delay not available in fixed-step rendering).
/// `state.counter` = eye position, `state.aux`: low 16 = destination, bit 16 = settled flag.
pub fn icu(pixels: &mut [RGB8], state: &mut EffectState, config: &EffectConfig) {
    let len = pixels.len();
    let half = (len / 2).max(1);
    let pos = state.counter as usize;
    let dest = (state.aux & 0xFFFF) as usize;
    let settled = (state.aux >> 16) & 1 == 1;

    for p in pixels.iter_mut() {
        *p = BLACK;
    }

    if settled {
        // Show eyes and pick a new target after a "pause" (8 steps).
        if pos < len && pos + half < len {
            pixels[pos] = config.colors[0];
            pixels[pos + half] = config.colors[0];
        }
        let wait = (state.aux >> 17) as u32;
        if wait >= 8 {
            let rng = next_rand(state.aux as u32);
            let new_dest = (rng % half as u32) as usize;
            state.aux = (new_dest as u32) & 0xFFFF;
        } else {
            state.aux = (state.aux & !(0x7F << 17)) | ((wait + 1) << 17);
        }
        return;
    }

    // Move toward destination.
    let new_pos = if dest > pos {
        pos + 1
    } else if dest < pos {
        pos - 1
    } else {
        pos
    };
    state.counter = new_pos as u32;

    if new_pos < len && new_pos + half < len {
        pixels[new_pos] = config.colors[0];
        pixels[new_pos + half] = config.colors[0];
    }

    if new_pos == dest {
        state.aux = (dest as u32 & 0xFFFF) | (1 << 16); // mark settled
    }
}

/// Liquid filling: a droplet falls and accumulates at the bottom, then swaps colors.
/// `state.counter` = drop position, `state.aux`: low 16 = fill level, bit 16 = color swap.
pub fn filler_up(pixels: &mut [RGB8], state: &mut EffectState, config: &EffectConfig) {
    let len = pixels.len();
    let drop_pos = state.counter as usize;
    let fill_level = (state.aux & 0xFFFF) as usize;
    let swapped = (state.aux >> 16) & 1 == 1;

    let fg = if swapped {
        config.colors[1]
    } else {
        config.colors[0]
    };
    let bg = if swapped {
        config.colors[0]
    } else {
        config.colors[1]
    };

    // Background
    for p in pixels.iter_mut() {
        *p = bg;
    }
    // Accumulated fill at the bottom
    if fill_level > 0 && fill_level <= len {
        for p in pixels[len - fill_level..len].iter_mut() {
            *p = fg;
        }
    }
    // Falling drop
    let dp = drop_pos.min(len.saturating_sub(fill_level + 1));
    if dp < len {
        pixels[dp] = fg;
    }

    // Advance drop
    let next_drop = drop_pos + 1;
    if next_drop >= len.saturating_sub(fill_level) {
        // Drop landed — increase fill level
        let new_fill = fill_level + 1;
        if new_fill >= len {
            // Glass full — reset and swap colors
            state.aux = (swapped as u32 ^ 1) << 16;
        } else {
            state.aux = (new_fill as u32) | ((swapped as u32) << 16);
        }
        state.counter = 0;
    } else {
        state.counter = next_drop as u32;
    }
}

/// Smooth cross-fade between three colors in sequence.
/// With `colors[2] == BLACK`, also fades to black between each pair.
pub fn trifade(pixels: &mut [RGB8], state: &mut EffectState, config: &EffectConfig) {
    let is_black = |c: RGB8| c.r == 0 && c.g == 0 && c.b == 0;
    let use_black = is_black(config.colors[2]);

    let colors_main: [RGB8; 3] = config.colors;
    let colors_alt: [RGB8; 6] = [
        config.colors[0],
        BLACK,
        config.colors[1],
        BLACK,
        config.colors[2],
        BLACK,
    ];

    let num = if use_black { 6usize } else { 3usize };
    let idx = (state.aux as usize) % num;
    let next_idx = (idx + 1) % num;

    let (c1, c2) = if use_black {
        (colors_alt[idx], colors_alt[next_idx])
    } else {
        (colors_main[idx], colors_main[next_idx])
    };

    let blend = (state.counter & 0xFF) as u8;
    let color = color_blend(c1, c2, blend);
    for pixel in pixels.iter_mut() {
        *pixel = color;
    }

    state.counter = state.counter.wrapping_add(4);
    if state.counter & 0xFF < 4 {
        state.aux = (state.aux + 1) % num as u32;
    }
}

/// Dual heartbeat pulses expand from the center outward with a fading trail.
/// `state.counter` encodes beat phase (0 = first beat, 8 = second beat, 24+ = rest).
pub fn heartbeat(pixels: &mut [RGB8], state: &mut EffectState, config: &EffectConfig) {
    let len = pixels.len();
    let half = len / 2;

    // Shift pixels from center toward edges (heartbeat expansion).
    if half > 1 {
        pixels.copy_within(1..half, 0); // left half: shift left
        pixels.copy_within(half..len - 1, half + 1); // right half: shift right
    }

    fade_out(pixels, BLACK, 32);

    let step = state.counter;
    if step == 0 || step == 8 {
        // Beat: light up center pixels
        let size = 2.min(half);
        for p in pixels[half - size..half + size].iter_mut() {
            *p = config.colors[0];
        }
    }

    state.counter = (state.counter + 1) % 60;
}

/// Spectral fireworks: red pixels expand through the rainbow as they fade.
pub fn rainbow_fireworks(pixels: &mut [RGB8], state: &mut EffectState, _config: &EffectConfig) {
    let len = pixels.len();

    // Fade and expand each "burst" through spectral colors.
    for i in 0..len {
        let c = pixels[i];
        let faded = RGB8 {
            r: c.r / 2,
            g: c.g / 2,
            b: c.b / 2,
        };
        pixels[i] = faded;

        // When a red pixel reaches a specific fade level, spawn a neighbor of the next color.
        macro_rules! spawn_neighbor {
            ($threshold_r:expr, $color:expr) => {
                if faded.r == $threshold_r && faded.g == 0 && faded.b == 0 {
                    let neighbor = ($color) as RGB8;
                    let dist = match $threshold_r {
                        0x7F => 1usize,
                        0x3F => 2,
                        0x1F => 3,
                        0x0F => 4,
                        0x07 => 5,
                        0x03 => 6,
                        _ => 0,
                    };
                    if i >= dist {
                        pixels[i - dist] = neighbor;
                    }
                    if i + dist < len {
                        pixels[i + dist] = neighbor;
                    }
                }
            };
        }

        spawn_neighbor!(
            0x7F,
            RGB8 {
                r: 255,
                g: 127,
                b: 0
            }
        ); // orange
        spawn_neighbor!(
            0x3F,
            RGB8 {
                r: 255,
                g: 255,
                b: 0
            }
        ); // yellow
        spawn_neighbor!(0x1F, RGB8 { r: 0, g: 255, b: 0 }); // green
        spawn_neighbor!(0x0F, RGB8 { r: 0, g: 0, b: 255 }); // blue
        spawn_neighbor!(
            0x07,
            RGB8 {
                r: 75,
                g: 0,
                b: 130
            }
        ); // indigo
        spawn_neighbor!(
            0x03,
            RGB8 {
                r: 148,
                g: 0,
                b: 211
            }
        ); // violet
    }

    // Randomly ignite a new red pixel.
    let rng = next_rand(state.aux);
    state.aux = rng;
    if rng % 4 == 0 && len > 12 {
        let idx = (next_rand(rng) % (len - 12) as u32 + 6) as usize;
        pixels[idx] = RGB8 { r: 255, g: 0, b: 0 };
    }
}
