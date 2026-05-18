use smart_leds_trait::RGB8;

use crate::effects;
use crate::segment::{EffectConfig, EffectState};
use crate::utils::{color_wheel, next_rand, BLACK, BLUE, GREEN, ORANGE, PURPLE, RED, WHITE};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Effect {
    // --- color ---
    Static,
    Blink,
    BlinkRainbow,
    Strobe,          // fast blink (same visual, intended for short speed)
    StrobeRainbow,
    Breath,
    Rainbow,
    Fade,
    HyperSparkle,
    MultiStrobe,

    // --- rainbow ---
    RainbowCycle,

    // --- wipe / scan ---
    ColorWipe,
    ColorWipeInv,
    ColorWipeRandom,
    ColorSweepRandom,
    Scan,
    DualScan,

    // --- chase ---
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

    // --- running ---
    RunningColor,
    RunningRedBlue,
    MerryChristmas,
    Halloween,
    RunningRandom,
    RunningRandom2,
    RunningLights,

    // --- dynamic ---
    RandomColor,
    SingleDynamic,
    MultiDynamic,
    BlockDissolve,

    // --- twinkle ---
    Twinkle,
    TwinkleRandom,
    TwinkleFade,
    TwinkleFadeRandom,
    Sparkle,
    FlashSparkle,

    // --- scanner ---
    LarsonScanner,
    Comet,
    DualLarson,
    RainbowLarson,
    MultiComet,

    // --- fire ---
    Fireworks,
    FireworksRandom,
    FireFlicker,
    FireFlickerSoft,
    FireFlickerIntense,

    // --- complex ---
    TwinkleFox,
    Rain,
    Icu,
    FillerUp,
    TriFade,
    Heartbeat,
    RainbowFireworks,
}

