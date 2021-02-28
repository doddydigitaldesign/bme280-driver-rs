use crate::{constants::{addresses::{BME280_E_COMM_FAIL, BME280_E_INVALID_LEN, BME280_P_T_H_DATA_LEN}, codes::{BME280_INTF_RET_SUCCESS, BME280_OK}}, structs::device::Bme280Dev};

use super::init::Bme280Interface;

/// This API reads the data from the given register address of the sensor.
pub fn bme280_get_regs(reg_addr: u8, reg_data: [u8;BME280_P_T_H_DATA_LEN], len: usize, dev: Bme280Dev) -> i8 {
    let rslt: i8 = 0;
    // If interface selected is SPI
    if dev.interface != Bme280Interface::Bme280I2cInterface {
        reg_addr = reg_addr | 0x80;
    }

    // Read the data
    dev.interface_result = dev.read(reg_addr, reg_data, len, dev.interface_ptr);

    // Check for communication error
    if dev.interface_result != BME280_INTF_RET_SUCCESS {
        rslt = BME280_E_COMM_FAIL;
    }

    return rslt;
}

/// This API writes the given data to the register address of the sensor.
pub fn bme280_set_regs(reg_addr: [u8;20], reg_data: [u8;BME280_P_T_H_DATA_LEN], len: usize, dev: Bme280Dev) -> i8 {
    let rslt: i8 = 0;
    let temp_buff: [u8;20] = [0;20];

    if len > 10 {
        len = 10;
    }

    let mut temp_len: usize = 0;
    let reg_addr_cnt: u8 = 0;

    // Check for arguments validity
    if rslt == BME280_OK {
        if len != 0 {
            temp_buff[0] = reg_data[0];

            // If interface selected is SPI
            // if dev.interface != BME280_I2C_INTF {
                if dev.interface != Bme280Interface::Bme280I2cInterface {
                //  for (reg_addr_cnt = 0; reg_addr_cnt < len; reg_addr_cnt++)
                for reg_addr_cnt in 0..len {
                    reg_addr[reg_addr_cnt] = reg_addr[reg_addr_cnt] & 0x7F;
                }
            }

            // Burst write mode
            if len > 1 {
                // Interleave register address w.r.t data for burst write
                interleave_reg_addr(reg_addr, temp_buff, reg_data, len);
                temp_len = (len * 2) - 1;
            } else {
                temp_len = len;
            }

            dev.interface_result = dev.write(reg_addr[0], temp_buff, temp_len, dev.interface_ptr);

            // Check for communication error
            if dev.interface_result != BME280_INTF_RET_SUCCESS {
                rslt = BME280_E_COMM_FAIL;
            }
        } else {
            rslt = BME280_E_INVALID_LEN;
        }
    }

    return rslt;
}
