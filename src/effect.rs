use smart_leds_trait::RGB8;

use crate::effects;
use crate::params::{Params, Setting};
use crate::pixel::Pixel;
use crate::segment::{EffectConfig, EffectState};
use crate::utils::{BLACK, BLUE, GREEN, ORANGE, PURPLE, RED, WHITE, next_rand};

/// A built-in effect.
///
/// New effects may be added in minor releases, so a `match` over effects needs
/// a wildcard arm; [`Effect::ALL`] always lists every one.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
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
    Sine,
    Bpm,
    Percent,
    Wavesins,
    SolidPattern,
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
            Effect::Sine => "Sine",
            Effect::Bpm => "Bpm",
            Effect::Percent => "Percent",
            Effect::Wavesins => "Wavesins",
            Effect::SolidPattern => "Solid Pattern",
        }
    }

    /// All available effects in definition order.
    pub const ALL: &'static [Effect] = &[
        Effect::Static,
        Effect::Blink,
        Effect::BlinkRainbow,
        Effect::Strobe,
        Effect::StrobeRainbow,
        Effect::Breath,
        Effect::Rainbow,
        Effect::Fade,
        Effect::HyperSparkle,
        Effect::MultiStrobe,
        Effect::RainbowCycle,
        Effect::ColorWipe,
        Effect::ColorWipeInv,
        Effect::ColorWipeRandom,
        Effect::ColorSweepRandom,
        Effect::Scan,
        Effect::DualScan,
        Effect::TricolorChase,
        Effect::CircusCombustus,
        Effect::TheaterChase,
        Effect::TheaterChaseRainbow,
        Effect::BicolorChase,
        Effect::ChaseColor,
        Effect::ChaseBlackout,
        Effect::ChaseWhite,
        Effect::ChaseRandom,
        Effect::ChaseRainbowWhite,
        Effect::ChaseRainbow,
        Effect::ChaseBlackoutRainbow,
        Effect::ChaseFlash,
        Effect::ChaseFlashRandom,
        Effect::RunningColor,
        Effect::RunningRedBlue,
        Effect::MerryChristmas,
        Effect::Halloween,
        Effect::RunningRandom,
        Effect::RunningRandom2,
        Effect::RunningLights,
        Effect::RandomColor,
        Effect::SingleDynamic,
        Effect::MultiDynamic,
        Effect::BlockDissolve,
        Effect::Twinkle,
        Effect::TwinkleRandom,
        Effect::TwinkleFade,
        Effect::TwinkleFadeRandom,
        Effect::Sparkle,
        Effect::FlashSparkle,
        Effect::SparkleRandom,
        Effect::LarsonScanner,
        Effect::Comet,
        Effect::DualLarson,
        Effect::RainbowLarson,
        Effect::MultiComet,
        Effect::Fireworks,
        Effect::FireworksRandom,
        Effect::FireFlicker,
        Effect::FireFlickerSoft,
        Effect::FireFlickerIntense,
        Effect::TwinkleFox,
        Effect::Rain,
        Effect::Icu,
        Effect::FillerUp,
        Effect::TriFade,
        Effect::Heartbeat,
        Effect::RainbowFireworks,
        Effect::Sine,
        Effect::Bpm,
        Effect::Percent,
        Effect::Wavesins,
        Effect::SolidPattern,
    ];

    /// Total number of available effects.
    pub const fn count() -> usize {
        Self::ALL.len()
    }

    /// Iterator over every effect variant.
    pub fn iter() -> impl Iterator<Item = Effect> {
        Self::ALL.iter().copied()
    }

    /// The settings this effect reads, beyond its colors, palette, grouping,
    /// direction and — for time-driven effects — the clock.
    ///
    /// For callers that map controls of their own onto effects: a control
    /// mapped to a setting not listed here changes nothing.
    pub const fn settings(self) -> &'static [Setting] {
        match self {
            Effect::Strobe
            | Effect::StrobeRainbow
            | Effect::HyperSparkle
            | Effect::RainbowCycle
            | Effect::LarsonScanner
            | Effect::DualLarson
            | Effect::RainbowLarson
            | Effect::Comet
            | Effect::MultiComet
            | Effect::TwinkleFade
            | Effect::TwinkleFadeRandom
            | Effect::Fireworks
            | Effect::FireworksRandom
            | Effect::FireFlicker
            | Effect::FireFlickerSoft
            | Effect::FireFlickerIntense
            | Effect::Rain
            | Effect::Heartbeat => &[Setting::Intensity],
            Effect::Sine => &[Setting::Rate, Setting::Scale],
            Effect::Bpm => &[Setting::Rate],
            Effect::Percent => &[Setting::Fill, Setting::OneColor],
            Effect::Wavesins => &[
                Setting::Rate,
                Setting::Variation,
                Setting::PaletteStart,
                Setting::PaletteSpan,
                Setting::PaletteStep,
            ],
            Effect::SolidPattern => &[Setting::Width, Setting::Gap],
            _ => &[],
        }
    }

    /// Renders one step of this effect into `pixels`.
    ///
    /// The same as [`step`](Self::step) with parameters converted from
    /// `config`: default intensity, no grouping, forward.
    pub fn render(self, pixels: &mut [RGB8], state: &mut EffectState, config: &EffectConfig) {
        self.step(pixels, state, &Params::from(config));
    }

    /// Renders one step of this effect into a buffer of any [`Pixel`] type.
    ///
    /// Each call advances the animation by one step, drawing over what the
    /// buffer held after the previous step — many effects fade, shift or
    /// restore those pixels — so pass the same buffer and `state` every time.
    /// The caller decides when to step; [`StripFx`](crate::StripFx) steps a
    /// segment every [`Params::speed`] milliseconds. Time-driven effects (Sine,
    /// Bpm, Wavesins) draw from [`Params::now_ms`] instead of a step count, so
    /// stepping them every frame makes them move smoothly.
    ///
    /// Grouping ([`Params::size`]) and direction ([`Params::reverse`]) apply
    /// to every effect: it draws on the grouped, possibly reversed strip and
    /// the result is spread back over `pixels`.
    ///
    /// A [`Params::palette`] replaces the primary color: effects draw it along
    /// the strip ([`Params::primary_at`]), and effects that cycle the hue
    /// wheel cycle through it ([`Params::wheel`]). Effects with fixed colors
    /// of their own — Circus Combustus, Chase White, Running Red Blue, Merry
    /// Christmas, Halloween and Rainbow Fireworks — ignore it, as does Running
    /// Random 2, whose colors are random RGB.
    pub fn step<P: Pixel>(self, pixels: &mut [P], state: &mut EffectState, params: &Params) {
        // Effects index and divide by the length; an empty strip has nothing to draw.
        if pixels.is_empty() {
            return;
        }
        with_layout(pixels, params, |view| self.draw(view, state, params));
    }

    /// Draws one step on the strip as the effect sees it.
    fn draw<P: Pixel>(self, pixels: &mut [P], state: &mut EffectState, params: &Params) {
        use effects::*;

        match self {
            Effect::Static => color::static_color(pixels, state, params),
            Effect::Blink => color::blink(pixels, state, params),
            Effect::BlinkRainbow => color::blink_rainbow(pixels, state, params),
            Effect::Strobe => color::strobe(pixels, state, params),
            Effect::StrobeRainbow => color::strobe_rainbow(pixels, state, params),
            Effect::Breath => color::breath(pixels, state, params),
            Effect::Rainbow => color::rainbow(pixels, state, params),
            Effect::Fade => color::fade(pixels, state, params),
            Effect::HyperSparkle => color::hyper_sparkle(pixels, state, params),
            Effect::MultiStrobe => color::multi_strobe(pixels, state, params),

            Effect::RainbowCycle => rainbow::rainbow_cycle(pixels, state, params),

            Effect::ColorWipe => chase::color_wipe(pixels, state, params),
            Effect::ColorWipeInv => chase::color_wipe_inv(pixels, state, params),
            Effect::ColorWipeRandom => chase::color_wipe_random(pixels, state, params),
            Effect::ColorSweepRandom => chase::color_sweep_random(pixels, state, params),
            Effect::Scan => chase::scan(pixels, state, params),
            Effect::DualScan => chase::dual_scan(pixels, state, params),

            Effect::TricolorChase => chase::tricolor_chase(pixels, state, params),
            Effect::CircusCombustus => {
                let mut p = *params;
                p.colors = [RED, WHITE, BLACK];
                p.palette = None;
                chase::tricolor_chase(pixels, state, &p);
            }
            Effect::TheaterChase => {
                let mut p = *params;
                p.colors[2] = params.colors[1];
                chase::tricolor_chase(pixels, state, &p);
            }
            Effect::TheaterChaseRainbow => chase::theater_chase_rainbow(pixels, state, params),
            Effect::BicolorChase => chase::chase(pixels, state, params),
            Effect::ChaseColor => {
                let mut p = *params;
                p.colors[1] = WHITE;
                p.colors[2] = WHITE;
                chase::chase(pixels, state, &p);
            }
            Effect::ChaseBlackout => {
                let mut p = *params;
                p.colors[1] = BLACK;
                p.colors[2] = BLACK;
                chase::chase(pixels, state, &p);
            }
            Effect::ChaseWhite => {
                let mut p = *params;
                p.colors[0] = WHITE;
                p.palette = None;
                p.colors[1] = params.colors[0];
                p.colors[2] = params.colors[0];
                chase::chase(pixels, state, &p);
            }
            Effect::ChaseRandom => chase::chase_random(pixels, state, params),
            Effect::ChaseRainbowWhite => chase::chase_rainbow_white(pixels, state, params),
            Effect::ChaseRainbow => chase::chase_rainbow(pixels, state, params),
            Effect::ChaseBlackoutRainbow => chase::chase_blackout_rainbow(pixels, state, params),
            Effect::ChaseFlash => {
                let mut p = *params;
                p.colors[1] = WHITE;
                chase::chase_flash(pixels, state, &p);
            }
            Effect::ChaseFlashRandom => chase::chase_flash_random(pixels, state, params),

            Effect::RunningColor => chase::running(pixels, state, params),
            Effect::RunningRedBlue => {
                let mut p = *params;
                p.colors = [RED, BLUE, BLACK];
                p.palette = None;
                chase::running(pixels, state, &p);
            }
            Effect::MerryChristmas => {
                let mut p = *params;
                p.colors = [RED, GREEN, BLACK];
                p.palette = None;
                chase::running(pixels, state, &p);
            }
            Effect::Halloween => {
                let mut p = *params;
                p.colors = [PURPLE, ORANGE, BLACK];
                p.palette = None;
                chase::running(pixels, state, &p);
            }
            Effect::RunningRandom => chase::running_random(pixels, state, params),
            Effect::RunningRandom2 => chase::running_random2(pixels, state, params),
            Effect::RunningLights => dynamic::running_lights(pixels, state, params),

            Effect::RandomColor => dynamic::random_color(pixels, state, params),
            Effect::SingleDynamic => dynamic::single_dynamic(pixels, state, params),
            Effect::MultiDynamic => dynamic::multi_dynamic(pixels, state, params),
            Effect::BlockDissolve => dynamic::block_dissolve(pixels, state, params),

            Effect::Twinkle => twinkle::twinkle(pixels, state, params),
            Effect::TwinkleRandom => twinkle::twinkle_random(pixels, state, params),
            Effect::TwinkleFade => twinkle::twinkle_fade(pixels, state, params),
            Effect::TwinkleFadeRandom => twinkle::twinkle_fade_random(pixels, state, params),
            Effect::Sparkle => twinkle::sparkle(pixels, state, params),
            Effect::FlashSparkle => twinkle::flash_sparkle(pixels, state, params),

            Effect::LarsonScanner => scanner::larson_scanner(pixels, state, params),
            Effect::Comet => scanner::comet(pixels, state, params),
            Effect::DualLarson => scanner::dual_larson(pixels, state, params),
            Effect::RainbowLarson => scanner::rainbow_larson(pixels, state, params),
            Effect::MultiComet => scanner::multi_comet(pixels, state, params),

            Effect::Fireworks => fire::fireworks(pixels, state, params),
            Effect::FireworksRandom => {
                let mut p = *params;
                let rng = next_rand(state.aux);
                state.aux = rng;
                p.colors[0] = params.wheel(rng as u8);
                p.palette = None;
                fire::fireworks(pixels, state, &p);
            }
            Effect::FireFlicker => fire::fire_flicker(pixels, state, params),
            Effect::FireFlickerSoft => fire::fire_flicker_soft(pixels, state, params),
            Effect::FireFlickerIntense => fire::fire_flicker_intense(pixels, state, params),

            Effect::TwinkleFox => complex::twinkle_fox(pixels, state, params),
            Effect::Rain => complex::rain(pixels, state, params),
            Effect::Icu => complex::icu(pixels, state, params),
            Effect::FillerUp => complex::filler_up(pixels, state, params),
            Effect::TriFade => complex::trifade(pixels, state, params),
            Effect::Heartbeat => complex::heartbeat(pixels, state, params),
            Effect::RainbowFireworks => complex::rainbow_fireworks(pixels, state, params),
            Effect::SparkleRandom => twinkle::sparkle_random(pixels, state, params),

            Effect::Sine => wave::sine(pixels, state, params),
            Effect::Bpm => wave::bpm(pixels, state, params),
            Effect::Percent => wave::percent(pixels, state, params),
            Effect::Wavesins => wave::wavesins(pixels, state, params),
            Effect::SolidPattern => wave::solid_pattern(pixels, state, params),
        }
    }
}

/// Runs `draw` on the strip as the effect sees it: `params.group_len()` LEDs
/// per drawn pixel and, if reversed, back to front.
///
/// Between steps `pixels` holds the displayed frame, so the effect's view is
/// rebuilt from it first: effects that read back their previous frame find it
/// exactly as they drew it.
fn with_layout<P: Pixel>(pixels: &mut [P], params: &Params, draw: impl FnOnce(&mut [P])) {
    let group = params.group_len();
    let len = pixels.len();
    let drawn = len.div_ceil(group);

    // Collapse each group to its first LED. Reads stay ahead of writes.
    if group > 1 {
        for i in 1..drawn {
            pixels[i] = pixels[i * group];
        }
    }

    let view = &mut pixels[..drawn];
    if params.reverse {
        view.reverse();
    }
    draw(view);
    if params.reverse {
        view.reverse();
    }

    // Spread each drawn pixel over its group, from the far end back, so no
    // drawn pixel is overwritten before it is spread.
    if group > 1 {
        for i in (0..drawn).rev() {
            let color = pixels[i];
            let start = i * group;
            for pixel in &mut pixels[start..(start + group).min(len)] {
                *pixel = color;
            }
        }
    }
}
