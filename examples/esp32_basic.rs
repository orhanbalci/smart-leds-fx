//! Pseudo-code example showing how to wire smart-leds-fx to an ESP32.
//!
//! This file does not compile standalone — it is meant to show the integration
//! pattern for a real ESP32 project that brings in ws2812-esp32-rmt-driver
//! (or similar) alongside this crate.
//!
//! ```toml
//! [dependencies]
//! smart-leds-fx = { path = ".." }
//! ws2812-esp32-rmt-driver = "0.5"
//! esp-hal = { version = "0.18", features = ["esp32s3"] }
//! ```
//!
//! ```rust,ignore
//! use smart_leds_fx::{Effect, Ws2812Fx};
//! use smart_leds_trait::RGB8;
//! use ws2812_esp32_rmt_driver::Ws2812Esp32RmtDriver;
//!
//! const NUM_LEDS: usize = 60;
//!
//! fn main() {
//!     // Board-specific setup (clocks, RMT peripheral, GPIO pin) omitted.
//!     let mut driver = Ws2812Esp32RmtDriver::new(/* rmt channel, gpio pin */);
//!
//!     let mut fx: Ws2812Fx<NUM_LEDS> = Ws2812Fx::new(128); // 50 % brightness
//!     fx.set_effect(0, Effect::RainbowCycle);
//!     fx.set_speed(0, 20); // 20 ms between steps → fast rainbow
//!
//!     loop {
//!         let now_ms: u64 = /* read SystemTimer::now() / SystemTimer::TICKS_PER_MS */;
//!         if fx.service(now_ms) {
//!             driver.write(fx.iter()).unwrap();
//!         }
//!     }
//! }
//! ```

fn main() {
    // This file is documentation only — see the doc comment above.
}
