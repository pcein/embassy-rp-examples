// A basic PIO demo. Blinks GP25 LED.
// Note: purpose is to demonstrate PIO functionality with a simple program.
// This is not a good way to blink an LED using PIO.
#![no_std]
#![no_main]
use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio::program::pio_asm;
use embassy_rp::pio::{Common, Config, Direction, InterruptHandler, Pio, PioPin, StateMachine};
use embassy_rp::{bind_interrupts, Peri};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

fn setup_pio_task_sm0<'d>(
    pio: &mut Common<'d, PIO0>,
    sm: &mut StateMachine<'d, PIO0, 0>,
    pin: Peri<'d, impl PioPin>,
) {
    // Setup sm0

    // Send lsb of data from FIFO to pin.
    // The "pull" blocks if no new data is available on the FIFO.
    let prg = pio_asm!("loop:", "pull", "out pins, 1", "jmp loop");

    let mut cfg = Config::default();
    cfg.use_program(&pio.load_program(&prg.program), &[]);
    let out_pin = pio.make_pio_pin(pin);
    cfg.set_out_pins(&[&out_pin]);
    sm.set_pin_dirs(Direction::Out, &[&out_pin]);
    sm.set_config(&cfg);
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let pio = p.PIO0;

    let Pio {
        mut common,
        mut sm0,
        ..
    } = Pio::new(pio, Irqs);

    setup_pio_task_sm0(&mut common, &mut sm0, p.PIN_25);
    sm0.set_enable(true);

    let mut v = 1;
    loop {
        sm0.tx().push(v);
        v ^= 1;
        info!("Pushed {} to FIFO", v);
        Timer::after(Duration::from_millis(500)).await;
    }
}
