//! A small example program that lights up the RGB LED

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_time::Timer;
use gpio::{Level, Output};

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

#[embassy_executor::main(
    executor = "embassy_rp::executor::Executor",
    entry = "cortex_m_rt::entry"
)]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut red = Output::new(p.PIN_21, Level::High);
    let mut green = Output::new(p.PIN_22, Level::High);
    let mut blue = Output::new(p.PIN_23, Level::High);

    // All LEDs are active-low, so we can also set inversion here
    // and then "high" means LED "on".
    //
    // Note: the red LED unfortunately never turns off entirely
    // due to the series resistor being a bit too small.
    red.set_inversion(true);
    green.set_inversion(true);
    blue.set_inversion(true);

    Timer::after_millis(1000).await;

    let mut leds = [red, green, blue];

    loop {
        for i in 0..leds.len() {
            info!("Toggling led");

            // Just for fun – not really worth it for 3 pins:
            // Use an iter_mut to change the pin state based on a condition
            leds.iter_mut().enumerate().for_each(|(idx, led)| {
                if idx == i {
                    led.set_high()
                } else {
                    led.set_low()
                }
            });

            Timer::after_millis(1500).await;
        }
    }
}
