use smart_leds_trait::RGB8;

use crate::effects;
use crate::segment::{EffectConfig, EffectState};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Effect {
    Static,
    Blink,
    Breath,
    RainbowCycle,
}

impl Effect {
    pub fn render(self, pixels: &mut [RGB8], state: &mut EffectState, config: &EffectConfig) {
        match self {
            Effect::Static => effects::color::static_color(pixels, state, config),
            Effect::Blink => effects::color::blink(pixels, state, config),
            Effect::Breath => effects::color::breath(pixels, state, config),
            Effect::RainbowCycle => effects::rainbow::rainbow_cycle(pixels, state, config),
        }
    }
}
