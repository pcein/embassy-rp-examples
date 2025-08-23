// Two buttons (on GP16 and GP17) control two LED's (on GP14 and GP15)
// One button changes the blink rate and another one changes the pwm intensity
// of the corresponding LED.
#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::peripherals::{PIN_15, PWM_SLICE7};
use embassy_rp::pwm::{Config, Pwm};
use embassy_rp::Peri;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Duration, Timer};
use panic_probe as _;

const FAST_BLINK_DELAY_MS: u32 = 50;
const SLOW_BLINK_DELAY_MS: u32 = 500;
const DEBOUNCE_DELAY_MS: u64 = 20;
static BLINK_DELAY_MS: AtomicU32 = AtomicU32::new(SLOW_BLINK_DELAY_MS);
static CHANNEL: Channel<CriticalSectionRawMutex, u32, 1> = Channel::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    // Blink LED on GP14
    let led = Output::new(p.PIN_14, Level::Low);

    // Use GP16 and GP17 as input pins (pulled up).
    // You need to add your own push button switch between
    // GP16 and GND and also GP17 and GND.
    let button_blink = Input::new(p.PIN_16, Pull::Up);
    let button_pwm = Input::new(p.PIN_17, Pull::Up);
    spawner.spawn(led_blink_task(led));
    spawner.spawn(button_blink_task(button_blink));
    spawner.spawn(led_pwm_task(p.PWM_SLICE7, p.PIN_15));
    spawner.spawn(button_pwm_task(button_pwm));
}

// Blink LED on GP14
#[embassy_executor::task]
async fn led_blink_task(mut led: Output<'static>) {
    let mut delay: u32;
    loop {
        led.toggle();
        delay = BLINK_DELAY_MS.load(Ordering::Relaxed);
        Timer::after(Duration::from_millis(delay as u64)).await;
    }
}

// Button on GP16 changes blinking speed of LED on GP14
#[embassy_executor::task]
async fn button_blink_task(mut button: Input<'static>) {
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

// PWM LED on GP15
#[embassy_executor::task]
async fn led_pwm_task(slice7: Peri<'static, PWM_SLICE7>, pin15: Peri<'static, PIN_15>) {
    let mut c = Config::default();
    c.top = 32_768;
    c.compare_b = 8;
    let mut pwm = Pwm::new_output_b(slice7, pin15, c.clone());
    let msg_receiver = CHANNEL.receiver();
    loop {
        msg_receiver.receive().await;
        c.compare_b = c.compare_b.rotate_left(4);
        pwm.set_config(&c);
    }
}

// Button on GP17 changes brightness of LED on GP15
#[embassy_executor::task]
async fn button_pwm_task(mut button: Input<'static>) {
    let mut delay: u32;
    let msg_sender = CHANNEL.sender();
    loop {
        button.wait_for_falling_edge().await;
        Timer::after(Duration::from_millis(DEBOUNCE_DELAY_MS)).await;
        if button.is_high() {
            continue;
        }
        msg_sender.send(0).await; // wake up PWM task
    }
}
