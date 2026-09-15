# Changelog

## 0.3.1 — 2026-09-15

### Added

- Effects: Sine, Bpm, Percent, Wavesins and Solid Pattern. Sine, Bpm and
  Wavesins are time-driven: they draw from the clock rather than a step count.
- Named settings on `Params` for them: `rate`, `scale`, `fill`, `one_color`,
  `width`, `gap`, `variation`, `palette_start`, `palette_span` and
  `palette_step`, each with a default. `Params::now_ms` carries the clock, and
  `StripFx` fills it in.
- `Setting`, `Params::with` and `Effect::settings`: for callers that map
  controls of their own onto the settings each effect reads.

## 0.3.0 — 2026-09-15

### Added

- Palettes: `Params::palette` takes a 16-entry `color8::CrgbPalette16` that
  replaces the primary color. Effects that draw the primary color follow the
  palette along the strip; effects that cycle the hue wheel cycle through it.
  Effects with fixed colors of their own, and Running Random 2, ignore it.
  Without a palette every effect draws exactly as before.
- Palettes in `StripFx`: `Segment::palette` when building a segment, and
  `StripFx::set_palette` / `StripFx::get_palette` on a running strip.
- `Params::primary`, `Params::primary_at` and `Params::wheel`: the colors
  effects draw with, for callers writing their own effects.
- `smart_leds_fx::color8` re-exports the color8 release palettes come from,
  and the prelude exports `CrgbPalette16`.

### Changed

- **Breaking:** `EffectConfig` has a new `palette` field, so building one with
  a struct literal needs `palette: None` (or `..Default::default()`).
  Converting an `EffectConfig` into `Params` now carries its palette.
- **Breaking:** the `color8` feature is gone. `color8` is a regular
  dependency, and `Pixel` is always implemented for `color8::Crgb`; remove
  `features = ["color8"]` from your manifest.

## 0.2.0 — 2026-09-15

### Added

- `Pixel` trait: effects draw into buffers of any pixel type that implements
  it. Implemented for `RGB8`, and for `color8::Crgb` behind the new `color8`
  feature.
- `Params`: everything an effect reads on each step — colors, speed, and the
  new `intensity`, `size` and `reverse`. Built with `Params::new` and builder
  methods, or converted from `EffectConfig`.
- `Effect::step`: renders one step of an effect into a `Pixel` buffer with
  `Params`, for callers that schedule steps and own their buffers instead of
  using `StripFx`. `Effect::render` is unchanged and delegates to it.
- Intensity: trail length for the scanners and twinkle fades, fade rate for
  fireworks, rain and heartbeat, flicker depth for the fire flickers, sparkle
  count for Hyper Sparkle, hue span for Rainbow Cycle, flash rate for the
  strobes. The default (128) draws each effect as before.
- `size` and `reverse` now work for every effect. `SegmentOptions::size` and
  `SegmentOptions::reverse` take effect in `StripFx`; before, they were stored
  but ignored.
- `Segment::is_empty` (always `false`; segments are inclusive ranges).

### Changed

- **Breaking:** `Effect` is `#[non_exhaustive]`, so effects can be added in
  minor releases. A `match` over every variant needs a wildcard arm.
- Strobe and Strobe Rainbow are real strobes: a single-step flash followed by
  a pause. They drew the same frames as Blink and Blink Rainbow before.
- The sine used by Running Lights and TwinkleFOX is integer-only
  (`lib8tion::sin8`) instead of floating point, which is much cheaper on cores
  without an FPU. Their frames differ slightly.
- `micromath` is no longer a dependency; `lib8tion` is.

### Fixed

- Block Dissolve no longer indexes past its colors when `state.aux` holds 3 in
  its low bits (for example after `set_random_seed`).
- Stepping any effect over an empty pixel slice no longer panics.
- `StripFx::<0>` no longer underflows creating or resetting its segment.

### Unchanged

- The rest of the 0.1 public API. Every effect not listed above draws exactly
  the frames it drew in 0.1.0.

## 0.1.0

- Initial release: 66 effects ported from WS2812FX, `StripFx` engine,
  segments, terminal simulator.
