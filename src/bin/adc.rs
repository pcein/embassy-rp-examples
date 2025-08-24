// ADC demo. GP29 is ADC input. On Pico board, this
// is connected to VSYS (through a voltage divider).
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

    let mut p29 = Channel::new_pin(p.PIN_29, Pull::None);

    loop {
        let level = adc.read(&mut p29).await.unwrap();
        info!("Pin 29 ADC: {}", level);
        Timer::after_secs(1).await;
    }
}
