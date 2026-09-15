//! What an effect reads on each step.

use smart_leds_trait::RGB8;

use crate::segment::{EffectConfig, SegmentOptions};

/// Everything an effect reads on each step: a superset of [`EffectConfig`]
/// and [`SegmentOptions`].
///
/// Build one with [`Params::new`] and the builder methods, or convert from an
/// [`EffectConfig`]. New fields may be added in minor releases, each with a
/// default that keeps effects looking the way they did before it existed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Params {
    /// Up to three colors the effect can use (primary, secondary, background).
    pub colors: [RGB8; 3],
    /// Milliseconds between steps. Most effects ignore it — the caller decides
    /// when to step — but a few shape their pattern with it.
    pub speed: u16,
    /// Effect-specific strength, `0`–`255`: trail length, fade rate, flicker
    /// depth, sparkle count or rainbow span, depending on the effect.
    /// [`Params::DEFAULT_INTENSITY`] draws each effect's classic look.
    pub intensity: u8,
    /// Pixel grouping: each drawn pixel covers `2^size` LEDs (`0` = 1 LED,
    /// `3` = 8 LEDs). Values above 3 are treated as 3.
    pub size: u8,
    /// Draw from the far end of the strip.
    pub reverse: bool,
}

impl Params {
    /// The intensity at which every effect draws its classic look.
    pub const DEFAULT_INTENSITY: u8 = 128;

    /// Default parameters with the given colors: 200 ms steps, default
    /// intensity, no grouping, forward.
    pub const fn new(colors: [RGB8; 3]) -> Self {
        Self {
            colors,
            speed: 200,
            intensity: Self::DEFAULT_INTENSITY,
            size: 0,
            reverse: false,
        }
    }

    /// With `speed` milliseconds between steps.
    pub const fn speed(mut self, speed: u16) -> Self {
        self.speed = speed;
        self
    }

    /// With `intensity`.
    pub const fn intensity(mut self, intensity: u8) -> Self {
        self.intensity = intensity;
        self
    }

    /// With pixel grouping `size`.
    pub const fn size(mut self, size: u8) -> Self {
        self.size = size;
        self
    }

    /// Drawn in reverse.
    pub const fn reverse(mut self, reverse: bool) -> Self {
        self.reverse = reverse;
        self
    }

    /// `config` with `options` applied.
    pub const fn from_segment(config: &EffectConfig, options: &SegmentOptions) -> Self {
        Self::new(config.colors)
            .speed(config.speed)
            .size(options.size)
            .reverse(options.reverse)
    }

    /// LEDs covered by each drawn pixel.
    pub(crate) const fn group_len(&self) -> usize {
        1 << if self.size > 3 { 3 } else { self.size }
    }
}

impl Default for Params {
    fn default() -> Self {
        Self::from(EffectConfig::default())
    }
}

impl From<EffectConfig> for Params {
    fn from(config: EffectConfig) -> Self {
        Self::new(config.colors).speed(config.speed)
    }
}

impl From<&EffectConfig> for Params {
    fn from(config: &EffectConfig) -> Self {
        Self::from(*config)
    }
}
