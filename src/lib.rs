#![no_std]

pub mod effect;
pub mod effects;
pub mod segment;

use heapless::Vec;
use smart_leds_trait::RGB8;

pub use effect::Effect;
pub use segment::{EffectConfig, Segment};

const MAX_SEGMENTS: usize = 10;

/// Hardware-agnostic WS2812FX effect engine.
///
/// `N` is the number of LEDs on the strip. Call [`service`](Self::service) from
/// your main loop with the current timestamp, then feed [`iter`](Self::iter) to
/// your [`SmartLedsWrite`](smart_leds_trait::SmartLedsWrite) driver when it
/// returns `true`.
pub struct Ws2812Fx<const N: usize> {
    pixels: [RGB8; N],
    segments: Vec<Segment, MAX_SEGMENTS>,
    brightness: u8,
}

impl<const N: usize> Ws2812Fx<N> {
    /// Creates an instance with a single segment covering the whole strip,
    /// using the `Static` effect at full brightness.
    pub fn new(brightness: u8) -> Self {
        let mut fx = Self {
            pixels: [RGB8 { r: 0, g: 0, b: 0 }; N],
            segments: Vec::new(),
            brightness,
        };
        let _ = fx.segments.push(Segment::new(0, N - 1, Effect::Static));
        fx
    }

    /// Drive the effect engine. Pass the current time in milliseconds.
    ///
    /// Returns `true` when at least one segment was re-rendered and the strip
    /// should be written to hardware via [`iter`](Self::iter).
    pub fn service(&mut self, now_ms: u64) -> bool {
        let mut updated = false;
        for i in 0..self.segments.len() {
            let elapsed = now_ms.wrapping_sub(self.segments[i].last_update);
            if elapsed >= self.segments[i].config.speed as u64 {
                // Copy out the immutable parts so we can mutably borrow pixels.
                let start = self.segments[i].start;
                let stop = self.segments[i].stop;
                let effect = self.segments[i].effect;
                let config = self.segments[i].config;
                let mut state = self.segments[i].state;

                let end = (stop + 1).min(N);
                effect.render(&mut self.pixels[start..end], &mut state, &config);

                self.segments[i].state = state;
                self.segments[i].last_update = now_ms;
                updated = true;
            }
        }
        updated
    }

    /// Returns an iterator of brightness-scaled pixels ready for a
    /// `SmartLedsWrite` driver.
    pub fn iter(&self) -> impl Iterator<Item = RGB8> + '_ {
        let brightness = self.brightness;
        self.pixels.iter().map(move |c| RGB8 {
            r: (c.r as u16 * brightness as u16 / 255) as u8,
            g: (c.g as u16 * brightness as u16 / 255) as u8,
            b: (c.b as u16 * brightness as u16 / 255) as u8,
        })
    }

    pub fn set_brightness(&mut self, brightness: u8) {
        self.brightness = brightness;
    }

    /// Set the effect for a segment, resetting its animation state.
    pub fn set_effect(&mut self, segment_idx: usize, effect: Effect) {
        if let Some(seg) = self.segments.get_mut(segment_idx) {
            seg.effect = effect;
            seg.state = Default::default();
        }
    }

    /// Set the primary color for a segment.
    pub fn set_color(&mut self, segment_idx: usize, color: RGB8) {
        if let Some(seg) = self.segments.get_mut(segment_idx) {
            seg.config.colors[0] = color;
        }
    }

    /// Set the effect speed (ms between steps) for a segment.
    pub fn set_speed(&mut self, segment_idx: usize, speed: u16) {
        if let Some(seg) = self.segments.get_mut(segment_idx) {
            seg.config.speed = speed;
        }
    }

    /// Add a segment. Returns `Err(segment)` if the segment pool is full.
    pub fn add_segment(&mut self, segment: Segment) -> Result<(), Segment> {
        self.segments.push(segment).map_err(|s| s)
    }

    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }
}
