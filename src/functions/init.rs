
use crate::{constants::{addresses::{BME280_CHIP_ID, BME280_CHIP_ID_ADDR, BME280_E_DEV_NOT_FOUND}, codes::BME280_OK}, structs::device::Bme280Dev};

use super::registers::bme280_get_regs;

#[derive(PartialEq)]
pub enum Bme280Interface {
    /// SPI Interface
    Bme280SpiInterface,
    /// I2C Interface
    Bme280I2cInterface,
}

/// This API is the entry point.
/// It reads the chip-id and calibration data from the sensor.
pub fn bme280_init(dev: &mut Bme280Dev) -> i8 {
    let mut result: i8 = 0;

    // Chip id read count
    let try_count: u8 = 5;
    let chip_id: u8 = 0;

    while try_count != 0
        {
            // Read the chip-id of bme280 sensor
            result = bme280_get_regs(BME280_CHIP_ID_ADDR, chip_id, 1, *dev);

            // Check for chip id validity
            if (result == BME280_OK) && (chip_id == BME280_CHIP_ID)
            {
                dev.chip_id = chip_id;

                // Reset the sensor 
                result = bme280_soft_reset(dev);

                if result == BME280_OK
                {
                    // Read the calibration data 
                    result = get_calib_data(dev);
                }

                break;
            }

            // Wait for 1 ms 
            dev.delay_us(1000, dev.interface_ptr);
            try_count -= 1;
        }

        // Chip id check failed 
        if !(try_count != 0)
        {
            result = BME280_E_DEV_NOT_FOUND;
        }

        result
}
