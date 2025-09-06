// ADC demo. GP26 is ADC input.
#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::adc::{Adc, Channel, Config, InterruptHandler};
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::Pull;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(
    /// Binds the ADC interrupts.
    struct Irqs {
        ADC_IRQ_FIFO => InterruptHandler;
    }
);

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut adc = Adc::new(p.ADC, Irqs, Config::default());

    // Connect GP26 to GND. ADC reading may not be 0 - the reading
    // will show some GND offset (looks like there are some bugs in
    // the rp2040 adc implmentation). I was getting a reading less
    // than or equal to 14.
    //
    // Connect GP26 to 3v3(OUT), ADC read should return 4095.
    let mut p26 = Channel::new_pin(p.PIN_26, Pull::None);

    loop {
        let level = adc.read(&mut p26).await.unwrap();
        info!("Pin 26 ADC: {}", level);
        Timer::after_secs(1).await;
    }
}
