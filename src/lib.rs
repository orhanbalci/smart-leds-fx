#![no_std]

pub mod effect;
pub mod effects;
pub mod segment;
pub mod utils;

use heapless::Vec;
use smart_leds_trait::RGB8;

pub use effect::Effect;
pub use segment::{EffectConfig, Segment, SegmentOptions};

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
    running: bool,
    triggered: bool,
}

impl<const N: usize> Ws2812Fx<N> {
    /// Creates an instance covering the whole strip with `Static` at full brightness.
    /// Call [`start`](Self::start) before the main loop to begin animation.
    pub fn new(brightness: u8) -> Self {
        let mut fx = Self {
            pixels: [RGB8 { r: 0, g: 0, b: 0 }; N],
            segments: Vec::new(),
            brightness,
            running: false,
            triggered: false,
        };
        let _ = fx.segments.push(Segment::new(0, N - 1, Effect::Static));
        fx
    }
}

impl<const N: usize> Ws2812Fx<N> {
    /// Start the animation engine.
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the animation engine and blank the strip.
    pub fn stop(&mut self) {
        self.running = false;
        self.strip_off();
    }

    /// Pause without clearing the display.
    pub fn pause(&mut self) {
        self.running = false;
    }

    /// Resume after [`pause`](Self::pause).
    pub fn resume(&mut self) {
        self.running = true;
    }

