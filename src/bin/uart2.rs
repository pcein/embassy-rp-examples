// UART and LED task running concurrently
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::uart;
use embassy_time::{Duration, Timer};
use panic_probe as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let led15 = Output::new(p.PIN_15, Level::Low);
    let config = uart::Config::default();
    let mut uart = uart::Uart::new_blocking(p.UART0, p.PIN_0, p.PIN_1, config);
    spawner.spawn(send_uart(uart));
    spawner.spawn(blink_led(led15, Duration::from_millis(100)));
}

#[embassy_executor::task]
async fn send_uart(mut uart: uart::Uart<'static, uart::Blocking>) {
    loop {
        uart.blocking_write("hello there!\r\n".as_bytes());
        Timer::after(Duration::from_millis(1000)).await;
    }
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
