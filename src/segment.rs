use smart_leds_trait::RGB8;

use crate::effect::Effect;

#[derive(Clone, Copy, Default)]
pub struct EffectState {
    pub counter: u32,
    pub aux: u32,
}

#[derive(Clone, Copy)]
pub struct EffectConfig {
    /// Up to three colors the effect can use (primary, secondary, background).
    pub colors: [RGB8; 3],
    /// Milliseconds between effect steps.
    pub speed: u16,
}

impl Default for EffectConfig {
    fn default() -> Self {
        Self {
            colors: [
                RGB8 { r: 255, g: 0, b: 0 },
                RGB8 { r: 0, g: 0, b: 0 },
                RGB8 { r: 0, g: 0, b: 0 },
            ],
            speed: 200,
        }
    }
}

/// Per-segment rendering options.
#[derive(Clone, Copy, Default)]
pub struct SegmentOptions {
    /// Reverse the direction of directional effects.
    pub reverse: bool,
    /// Dot/block size for effects that support it (0 = 1 px, 1 = 2 px, 2 = 4 px, 3 = 8 px).
    pub size: u8,
}

pub struct Segment {
    /// First pixel index on the strip (inclusive).
    pub start: usize,
    /// Last pixel index on the strip (inclusive).
    pub stop: usize,
    pub effect: Effect,
    pub config: EffectConfig,
    pub options: SegmentOptions,
    pub state: EffectState,
    pub last_update: u64,
}

impl Segment {
    pub fn new(start: usize, stop: usize, effect: Effect) -> Self {
        Self {
            start,
            stop,
            effect,
            config: EffectConfig::default(),
            options: SegmentOptions::default(),
            state: EffectState::default(),
            last_update: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.stop.saturating_sub(self.start) + 1
    }

    pub fn speed(mut self, ms: u16) -> Self {
        self.config.speed = ms;
        self
    }

    pub fn color(mut self, color: RGB8) -> Self {
        self.config.colors[0] = color;
        self
    }

    pub fn colors(mut self, colors: [RGB8; 3]) -> Self {
        self.config.colors = colors;
        self
    }

    pub fn options(mut self, options: SegmentOptions) -> Self {
        self.options = options;
        self
    }
}
