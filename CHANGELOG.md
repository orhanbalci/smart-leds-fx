# Changelog

## Unreleased

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

- The 0.1 public API. Every other effect draws exactly the frames it drew in
  0.1.0.

## 0.1.0

- Initial release: 66 effects ported from WS2812FX, `StripFx` engine,
  segments, terminal simulator.