    /// Force one render frame on the next [`service`](Self::service) call,
    /// regardless of timing.
    pub fn trigger(&mut self) {
        self.triggered = true;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn is_triggered(&self) -> bool {
        self.triggered
    }

    /// Set all pixels to black. Does not stop the engine.
    pub fn strip_off(&mut self) {
        for p in self.pixels.iter_mut() {
            *p = RGB8 { r: 0, g: 0, b: 0 };
        }
    }

    /// Drive the effect engine. Pass the current time in milliseconds.
    ///
    /// Returns `true` when at least one segment was re-rendered and the strip
    /// should be written to hardware via [`iter`](Self::iter).
    pub fn service(&mut self, now_ms: u64) -> bool {
        if !self.running && !self.triggered {
            return false;
        }
        let force = self.triggered;
        self.triggered = false;

        let mut updated = false;
        for i in 0..self.segments.len() {
            let elapsed = now_ms.wrapping_sub(self.segments[i].last_update);
            if force || elapsed >= self.segments[i].config.speed as u64 {
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
}

impl<const N: usize> Ws2812Fx<N> {
    pub fn set_brightness(&mut self, brightness: u8) {
        self.brightness = brightness;
    }

    pub fn get_brightness(&self) -> u8 {
        self.brightness
    }

    pub fn increase_brightness(&mut self, amount: u8) {
        self.brightness = self.brightness.saturating_add(amount);
    }

    pub fn decrease_brightness(&mut self, amount: u8) {
        self.brightness = self.brightness.saturating_sub(amount);
    }

    /// Sum of all RGB channel values after brightness scaling — useful for
    /// estimating power draw before writing to hardware.
    pub fn intensity_sum(&self) -> u32 {
        self.iter()
            .map(|c| c.r as u32 + c.g as u32 + c.b as u32)
            .sum()
    }
}

impl<const N: usize> Ws2812Fx<N> {
    /// Add a new segment to the pool. Returns `Err` if the pool (max 10) is full.
    pub fn add_segment(&mut self, segment: Segment) -> Result<(), Segment> {
        self.segments.push(segment).map_err(|s| s)
    }

    /// Configure a segment by index. If `idx` equals the current segment count
    /// and the pool is not full, a new segment is appended.
    /// Returns `false` if `idx` is out of range or the pool is full.
    pub fn set_segment(&mut self, idx: usize, start: usize, stop: usize, effect: Effect) -> bool {
        if let Some(seg) = self.segments.get_mut(idx) {
            seg.start = start;
            seg.stop = stop;
            seg.effect = effect;
            seg.state = Default::default();
            seg.last_update = 0;
            true
        } else if idx == self.segments.len() {
            self.segments
                .push(Segment::new(start, stop, effect))
                .is_ok()
        } else {
            false
        }
    }

    /// Reset all segments to a single full-strip `Static` segment.
    pub fn reset_segments(&mut self) {
        self.segments.clear();
        let _ = self.segments.push(Segment::new(0, N - 1, Effect::Static));
    }

    /// Reset the animation state of one segment (keeps its effect and colors).
    pub fn reset_segment(&mut self, idx: usize) {
        if let Some(seg) = self.segments.get_mut(idx) {
            seg.state = Default::default();
            seg.last_update = 0;
        }
    }

    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }

    pub fn get_segment(&self, idx: usize) -> Option<&Segment> {
        self.segments.get(idx)
    }

    pub fn get_segment_mut(&mut self, idx: usize) -> Option<&mut Segment> {
        self.segments.get_mut(idx)
    }
}

impl<const N: usize> Ws2812Fx<N> {
    /// Set the effect for a segment, resetting its animation state.
    pub fn set_effect(&mut self, idx: usize, effect: Effect) {
        if let Some(seg) = self.segments.get_mut(idx) {
            seg.effect = effect;
            seg.state = Default::default();
        }
    }

    pub fn get_effect(&self, idx: usize) -> Option<Effect> {
        self.segments.get(idx).map(|s| s.effect)
    }

    pub fn set_speed(&mut self, idx: usize, speed: u16) {
        if let Some(seg) = self.segments.get_mut(idx) {
            seg.config.speed = speed;
        }
    }

    pub fn get_speed(&self, idx: usize) -> Option<u16> {
        self.segments.get(idx).map(|s| s.config.speed)
    }

    pub fn increase_speed(&mut self, idx: usize, amount: u16) {
        if let Some(seg) = self.segments.get_mut(idx) {
            seg.config.speed = seg.config.speed.saturating_sub(amount);
        }
    }

    pub fn decrease_speed(&mut self, idx: usize, amount: u16) {
        if let Some(seg) = self.segments.get_mut(idx) {
            seg.config.speed = seg.config.speed.saturating_add(amount);
        }
    }

    /// Set the primary color (`colors[0]`) of a segment.
    pub fn set_color(&mut self, idx: usize, color: RGB8) {
        if let Some(seg) = self.segments.get_mut(idx) {
            seg.config.colors[0] = color;
        }
    }

    pub fn get_color(&self, idx: usize) -> Option<RGB8> {
        self.segments.get(idx).map(|s| s.config.colors[0])
    }

    /// Set all three colors of a segment at once.
    pub fn set_colors(&mut self, idx: usize, colors: [RGB8; 3]) {
        if let Some(seg) = self.segments.get_mut(idx) {
            seg.config.colors = colors;
        }
    }

    pub fn get_colors(&self, idx: usize) -> Option<[RGB8; 3]> {
        self.segments.get(idx).map(|s| s.config.colors)
    }

    pub fn set_options(&mut self, idx: usize, options: SegmentOptions) {
        if let Some(seg) = self.segments.get_mut(idx) {
            seg.options = options;
        }
    }

    pub fn get_options(&self, idx: usize) -> Option<SegmentOptions> {
        self.segments.get(idx).map(|s| s.options)
    }
}

impl<const N: usize> Ws2812Fx<N> {
    /// Seed the PRNG state of every segment. Useful for reproducible patterns.
    pub fn set_random_seed(&mut self, seed: u32) {
        for seg in self.segments.iter_mut() {
            seg.state.aux = seed;
        }
    }

    /// Total number of built-in effects.
    pub fn mode_count() -> usize {
        Effect::count()
    }

    /// Name string for any effect.
    pub fn mode_name(effect: Effect) -> &'static str {
        effect.name()
    }
}
