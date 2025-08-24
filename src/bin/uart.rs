// UART demo.
// GP0: UART0 Tx. GP1: UART0 Rx.

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::uart;
use embassy_time::{Duration, Timer};
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let config = uart::Config::default();
    let mut uart = uart::Uart::new_blocking(p.UART0, p.PIN_0, p.PIN_1, config);

    loop {
        uart.blocking_write("hello there!\r\n".as_bytes());
        //cortex_m::asm::delay(1_000_000);
        Timer::after(Duration::from_millis(2000)).await;
    }
}
