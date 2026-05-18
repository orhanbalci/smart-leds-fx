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

pub struct Segment {
    /// First pixel index on the strip (inclusive).
    pub start: usize,
    /// Last pixel index on the strip (inclusive).
    pub stop: usize,
    pub effect: Effect,
    pub config: EffectConfig,
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
            state: EffectState::default(),
            last_update: 0,
        }
    }
}
