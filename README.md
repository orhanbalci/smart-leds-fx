# smart-leds-fx

A Rust port of the [WS2812FX](https://github.com/kitesurfer1404/WS2812FX) LED effect library, built on top of the [`smart-leds-trait`](https://crates.io/crates/smart-leds-trait) ecosystem.

> **⚠️ Not yet tested on real hardware.**
> This library compiles and runs correctly in the terminal simulator, but has not been validated against a physical LED strip or microcontroller. Use with caution and please report issues if you try it on real hardware.

---

## What is this?

WS2812FX is a popular Arduino library that brings 60+ animated effects to addressable LED strips. This crate ports that effect engine to Rust with a `no_std`-compatible design so it works across the entire `smart-leds` ecosystem — ESP32, RP2040, STM32, and any other board with a `SmartLedsWrite` driver.

The library sits between your effect logic and your hardware driver:

```
[ smart-leds-fx ]  ← effect engine (this crate)
        ↓  Iterator<Item = RGB8>
[ SmartLedsWrite ] ← hardware driver (ws2812-esp32-rmt-driver, etc.)
        ↓
[ LED strip ]
```

Your code calls `service(now_ms)` each loop iteration. When it returns `true`, a frame is ready — feed `iter()` to your driver.

---

## Features

- **91 effects**: 66 ported from WS2812FX, plus time-driven waves, patterns, motion and highlights
- `no_std` + `heapless` — no heap allocation, works on bare-metal
- Const-generic strip size — `StripFx<60>` sizes the pixel buffer at compile time
- Up to 10 independent segments, each with its own effect, speed, and colors
- Brightness scaling applied lazily in `iter()` — stored pixels are always full-brightness
- Segment builder API for clean configuration
- Hardware-agnostic — works with any `SmartLedsWrite` driver

---

## Quick start

```toml
[dependencies]
smart-leds-fx = "0.3"
```

```rust
use smart_leds_fx::prelude::*;

// 60-LED strip, brightness 200/255
let mut fx: StripFx<60> = StripFx::new(200);

fx.set_segment(0, Segment::new(0, 59, Effect::RainbowCycle).speed(50));

loop {
    let now_ms: u64 = /* milliseconds from your HAL timer */;
    if fx.service(now_ms) {
        driver.write(fx.iter()).unwrap();
    }
}
```

---

## Effects

| Category | Effects |
|---|---|
| Color | Static, Blink, Blink Rainbow, Strobe, Strobe Rainbow, Breath, Rainbow, Fade, Hyper Sparkle, Multi Strobe |
| Wipe / Scan | Color Wipe, Color Wipe Inv, Color Wipe Random, Color Sweep Random, Scan, Dual Scan |
| Chase | Tricolor Chase, Circus Combustus, Theater Chase, Theater Chase Rainbow, Bicolor Chase, Chase Color, Chase Blackout, Chase White, Chase Random, Chase Rainbow White, Chase Rainbow, Chase Blackout Rainbow, Chase Flash, Chase Flash Random |
| Running | Running Color, Running Red Blue, Merry Christmas, Halloween, Running Random, Running Random 2, Running Lights |
| Dynamic | Random Color, Single Dynamic, Multi Dynamic, Block Dissolve |
| Twinkle | Twinkle, Twinkle Random, Twinkle Fade, Twinkle Fade Random, Sparkle, Flash Sparkle, Sparkle Random |
| Scanner | Larson Scanner, Comet, Dual Larson, Rainbow Larson, Multi Comet |
| Fire | Fireworks, Fireworks Random, Fire Flicker, Fire Flicker (Soft), Fire Flicker (Intense) |
| Complex | TwinkleFOX, Rain, ICU, Filler Up, Tri Fade, Heartbeat, Rainbow Fireworks |
| Wave / Pattern | Sine, Bpm, Wavesins, Lake, Plasma, Twinkle Up (time-driven), Percent, Solid Pattern, Solid Pattern Tri |
| Motion | Saw, Bands, Stream, Stream 2, Gradient, Loading, Running Dual, Flow, Railway (time-driven), Tri Wipe |
| Highlights | Two Dots, Lightning, Spots, Spots Fade, Glitter — each draws over a background, or only its highlights with `overlay` — and Solid Glitter |

Iterate all effects at runtime:

```rust
for effect in Effect::iter() {
    println!("{}", effect.name());
}
```

---

## Segments

Split the strip into independently animated regions:

```rust
use smart_leds_fx::prelude::*;

let mut fx: StripFx<60> = StripFx::new(200);

// First 30 LEDs: fire
fx.set_segment(0, Segment::new(0, 29, Effect::FireFlicker)
    .speed(30)
    .color(rgb(255, 80, 0)));

// Last 30 LEDs: slow rainbow
fx.set_segment(1, Segment::new(30, 59, Effect::RainbowCycle)
    .speed(100));

// Swap the rainbow for ocean colors while it runs
fx.set_palette(1, Some(smart_leds_fx::color8::OCEAN_COLORS));
```

A segment's [palette](#palettes) replaces its primary color; build one in with
`Segment::palette`.

---

## Rendering into your own buffer

`StripFx` owns the pixels, the segments and the timing. To drive effects from
your own engine instead, step an effect directly into a buffer you keep:

```rust
use smart_leds_fx::prelude::*;

use smart_leds_fx::Params;

let mut pixels = [BLACK; 30];
let mut state = EffectState::default();
let params = Params::new([RED, BLACK, BLACK])
    .intensity(200) // shorter trail
    .size(1)        // each drawn pixel lights 2 LEDs
    .reverse(true);

// Call once per step, at whatever rate you choose.
Effect::LarsonScanner.step(&mut pixels, &mut state, &params);
```

Keep the same buffer and `state` between steps: many effects fade, shift or
restore what the previous step drew. `intensity` shapes each effect's
strength — trail length, fade rate, flicker depth — and the default of 128
draws its classic look.

The buffer can hold any type implementing `Pixel`, including
[`color8`](https://crates.io/crates/color8) `Crgb` pixels.

### Palettes

Give `Params` a 16-entry [`color8`](https://crates.io/crates/color8) palette
and it replaces the primary color:

```rust
use smart_leds_fx::color8::LAVA_COLORS;

let params = Params::new([RED, BLACK, BLACK]).palette(LAVA_COLORS);
Effect::Comet.step(&mut pixels, &mut state, &params);
```

Effects that draw the primary color follow the palette along the strip — the
comet above changes color as it travels. Effects that cycle the hue wheel
(Rainbow, Rainbow Cycle, the random-color effects) cycle through the palette
instead. Secondary and background colors stay as given, and effects with fixed
colors of their own (Circus Combustus, Chase White, Running Red Blue, Merry
Christmas, Halloween, Rainbow Fireworks), those whose colors are random RGB
(Running Random 2, Stream 2) and those that draw their own (Solid Pattern Tri,
Solid Glitter) ignore the palette; `Effect::uses_palette` says which. Without a
palette every effect draws exactly as before.

---

## Terminal simulator

Not sure which effect you want? Run the built-in terminal simulator to preview all 91 effects live in your terminal — no hardware needed:

```sh
cargo run --example terminal_sim
```

Controls:

| Key | Action |
|---|---|
| `←` / `p` | Previous effect |
| `→` / `n` | Next effect |
| `q` / `Esc` | Quit |

The simulator renders each LED as a colored `██` block using 24-bit ANSI color codes. Effect name and index are shown on the first line.

---

## ESP32 example

```toml
[dependencies]
smart-leds-fx = "0.3"
ws2812-esp32-rmt-driver = "0.5"
esp-hal = { version = "0.18", features = ["esp32s3"] }
```

```rust
use smart_leds_fx::prelude::*;
use ws2812_esp32_rmt_driver::Ws2812Esp32RmtDriver;

const NUM_LEDS: usize = 60;

// board setup omitted
let mut driver = Ws2812Esp32RmtDriver::new(rmt_channel, gpio_pin);

let mut fx: StripFx<NUM_LEDS> = StripFx::new(128);
fx.set_effect(0, Effect::RainbowCycle);
fx.set_speed(0, 20);

loop {
    let now_ms = SystemTimer::now() / SystemTimer::TICKS_PER_MS;
    if fx.service(now_ms) {
        driver.write(fx.iter()).unwrap();
    }
}
```

---

## Credits

Effect logic ported from [WS2812FX](https://github.com/kitesurfer1404/WS2812FX) by Harm Aldick, licensed MIT.

Built on [`smart-leds-trait`](https://crates.io/crates/smart-leds-trait) by the smart-leds contributors.

## License

MIT
