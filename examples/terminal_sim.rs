use std::io::{self, Write};
use std::time::{Duration, Instant};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute, queue,
    terminal::{self, ClearType},
};
use smart_leds_fx::prelude::*;

const NUM_LEDS: usize = 60;

fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let result = run(&mut stdout);

    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    result
}

fn run(stdout: &mut impl Write) -> std::io::Result<()> {
    let mut fx: StripFx<NUM_LEDS> = StripFx::new(200);
    let mut idx = 0usize;
    apply_effect(&mut fx, Effect::ALL[idx]);

    let start = Instant::now();

    loop {
        if event::poll(Duration::ZERO)? {
            if let Event::Key(key) = event::read()? {
                match (key.code, key.modifiers) {
                    (KeyCode::Char('q'), _)
                    | (KeyCode::Esc, _)
                    | (KeyCode::Char('c'), KeyModifiers::CONTROL) => break,
                    (KeyCode::Right | KeyCode::Char('n'), _) => {
                        idx = (idx + 1) % Effect::ALL.len();
                        apply_effect(&mut fx, Effect::ALL[idx]);
                    }
                    (KeyCode::Left | KeyCode::Char('p'), _) => {
                        idx = idx.checked_sub(1).unwrap_or(Effect::ALL.len() - 1);
                        apply_effect(&mut fx, Effect::ALL[idx]);
                    }
                    _ => {}
                }
            }
        }

        let now_ms = start.elapsed().as_millis() as u64;
        if fx.service(now_ms) {
            queue!(
                stdout,
                cursor::MoveTo(0, 0),
                terminal::Clear(ClearType::CurrentLine)
            )?;
            write!(
                stdout,
                " [{:>2}/{}] {:<28}  ← → change  q quit",
                idx + 1,
                Effect::ALL.len(),
                Effect::ALL[idx].name(),
            )?;

            queue!(stdout, cursor::MoveTo(0, 1))?;
            for pixel in fx.iter() {
                write!(
                    stdout,
                    "\x1b[38;2;{};{};{}m██\x1b[0m",
                    pixel.r, pixel.g, pixel.b
                )?;
            }
            stdout.flush()?;
        }

        std::thread::sleep(Duration::from_millis(5));
    }

    Ok(())
}

fn apply_effect(fx: &mut StripFx<NUM_LEDS>, effect: Effect) {
    fx.set_effect(0, effect);
    match effect {
        Effect::FireFlicker | Effect::FireFlickerSoft | Effect::FireFlickerIntense => {
            fx.set_colors(0, [rgb(255, 80, 0), rgb(180, 30, 0), BLACK]);
        }
        Effect::Rain => {
            fx.set_colors(0, [BLUE, BLACK, rgb(0, 100, 255)]);
        }
        Effect::Fade | Effect::TriFade => {
            fx.set_colors(0, [RED, BLUE, GREEN]);
        }
        _ => {
            fx.set_colors(0, [RED, BLUE, GREEN]);
        }
    }
    fx.set_speed(0, 40);
}