impl Effect {
    pub fn render(self, pixels: &mut [RGB8], state: &mut EffectState, config: &EffectConfig) {
        use effects::*;

        match self {
            // ── color ───────────────────────────────────────────────────────
            Effect::Static          => color::static_color(pixels, state, config),
            Effect::Blink           => color::blink(pixels, state, config),
            Effect::BlinkRainbow    => color::blink_rainbow(pixels, state, config),
            Effect::Strobe          => color::blink(pixels, state, config),
            Effect::StrobeRainbow   => color::blink_rainbow(pixels, state, config),
            Effect::Breath          => color::breath(pixels, state, config),
            Effect::Rainbow         => color::rainbow(pixels, state, config),
            Effect::Fade            => color::fade(pixels, state, config),
            Effect::HyperSparkle    => color::hyper_sparkle(pixels, state, config),
            Effect::MultiStrobe     => color::multi_strobe(pixels, state, config),

            // ── rainbow ─────────────────────────────────────────────────────
            Effect::RainbowCycle    => rainbow::rainbow_cycle(pixels, state, config),

            // ── wipe / scan ─────────────────────────────────────────────────
            Effect::ColorWipe       => chase::color_wipe(pixels, state, config),
            Effect::ColorWipeInv    => {
                let mut cfg = *config;
                cfg.colors.swap(0, 1);
                chase::color_wipe(pixels, state, &cfg);
            }
            Effect::ColorWipeRandom  => chase::color_wipe_random(pixels, state, config),
            Effect::ColorSweepRandom => chase::color_sweep_random(pixels, state, config),
            Effect::Scan             => chase::scan(pixels, state, config),
            Effect::DualScan         => chase::dual_scan(pixels, state, config),

            // ── chase ───────────────────────────────────────────────────────
            Effect::TricolorChase        => chase::tricolor_chase(pixels, state, config),
            Effect::CircusCombustus      => {
                let mut cfg = *config;
                cfg.colors = [RED, WHITE, BLACK];
                chase::tricolor_chase(pixels, state, &cfg);
            }
            Effect::TheaterChase         => {
                let mut cfg = *config;
                cfg.colors[2] = config.colors[1];
                chase::tricolor_chase(pixels, state, &cfg);
            }
            Effect::TheaterChaseRainbow  => chase::theater_chase_rainbow(pixels, state, config),
            Effect::BicolorChase         => chase::chase(pixels, state, config),
            Effect::ChaseColor           => {
                let mut cfg = *config;
                cfg.colors[1] = WHITE;
                cfg.colors[2] = WHITE;
                chase::chase(pixels, state, &cfg);
            }
            Effect::ChaseBlackout        => {
                let mut cfg = *config;
                cfg.colors[1] = BLACK;
                cfg.colors[2] = BLACK;
                chase::chase(pixels, state, &cfg);
            }
            Effect::ChaseWhite           => {
                let mut cfg = *config;
                cfg.colors[0] = WHITE;
                cfg.colors[1] = config.colors[0];
                cfg.colors[2] = config.colors[0];
                chase::chase(pixels, state, &cfg);
            }
            Effect::ChaseRandom          => chase::chase_random(pixels, state, config),
            Effect::ChaseRainbowWhite    => chase::chase_rainbow_white(pixels, state, config),
            Effect::ChaseRainbow         => chase::chase_rainbow(pixels, state, config),
            Effect::ChaseBlackoutRainbow => chase::chase_blackout_rainbow(pixels, state, config),
            Effect::ChaseFlash           => {
                let mut cfg = *config;
                cfg.colors[1] = WHITE;
                chase::chase_flash(pixels, state, &cfg);
            }
            Effect::ChaseFlashRandom     => chase::chase_flash_random(pixels, state, config),

            // ── running ─────────────────────────────────────────────────────
            Effect::RunningColor    => chase::running(pixels, state, config),
            Effect::RunningRedBlue  => {
                let mut cfg = *config;
                cfg.colors = [RED, BLUE, BLACK];
                chase::running(pixels, state, &cfg);
            }
            Effect::MerryChristmas  => {
                let mut cfg = *config;
                cfg.colors = [RED, GREEN, BLACK];
                chase::running(pixels, state, &cfg);
            }
            Effect::Halloween       => {
                let mut cfg = *config;
                cfg.colors = [PURPLE, ORANGE, BLACK];
                chase::running(pixels, state, &cfg);
            }
            Effect::RunningRandom   => chase::running_random(pixels, state, config),
            Effect::RunningRandom2  => chase::running_random2(pixels, state, config),
            Effect::RunningLights   => dynamic::running_lights(pixels, state, config),

            // ── dynamic ─────────────────────────────────────────────────────
            Effect::RandomColor     => dynamic::random_color(pixels, state, config),
            Effect::SingleDynamic   => dynamic::single_dynamic(pixels, state, config),
            Effect::MultiDynamic    => dynamic::multi_dynamic(pixels, state, config),
            Effect::BlockDissolve   => dynamic::block_dissolve(pixels, state, config),

            // ── twinkle ─────────────────────────────────────────────────────
            Effect::Twinkle          => twinkle::twinkle(pixels, state, config),
            Effect::TwinkleRandom    => twinkle::twinkle_random(pixels, state, config),
            Effect::TwinkleFade      => twinkle::twinkle_fade(pixels, state, config),
            Effect::TwinkleFadeRandom => twinkle::twinkle_fade_random(pixels, state, config),
            Effect::Sparkle          => twinkle::sparkle(pixels, state, config),
            Effect::FlashSparkle     => twinkle::flash_sparkle(pixels, state, config),

            // ── scanner ─────────────────────────────────────────────────────
            Effect::LarsonScanner   => scanner::larson_scanner(pixels, state, config),
            Effect::Comet           => scanner::comet(pixels, state, config),
            Effect::DualLarson      => scanner::dual_larson(pixels, state, config),
            Effect::RainbowLarson   => scanner::rainbow_larson(pixels, state, config),
            Effect::MultiComet      => scanner::multi_comet(pixels, state, config),

            // ── fire ────────────────────────────────────────────────────────
            Effect::Fireworks        => fire::fireworks(pixels, state, config),
            Effect::FireworksRandom  => {
                let mut cfg = *config;
                let rng = next_rand(state.aux);
                state.aux = rng;
                cfg.colors[0] = color_wheel(rng as u8);
                fire::fireworks(pixels, state, &cfg);
            }
            Effect::FireFlicker        => fire::fire_flicker(pixels, state, config),
            Effect::FireFlickerSoft    => fire::fire_flicker_soft(pixels, state, config),
            Effect::FireFlickerIntense => fire::fire_flicker_intense(pixels, state, config),

            // ── complex ─────────────────────────────────────────────────────
            Effect::TwinkleFox      => complex::twinkle_fox(pixels, state, config),
            Effect::Rain            => complex::rain(pixels, state, config),
            Effect::Icu             => complex::icu(pixels, state, config),
            Effect::FillerUp        => complex::filler_up(pixels, state, config),
            Effect::TriFade         => complex::trifade(pixels, state, config),
            Effect::Heartbeat       => complex::heartbeat(pixels, state, config),
            Effect::RainbowFireworks => complex::rainbow_fireworks(pixels, state, config),
        }
    }
}
