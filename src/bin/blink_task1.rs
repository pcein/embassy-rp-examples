#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Duration, Timer};
use panic_probe as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Initialize GPIO pin 25 as a push-pull output.
    // This is the built-in LED on the Pico board.
    let mut led = Output::new(p.PIN_25, Level::Low);
    spawner.spawn(blink_led(led, Duration::from_millis(100)));
}

#[embassy_executor::task]
async fn blink_led(mut led: Output<'static>, d: Duration) {
    loop {
        led.set_high();
        Timer::after(d).await;
        led.set_low();
        Timer::after(d).await;
    }
}
