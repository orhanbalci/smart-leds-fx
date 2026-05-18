use smart_leds_trait::RGB8;

use crate::effects;
use crate::segment::{EffectConfig, EffectState};
use crate::utils::{BLACK, BLUE, GREEN, ORANGE, PURPLE, RED, WHITE, color_wheel, next_rand};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Effect {
    Static,
    Blink,
    BlinkRainbow,
    Strobe,
    StrobeRainbow,
    Breath,
    Rainbow,
    Fade,
    HyperSparkle,
    MultiStrobe,
    RainbowCycle,
    ColorWipe,
    ColorWipeInv,
    ColorWipeRandom,
    ColorSweepRandom,
    Scan,
    DualScan,
    TricolorChase,
    CircusCombustus,
    TheaterChase,
    TheaterChaseRainbow,
    BicolorChase,
    ChaseColor,
    ChaseBlackout,
    ChaseWhite,
    ChaseRandom,
    ChaseRainbowWhite,
    ChaseRainbow,
    ChaseBlackoutRainbow,
    ChaseFlash,
    ChaseFlashRandom,
    RunningColor,
    RunningRedBlue,
    MerryChristmas,
    Halloween,
    RunningRandom,
    RunningRandom2,
    RunningLights,
    RandomColor,
    SingleDynamic,
    MultiDynamic,
    BlockDissolve,
    Twinkle,
    TwinkleRandom,
    TwinkleFade,
    TwinkleFadeRandom,
    Sparkle,
    FlashSparkle,
    SparkleRandom,
    LarsonScanner,
    Comet,
    DualLarson,
    RainbowLarson,
    MultiComet,
    Fireworks,
    FireworksRandom,
    FireFlicker,
    FireFlickerSoft,
    FireFlickerIntense,
    TwinkleFox,
    Rain,
    Icu,
    FillerUp,
    TriFade,
    Heartbeat,
    RainbowFireworks,
}

