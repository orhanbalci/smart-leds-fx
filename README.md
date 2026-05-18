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

- **68 effects** ported from WS2812FX
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
smart-leds-fx = "0.1"
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
```

---

## Terminal simulator

Not sure which effect you want? Run the built-in terminal simulator to preview all 68 effects live in your terminal — no hardware needed:

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
smart-leds-fx = "0.1"
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
