//! A small example program that shows how to use the SSD1306 display

#![no_std]
#![no_main]

use cortex_m::asm::wfi;
use embassy_executor::Spawner;
use embassy_rp::{bind_interrupts, i2c, peripherals::I2C0};

use ese_2026_firmware::bme690;

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

bind_interrupts!(struct Irqs {
    I2C0_IRQ => i2c::InterruptHandler<I2C0>;
});

#[embassy_executor::main(
    executor = "embassy_rp::executor::Executor",
    entry = "cortex_m_rt::entry"
)]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let sda = p.PIN_4;
    let scl = p.PIN_5;

    info!("set up i2c");
    let i2c = i2c::I2c::new(p.I2C0, scl, sda, Irqs, i2c::Config::default());

    let mut bme = bme690::BME690::new(i2c);
    bme.init().await;

    loop {
        wfi();
    }
}
