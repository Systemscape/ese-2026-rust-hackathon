//! A small example program that lights up the LED strip

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_time::Timer;
use gpio::{Level, Output};
use panic_probe as _;

#[embassy_executor::main(
    executor = "embassy_rp::executor::Executor",
    entry = "cortex_m_rt::entry"
)]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut leds = [
        Output::new(p.PIN_11, Level::High),
        Output::new(p.PIN_12, Level::High),
        Output::new(p.PIN_13, Level::High),
        Output::new(p.PIN_14, Level::High),
        Output::new(p.PIN_15, Level::High),
        Output::new(p.PIN_16, Level::High),
        Output::new(p.PIN_17, Level::High),
        Output::new(p.PIN_18, Level::High),
        Output::new(p.PIN_19, Level::High),
        Output::new(p.PIN_20, Level::High),
    ];

    loop {
        for led in &mut leds {
            info!("Toggling led");
            led.toggle();
            Timer::after_millis(500).await;
        }
    }
}
