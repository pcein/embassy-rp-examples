// This example uses the RP Pico on board LED to test input pin GP16. This is not the button on the board.

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    // Built-in LED on the RP2040 board.
    let mut led = Output::new(p.PIN_25, Level::Low);

    // Use GP16 as input pin (pulled up).
    // You need to add your own push button switch between
    // GP16 and GND.
    let button = Input::new(p.PIN_16, Pull::Up);

    // button has two boolean methods defined on it: is_low() and
    // is_high().
    // led has two methods defined on it: set_high() and set_low().
    //
    // Complete the body of the loop: if button is kept pressed, led
    // will be high and if button is released, led will be low.
    loop {}
}
