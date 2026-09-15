//! What an effect reads on each step.

use color8::{ColorBlend, CrgbPalette16, color_from_palette16};
use smart_leds_trait::RGB8;

use crate::pixel::Pixel;
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
    /// With a [`palette`](Self::palette), the palette replaces the primary.
    pub colors: [RGB8; 3],
    /// A palette for the primary color. Effects that draw the primary color
    /// follow it along the strip, and effects that cycle the hue wheel cycle
    /// through it instead. `None` draws `colors[0]` and the hue wheel.
    pub palette: Option<CrgbPalette16>,
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
            palette: None,
            speed: 200,
            intensity: Self::DEFAULT_INTENSITY,
            size: 0,
            reverse: false,
        }
    }

    /// With `palette` in place of the primary color.
    pub const fn palette(mut self, palette: CrgbPalette16) -> Self {
        self.palette = Some(palette);
        self
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
        let mut params = Self::new(config.colors)
            .speed(config.speed)
            .size(options.size)
            .reverse(options.reverse);
        params.palette = config.palette;
        params
    }

    /// The primary color at `position` along the palette, blending from its
    /// last entry back into its first: `colors[0]` without a palette.
    pub fn primary(&self, position: u8) -> RGB8 {
        match &self.palette {
            Some(palette) => sample(palette, position, ColorBlend::LinearBlend),
            None => self.colors[0],
        }
    }

    /// The primary color of pixel `index` on a strip of `len` pixels: the
    /// palette spread once from the first pixel to the last, or `colors[0]`
    /// without a palette.
    pub fn primary_at(&self, index: usize, len: usize) -> RGB8 {
        match &self.palette {
            Some(palette) => {
                let position = if len > 1 {
                    (index.min(len - 1) * 255 / (len - 1)) as u8
                } else {
                    0
                };
                sample(palette, position, ColorBlend::LinearBlendNoWrap)
            }
            None => self.colors[0],
        }
    }

    /// The color at `position` around the hue wheel, or along the palette
    /// when there is one.
    pub fn wheel(&self, position: u8) -> RGB8 {
        match &self.palette {
            Some(palette) => sample(palette, position, ColorBlend::LinearBlend),
            None => crate::utils::color_wheel(position),
        }
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
        Self::from_segment(&config, &SegmentOptions::default())
    }
}

impl From<&EffectConfig> for Params {
    fn from(config: &EffectConfig) -> Self {
        Self::from(*config)
    }
}

fn sample(palette: &CrgbPalette16, position: u8, blend: ColorBlend) -> RGB8 {
    color_from_palette16(palette, position, 255, blend).to_rgb8()
}
