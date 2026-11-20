//! A minimal driver skeleton for the Bosch BME690 gas sensor.
//!
//! Refer to the datasheet for more info
//! <https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bme690-ds001.pdf>
//!
//! There are also drivers around for the BME680, which is quite similar.
//! You might get some inspiration from there
//! e.g. <https://github.com/embedded-drivers/embedded-drivers/blob/main/bme680/src/lib.rs>

use defmt::info;
/// 7-bit I2C address, SDO is HIGH
pub const ADDRESS: u8 = 0x77;

/// Expected chip ID register (`0xD0`) value
pub const CHIP_ID: u8 = 0x61;

/// Expected Variant ID for the BME690 (reg `0xF0`)
pub const VARIANT_ID: u8 = 0x02;

pub mod regs {
    /// `Chip_Id`
    pub const CHIP_ID: u8 = 0xD0;

    /// `Variant_Id`
    pub const VARIANT_ID: u8 = 0xF0;
}

/// BME690 driver
pub struct BME690<I2C: embedded_hal_async::i2c::I2c> {
    i2c: I2C,
}

impl<I2C: embedded_hal_async::i2c::I2c> BME690<I2C> {
    /// Create a driver for the BME690 sensor
    ///
    /// This does not configure anything on the sensor.
    /// We might need an `init` procedure to take care of some things before measuring...
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    pub async fn init(&mut self) {
        let mut chip_id = [0u8; 1];

        info!("Read CHIP_ID");
        // To read a register, we need to do a write_read, i.e., first write
        // the address of the register we want to read and then read from the bus.
        // There is a conveniece method for this in embedded-hal
        self.i2c
            .write_read(ADDRESS, &[regs::CHIP_ID], &mut chip_id)
            .await
            .unwrap();

        defmt::assert_eq!(chip_id[0], CHIP_ID, "Chip ID should match");

        let mut variant_id = [0u8; 1];
        info!("Read VARIANT_ID");
        self.i2c
            .write_read(ADDRESS, &[regs::VARIANT_ID], &mut variant_id)
            .await
            .unwrap();

        defmt::assert_eq!(variant_id[0], VARIANT_ID, "Variant ID should match");

        info!("CHIP_ID and VARIANT_ID match!");
    }
}
