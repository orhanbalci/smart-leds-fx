use crate::params::Params;
use crate::pixel::Pixel;
use crate::segment::EffectState;
use crate::utils::{BLACK, WHITE, color_wheel, fill, next_rand, rand_wheel_index};

pub fn color_wipe<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    let len = pixels.len();
    let step = state.counter as usize % (len * 2);
    if step < len {
        pixels[step] = P::from_rgb8(params.colors[0]);
    } else {
        pixels[step - len] = P::from_rgb8(params.colors[1]);
    }
    state.counter = state.counter.wrapping_add(1);
}

/// color_wipe that picks a new random wheel color each cycle.
/// `state.aux` packs the wheel index (low byte) and rng seed (high 3 bytes).
pub fn color_wipe_random<P: Pixel>(pixels: &mut [P], state: &mut EffectState, _params: &Params) {
    let len = pixels.len();
    let step = state.counter as usize % (len * 2);

    if step == 0 {
        let idx = (state.aux & 0xFF) as u8;
        let rng = state.aux >> 8;
        let (new_idx, new_rng) = rand_wheel_index(idx, rng);
        state.aux = new_idx as u32 | (new_rng << 8);
    }

    let color = P::from_rgb8(color_wheel((state.aux & 0xFF) as u8));
    if step < len {
        pixels[step] = color;
    } else {
        pixels[step - len] = color;
    }
    state.counter = state.counter.wrapping_add(1);
}

/// Alternating forward and reverse color wipes with a new random color each sweep.
pub fn color_sweep_random<P: Pixel>(pixels: &mut [P], state: &mut EffectState, _params: &Params) {
    let len = pixels.len();
    let step = state.counter as usize % (len * 2);
    let forward = (state.counter / (len as u32 * 2)) % 2 == 0;

    if step == 0 {
        let idx = (state.aux & 0xFF) as u8;
        let rng = state.aux >> 8;
        let (new_idx, new_rng) = rand_wheel_index(idx, rng);
        state.aux = new_idx as u32 | (new_rng << 8);
    }

    let color = P::from_rgb8(color_wheel((state.aux & 0xFF) as u8));
    let pos = if forward {
        step.min(len - 1)
    } else {
        len - 1 - step.min(len - 1)
    };

    if step < len {
        pixels[pos] = color;
    } else {
        let off_pos = if forward {
            step - len
        } else {
            len - 1 - (step - len).min(len - 1)
        };
        if off_pos < len {
            pixels[off_pos] = P::from_rgb8(BLACK);
        }
    }
    state.counter = state.counter.wrapping_add(1);
}

/// A single dot bouncing back and forth.
/// `colors[0]` = dot, `colors[1]` = background.
pub fn scan<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    let len = pixels.len();
    let pos = state.counter as usize;
    let going_forward = state.aux == 0;

    fill(pixels, params.colors[1]);
    if pos < len {
        pixels[pos] = P::from_rgb8(params.colors[0]);
    }

    if going_forward {
        state.counter += 1;
        if state.counter as usize >= len - 1 {
            state.aux = 1;
        }
    } else {
        state.counter = state.counter.saturating_sub(1);
        if state.counter == 0 {
            state.aux = 0;
        }
    }
}

/// Two dots mirrored from each end, converging and diverging.
pub fn dual_scan<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    let len = pixels.len();
    let pos = state.counter as usize;
    let going_forward = state.aux == 0;

    fill(pixels, params.colors[1]);
    if pos < len {
        pixels[pos] = P::from_rgb8(params.colors[0]);
        pixels[len - 1 - pos] = P::from_rgb8(params.colors[0]);
    }

    if going_forward {
        state.counter += 1;
        if state.counter as usize >= len / 2 {
            state.aux = 1;
        }
    } else {
        state.counter = state.counter.saturating_sub(1);
        if state.counter == 0 {
            state.aux = 0;
        }
    }
}

