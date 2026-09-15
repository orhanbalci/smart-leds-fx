use crate::params::Params;
use crate::pixel::Pixel;
use crate::segment::EffectState;
use crate::utils::color_wheel;

/// The hue wheel spread along the strip, rotating. Intensity sets how much of
/// the wheel the strip shows: all of it at default intensity, twice around at
/// full.
pub fn rainbow_cycle<P: Pixel>(pixels: &mut [P], state: &mut EffectState, params: &Params) {
    let len = pixels.len() as u32;
    let span = 2 * u32::from(params.intensity);
    for (i, pixel) in pixels.iter_mut().enumerate() {
        let hue = state.counter.wrapping_add(i as u32 * span / len) as u8;
        *pixel = P::from_rgb8(color_wheel(hue));
    }
    state.counter = state.counter.wrapping_add(1);
}