impl Effect {
    /// Human-readable name for display or debugging.
    pub fn name(self) -> &'static str {
        match self {
            Effect::Static => "Static",
            Effect::Blink => "Blink",
            Effect::BlinkRainbow => "Blink Rainbow",
            Effect::Strobe => "Strobe",
            Effect::StrobeRainbow => "Strobe Rainbow",
            Effect::Breath => "Breath",
            Effect::Rainbow => "Rainbow",
            Effect::Fade => "Fade",
            Effect::HyperSparkle => "Hyper Sparkle",
            Effect::MultiStrobe => "Multi Strobe",
            Effect::RainbowCycle => "Rainbow Cycle",
            Effect::ColorWipe => "Color Wipe",
            Effect::ColorWipeInv => "Color Wipe Inv",
            Effect::ColorWipeRandom => "Color Wipe Random",
            Effect::ColorSweepRandom => "Color Sweep Random",
            Effect::Scan => "Scan",
            Effect::DualScan => "Dual Scan",
            Effect::TricolorChase => "Tricolor Chase",
            Effect::CircusCombustus => "Circus Combustus",
            Effect::TheaterChase => "Theater Chase",
            Effect::TheaterChaseRainbow => "Theater Chase Rainbow",
            Effect::BicolorChase => "Bicolor Chase",
            Effect::ChaseColor => "Chase Color",
            Effect::ChaseBlackout => "Chase Blackout",
            Effect::ChaseWhite => "Chase White",
            Effect::ChaseRandom => "Chase Random",
            Effect::ChaseRainbowWhite => "Chase Rainbow White",
            Effect::ChaseRainbow => "Chase Rainbow",
            Effect::ChaseBlackoutRainbow => "Chase Blackout Rainbow",
            Effect::ChaseFlash => "Chase Flash",
            Effect::ChaseFlashRandom => "Chase Flash Random",
            Effect::RunningColor => "Running Color",
            Effect::RunningRedBlue => "Running Red Blue",
            Effect::MerryChristmas => "Merry Christmas",
            Effect::Halloween => "Halloween",
            Effect::RunningRandom => "Running Random",
            Effect::RunningRandom2 => "Running Random 2",
            Effect::RunningLights => "Running Lights",
            Effect::RandomColor => "Random Color",
            Effect::SingleDynamic => "Single Dynamic",
            Effect::MultiDynamic => "Multi Dynamic",
            Effect::BlockDissolve => "Block Dissolve",
            Effect::Twinkle => "Twinkle",
            Effect::TwinkleRandom => "Twinkle Random",
            Effect::TwinkleFade => "Twinkle Fade",
            Effect::TwinkleFadeRandom => "Twinkle Fade Random",
            Effect::Sparkle => "Sparkle",
            Effect::FlashSparkle => "Flash Sparkle",
            Effect::SparkleRandom => "Sparkle Random",
            Effect::LarsonScanner => "Larson Scanner",
            Effect::Comet => "Comet",
            Effect::DualLarson => "Dual Larson",
            Effect::RainbowLarson => "Rainbow Larson",
            Effect::MultiComet => "Multi Comet",
            Effect::Fireworks => "Fireworks",
            Effect::FireworksRandom => "Fireworks Random",
            Effect::FireFlicker => "Fire Flicker",
            Effect::FireFlickerSoft => "Fire Flicker (Soft)",
            Effect::FireFlickerIntense => "Fire Flicker (Intense)",
            Effect::TwinkleFox => "TwinkleFOX",
            Effect::Rain => "Rain",
            Effect::Icu => "ICU",
            Effect::FillerUp => "Filler Up",
            Effect::TriFade => "Tri Fade",
            Effect::Heartbeat => "Heartbeat",
            Effect::RainbowFireworks => "Rainbow Fireworks",
        }
    }

    /// Total number of available effects.
    pub const fn count() -> usize {
        68
    }

    pub fn render(self, pixels: &mut [RGB8], state: &mut EffectState, config: &EffectConfig) {
        use effects::*;

        match self {
            Effect::Static => color::static_color(pixels, state, config),
            Effect::Blink => color::blink(pixels, state, config),
            Effect::BlinkRainbow => color::blink_rainbow(pixels, state, config),
            Effect::Strobe => color::blink(pixels, state, config),
            Effect::StrobeRainbow => color::blink_rainbow(pixels, state, config),
            Effect::Breath => color::breath(pixels, state, config),
            Effect::Rainbow => color::rainbow(pixels, state, config),
            Effect::Fade => color::fade(pixels, state, config),
            Effect::HyperSparkle => color::hyper_sparkle(pixels, state, config),
            Effect::MultiStrobe => color::multi_strobe(pixels, state, config),

            Effect::RainbowCycle => rainbow::rainbow_cycle(pixels, state, config),

            Effect::ColorWipe => chase::color_wipe(pixels, state, config),
            Effect::ColorWipeInv => {
                let mut cfg = *config;
                cfg.colors.swap(0, 1);
                chase::color_wipe(pixels, state, &cfg);
            }
            Effect::ColorWipeRandom => chase::color_wipe_random(pixels, state, config),
            Effect::ColorSweepRandom => chase::color_sweep_random(pixels, state, config),
            Effect::Scan => chase::scan(pixels, state, config),
            Effect::DualScan => chase::dual_scan(pixels, state, config),

            Effect::TricolorChase => chase::tricolor_chase(pixels, state, config),
            Effect::CircusCombustus => {
                let mut cfg = *config;
                cfg.colors = [RED, WHITE, BLACK];
                chase::tricolor_chase(pixels, state, &cfg);
            }
            Effect::TheaterChase => {
                let mut cfg = *config;
                cfg.colors[2] = config.colors[1];
                chase::tricolor_chase(pixels, state, &cfg);
            }
            Effect::TheaterChaseRainbow => chase::theater_chase_rainbow(pixels, state, config),
            Effect::BicolorChase => chase::chase(pixels, state, config),
            Effect::ChaseColor => {
                let mut cfg = *config;
                cfg.colors[1] = WHITE;
                cfg.colors[2] = WHITE;
                chase::chase(pixels, state, &cfg);
            }
            Effect::ChaseBlackout => {
                let mut cfg = *config;
                cfg.colors[1] = BLACK;
                cfg.colors[2] = BLACK;
                chase::chase(pixels, state, &cfg);
            }
            Effect::ChaseWhite => {
                let mut cfg = *config;
                cfg.colors[0] = WHITE;
                cfg.colors[1] = config.colors[0];
                cfg.colors[2] = config.colors[0];
                chase::chase(pixels, state, &cfg);
            }
            Effect::ChaseRandom => chase::chase_random(pixels, state, config),
            Effect::ChaseRainbowWhite => chase::chase_rainbow_white(pixels, state, config),
            Effect::ChaseRainbow => chase::chase_rainbow(pixels, state, config),
            Effect::ChaseBlackoutRainbow => chase::chase_blackout_rainbow(pixels, state, config),
            Effect::ChaseFlash => {
                let mut cfg = *config;
                cfg.colors[1] = WHITE;
                chase::chase_flash(pixels, state, &cfg);
            }
            Effect::ChaseFlashRandom => chase::chase_flash_random(pixels, state, config),

            Effect::RunningColor => chase::running(pixels, state, config),
            Effect::RunningRedBlue => {
                let mut cfg = *config;
                cfg.colors = [RED, BLUE, BLACK];
                chase::running(pixels, state, &cfg);
            }
            Effect::MerryChristmas => {
                let mut cfg = *config;
                cfg.colors = [RED, GREEN, BLACK];
                chase::running(pixels, state, &cfg);
            }
            Effect::Halloween => {
                let mut cfg = *config;
                cfg.colors = [PURPLE, ORANGE, BLACK];
                chase::running(pixels, state, &cfg);
            }
            Effect::RunningRandom => chase::running_random(pixels, state, config),
            Effect::RunningRandom2 => chase::running_random2(pixels, state, config),
            Effect::RunningLights => dynamic::running_lights(pixels, state, config),

            Effect::RandomColor => dynamic::random_color(pixels, state, config),
            Effect::SingleDynamic => dynamic::single_dynamic(pixels, state, config),
            Effect::MultiDynamic => dynamic::multi_dynamic(pixels, state, config),
            Effect::BlockDissolve => dynamic::block_dissolve(pixels, state, config),

            Effect::Twinkle => twinkle::twinkle(pixels, state, config),
            Effect::TwinkleRandom => twinkle::twinkle_random(pixels, state, config),
            Effect::TwinkleFade => twinkle::twinkle_fade(pixels, state, config),
            Effect::TwinkleFadeRandom => twinkle::twinkle_fade_random(pixels, state, config),
            Effect::Sparkle => twinkle::sparkle(pixels, state, config),
            Effect::FlashSparkle => twinkle::flash_sparkle(pixels, state, config),

            Effect::LarsonScanner => scanner::larson_scanner(pixels, state, config),
            Effect::Comet => scanner::comet(pixels, state, config),
            Effect::DualLarson => scanner::dual_larson(pixels, state, config),
            Effect::RainbowLarson => scanner::rainbow_larson(pixels, state, config),
            Effect::MultiComet => scanner::multi_comet(pixels, state, config),

            Effect::Fireworks => fire::fireworks(pixels, state, config),
            Effect::FireworksRandom => {
                let mut cfg = *config;
                let rng = next_rand(state.aux);
                state.aux = rng;
                cfg.colors[0] = color_wheel(rng as u8);
                fire::fireworks(pixels, state, &cfg);
            }
            Effect::FireFlicker => fire::fire_flicker(pixels, state, config),
            Effect::FireFlickerSoft => fire::fire_flicker_soft(pixels, state, config),
            Effect::FireFlickerIntense => fire::fire_flicker_intense(pixels, state, config),

            Effect::TwinkleFox => complex::twinkle_fox(pixels, state, config),
            Effect::Rain => complex::rain(pixels, state, config),
            Effect::Icu => complex::icu(pixels, state, config),
            Effect::FillerUp => complex::filler_up(pixels, state, config),
            Effect::TriFade => complex::trifade(pixels, state, config),
            Effect::Heartbeat => complex::heartbeat(pixels, state, config),
            Effect::RainbowFireworks => complex::rainbow_fireworks(pixels, state, config),
            Effect::SparkleRandom => twinkle::sparkle_random(pixels, state, config),
        }
    }
}