pub fn tricolor_chase<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    let offset = state.counter as usize;
    for (i, pixel) in pixels.iter_mut().enumerate() {
        *pixel = P::from_rgb8(params.colors[(i + offset) % 3]);
    }
    state.counter = state.counter.wrapping_add(1);
}

/// Theater chase with a rotating hue on the leading dot.
/// `state.aux` = current wheel index.
pub fn theater_chase_rainbow<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    state.aux = state.aux.wrapping_add(1) & 0xFF;
    let color = color_wheel(state.aux as u8);
    let offset = state.counter as usize % 3;
    for (i, pixel) in pixels.iter_mut().enumerate() {
        *pixel = P::from_rgb8(if (i + offset) % 3 == 0 {
            color
        } else {
            params.colors[1]
        });
    }
    state.counter = state.counter.wrapping_add(1);
}

/// Three-color chase: block of each color advances one step per call.
pub fn chase<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    let len = pixels.len();
    let dot = (len / 8).max(1);
    let offset = state.counter as usize % len;
    for (i, pixel) in pixels.iter_mut().enumerate() {
        let pos = (i + offset) % len;
        *pixel = P::from_rgb8(if pos < dot {
            params.colors[0]
        } else if pos < dot * 2 {
            params.colors[1]
        } else {
            params.colors[2]
        });
    }
    state.counter = state.counter.wrapping_add(1);
}

/// Chase with a random hue that changes each full cycle.
/// Packs wheel_idx in low byte of `state.aux`, rng seed in high 3 bytes.
pub fn chase_random<P: Pixel>(pixels: &mut [P], state: &mut EffectState, _params: &Params) {
    let len = pixels.len();
    let dot = (len / 8).max(1);
    let offset = state.counter as usize % len;

    if offset == 0 {
        let idx = (state.aux & 0xFF) as u8;
        let rng = state.aux >> 8;
        let (new_idx, new_rng) = rand_wheel_index(idx, rng);
        state.aux = new_idx as u32 | (new_rng << 8);
    }

    let color = color_wheel((state.aux & 0xFF) as u8);
    for (i, pixel) in pixels.iter_mut().enumerate() {
        let pos = (i + offset) % len;
        *pixel = P::from_rgb8(if pos < dot { color } else { WHITE });
    }
    state.counter = state.counter.wrapping_add(1);
}

/// White dot chasing a per-pixel rainbow background.
pub fn chase_rainbow_white<P: Pixel>(pixels: &mut [P], state: &mut EffectState, _params: &Params) {
    let len = pixels.len();
    let dot = (len / 8).max(1);
    let offset = state.counter as usize % len;
    let call = (state.aux & 0xFF) as u8;

    for (i, pixel) in pixels.iter_mut().enumerate() {
        let pos = (i + offset) % len;
        *pixel = P::from_rgb8(if pos < dot {
            WHITE
        } else {
            color_wheel(((i * 256 / len) as u8).wrapping_add(call))
        });
    }
    state.counter = state.counter.wrapping_add(1);
    state.aux = (state.aux & !0xFF) | ((call.wrapping_add(1)) as u32);
}

/// Rainbow-colored dot chasing on a white background.
pub fn chase_rainbow<P: Pixel>(pixels: &mut [P], state: &mut EffectState, _params: &Params) {
    let len = pixels.len();
    let dot = (len / 8).max(1);
    let color_sep = (256 / len) as u8;
    let offset = state.counter as usize % len;
    let call = (state.aux & 0xFF) as u8;

    for (i, pixel) in pixels.iter_mut().enumerate() {
        let pos = (i + offset) % len;
        *pixel = P::from_rgb8(if pos < dot {
            color_wheel((i as u8 * color_sep).wrapping_add(call))
        } else {
            WHITE
        });
    }
    state.counter = state.counter.wrapping_add(1);
    state.aux = (state.aux & !0xFF) | ((call.wrapping_add(1)) as u32);
}

