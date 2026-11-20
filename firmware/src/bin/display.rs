//! A small example program that shows how to use the SSD1306 display

#![no_std]
#![no_main]

use cortex_m::asm::wfi;
use embassy_executor::Spawner;
use embassy_rp::{bind_interrupts, i2c, peripherals::I2C1};
use embedded_graphics::{
    Drawable as _,
    geometry::Dimensions as _,
    mono_font::{MonoTextStyle, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    primitives::{Primitive as _, PrimitiveStyleBuilder, StrokeAlignment},
    text::{Alignment, Text},
};
use ssd1306::{
    I2CDisplayInterface, Ssd1306, mode::DisplayConfig as _, rotation::DisplayRotation,
    size::DisplaySize128x64,
};

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

bind_interrupts!(struct Irqs {
    I2C1_IRQ => i2c::InterruptHandler<I2C1>;
});

#[embassy_executor::main(
    executor = "embassy_rp::executor::Executor",
    entry = "cortex_m_rt::entry"
)]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let sda = p.PIN_26;
    let scl = p.PIN_27;

    info!("set up i2c");
    let i2c = i2c::I2c::new(p.I2C1, scl, sda, Irqs, i2c::Config::default());

    // Initialisierung des Displaytreibers gemäß embedded-graphics doku
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    // Auswahl einer auf dem Display lesbaren Schrift
    let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

    // Auswahl eines Rahmenstils für die bounding_box
    let border_stroke = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(3)
        .stroke_alignment(StrokeAlignment::Inside)
        .build();

    // Rahmen um den Rand des Displays zeichnen
    display
        .bounding_box()
        .into_styled(border_stroke)
        .draw(&mut display)
        .unwrap();

    // Zentrierten Text zeichnen
    let text = "ESE Kongress 2026";
    Text::with_alignment(
        text,
        display.bounding_box().center(),
        character_style,
        Alignment::Center,
    )
    .draw(&mut display)
    .unwrap();

    // Daten auf das Display schreiben.
    // Ändert nur die Teile, die sich seit dem letzten Aufruf verändert haben.
    display.flush().unwrap();

    loop {
        wfi();
    }
}
