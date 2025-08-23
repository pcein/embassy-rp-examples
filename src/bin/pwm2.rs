// This example shows how to use PWM (Pulse Width Modulation) in the RP2040 chip.
// The LED on GP15 is used.

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::peripherals::{PIN_15, PWM_SLICE7};
use embassy_rp::pwm::{Config, Pwm};
use embassy_rp::Peri;
use embassy_time::Timer;
use panic_probe as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    spawner.spawn(pwm_task_pin15(p.PWM_SLICE7, p.PIN_15));
}

#[embassy_executor::task]
async fn pwm_task_pin15(slice7: Peri<'static, PWM_SLICE7>, pin15: Peri<'static, PIN_15>) {
    let mut c = Config::default();
    c.top = 32_768;
    c.compare_b = 8;
    let mut pwm = Pwm::new_output_b(slice7, pin15, c.clone());

    loop {
        Timer::after_secs(1).await;
        c.compare_b = c.compare_b.rotate_left(4);
        pwm.set_config(&c);
    }
}
