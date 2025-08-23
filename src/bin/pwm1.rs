// This example shows how to use PWM (Pulse Width Modulation) in the RP2040 chip.
// The on-board LED is used.

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::peripherals::{PIN_25, PWM_SLICE4};
use embassy_rp::pwm::{Config, Pwm};
use embassy_rp::Peri;
use embassy_time::Timer;
use panic_probe as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    spawner.spawn(pwm_task_pin25(p.PWM_SLICE4, p.PIN_25));
}

#[embassy_executor::task]
async fn pwm_task_pin25(slice4: Peri<'static, PWM_SLICE4>, pin25: Peri<'static, PIN_25>) {
    let mut c = Config::default();
    c.top = 32_768;
    c.compare_b = 8;
    let mut pwm = Pwm::new_output_b(slice4, pin25, c.clone());

    loop {
        Timer::after_secs(1).await;
        c.compare_b = c.compare_b.rotate_left(4);
        pwm.set_config(&c);
    }
}