/// Rainbow-colored dot chasing on a black background.
pub fn chase_blackout_rainbow<P: Pixel>(
    pixels: &mut [P],
    state: &mut EffectState,
    _params: &Params,
) {
    let len = pixels.len();
    let dot = (len / 8).max(1);
    let color_sep = (256 / len) as u8;
    let offset = state.counter as usize % len;
    let call = (state.aux & 0xFF) as u8;

    for (i, pixel) in pixels.iter_mut().enumerate() {
        let pos = (i + offset) % len;
        *pixel = P::from_rgb8(if pos < dot {
            color_wheel((i as u8 * color_sep).wrapping_add(call))
        } else {
            BLACK
        });
    }
    state.counter = state.counter.wrapping_add(1);
    state.aux = (state.aux & !0xFF) | ((call.wrapping_add(1)) as u32);
}

/// A white flash strobe that advances position after 4 flash frames.
/// `colors[0]` = background, `colors[1]` = flash color (usually WHITE).
pub fn chase_flash<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    let len = pixels.len();
    let pos = (state.counter / 4) as usize % len;
    let flash = state.counter % 4;

    fill(pixels, params.colors[0]);
    if flash == 0 || flash == 2 {
        pixels[pos] = P::from_rgb8(params.colors[1]);
    }
    state.counter = state.counter.wrapping_add(1);
}

/// chase_flash with the flash color advancing through random wheel hues.
/// Packs wheel_idx in low byte of `state.aux`, rng in high 3 bytes.
pub fn chase_flash_random<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    let len = pixels.len();
    let pos = (state.counter / 4) as usize % len;
    let flash = state.counter % 4;

    if state.counter % (len as u32 * 4) == 0 {
        let idx = (state.aux & 0xFF) as u8;
        let rng = state.aux >> 8;
        let (new_idx, new_rng) = rand_wheel_index(idx, rng);
        state.aux = new_idx as u32 | (new_rng << 8);
    }

    fill(pixels, params.colors[0]);
    if flash == 0 || flash == 2 {
        pixels[pos] = P::from_rgb8(color_wheel((state.aux & 0xFF) as u8));
    }
    state.counter = state.counter.wrapping_add(1);
}

/// Alternating color bands that shift along the strip.
pub fn running<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    let len = pixels.len();
    let color = if state.counter as usize & 2 != 0 {
        params.colors[0]
    } else {
        params.colors[1]
    };
    pixels.copy_within(0..len - 1, 1);
    pixels[0] = P::from_rgb8(color);
    state.counter = state.counter.wrapping_add(1);
}

/// Running with a random wheel color that changes every 4 pixels.
/// Packs wheel_idx in low byte of `state.aux`, rng seed in upper 3 bytes.
pub fn running_random<P: Pixel>(pixels: &mut [P], state: &mut EffectState, _params: &Params) {
    let len = pixels.len();

    if state.counter % 4 == 0 {
        let idx = (state.aux & 0xFF) as u8;
        let rng = state.aux >> 8;
        let (new_idx, new_rng) = rand_wheel_index(idx, rng);
        state.aux = new_idx as u32 | (new_rng << 8);
    }

    let color = color_wheel((state.aux & 0xFF) as u8);
    pixels.copy_within(0..len - 1, 1);
    pixels[0] = P::from_rgb8(color);
    state.counter = state.counter.wrapping_add(1);
}

/// Running with a new random 24-bit color every 4 pixels.
pub fn running_random2<P: Pixel>(pixels: &mut [P], state: &mut EffectState, _params: &Params) {
    let len = pixels.len();

    if state.counter % 4 == 0 {
        state.aux = next_rand(state.aux);
    }

    let color = smart_leds_trait::RGB8 {
        r: (state.aux >> 16) as u8,
        g: (state.aux >> 8) as u8,
        b: state.aux as u8,
    };
    pixels.copy_within(0..len - 1, 1);
    pixels[0] = P::from_rgb8(color);
    state.counter = state.counter.wrapping_add(1);
}
