// This example uses the RP Pico on board LED to test input pin GP16. This is not the button on the board.

#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_time::{Duration, Timer};
use panic_probe as _;

const FAST_BLINK_DELAY_MS: u32 = 50;
const SLOW_BLINK_DELAY_MS: u32 = 500;
const DEBOUNCE_DELAY_MS: u64 = 20;
static BLINK_DELAY_MS: AtomicU32 = AtomicU32::new(SLOW_BLINK_DELAY_MS);

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    // Built-in LED on the RP2040 board.
    let led = Output::new(p.PIN_25, Level::Low);

    // Use GP16 as input pin (pulled up).
    // You need to add your own push button switch between
    // GP16 and GND.
    let button = Input::new(p.PIN_16, Pull::Up);
    spawner.spawn(led_task(led));
    spawner.spawn(button_task(button));
}

#[embassy_executor::task]
async fn led_task(mut led: Output<'static>) {
    let mut delay: u32;
    loop {
        led.toggle();
        delay = BLINK_DELAY_MS.load(Ordering::Relaxed);
        Timer::after(Duration::from_millis(delay as u64)).await;
    }
}

#[embassy_executor::task]
async fn button_task(mut button: Input<'static>) {
    let mut delay: u32;

    loop {
        button.wait_for_falling_edge().await;
        Timer::after(Duration::from_millis(DEBOUNCE_DELAY_MS)).await;
        if button.is_high() {
            continue;
        }
        delay = BLINK_DELAY_MS.load(Ordering::Relaxed);
        if delay == SLOW_BLINK_DELAY_MS {
            delay = FAST_BLINK_DELAY_MS;
        } else if delay == FAST_BLINK_DELAY_MS {
            delay = SLOW_BLINK_DELAY_MS;
        }
        BLINK_DELAY_MS.store(delay, Ordering::Relaxed);
    }
}
