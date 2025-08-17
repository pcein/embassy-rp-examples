#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Duration, Timer};
use panic_probe as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Initialize GPIO pin 15 and 14 as a push-pull output.
    // These pins are usually labelled GP15 and GP14.
    let led15 = Output::new(p.PIN_15, Level::Low);
    let led14 = Output::new(p.PIN_14, Level::Low);
    spawner.spawn(blink_led(led15, Duration::from_millis(500)));
    spawner.spawn(blink_led(led14, Duration::from_millis(100)));
}

#[embassy_executor::task(pool_size = 2)]
async fn blink_led(mut led: Output<'static>, d: Duration) {
    loop {
        led.set_high();
        Timer::after(d).await;
        led.set_low();
        Timer::after(d).await;
    }
}
