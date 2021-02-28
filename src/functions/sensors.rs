use std::array;

use crate::{constants::{addresses::{BME280_DATA_ADDR, BME280_E_NULL_PTR, BME280_P_T_H_DATA_LEN, BME280_RESET_ADDR, BME280_SOFT_RESET_COMMAND, BME280_STATUS_IM_UPDATE, BME280_STATUS_REG_ADDR}, codes::BME280_OK}, functions::{
        data::bme280_compensate_data,
        registers::{bme280_get_regs, bme280_set_regs},
    }, structs::{
        data::{Bme280Data, Bme280UncompensatedData},
        device::Bme280Dev,
    }};

// /*!
//  * @brief This API sets the oversampling, filter and standby duration
//  * (normal mode) settings in the sensor.
//  */
//  int8_t bme280_set_sensor_settings(uint8_t desired_settings, struct bme280_dev *dev)
//  {
//      int8_t rslt;
//      uint8_t sensor_mode;

//      /* Check for null pointer in the device structure*/
//      rslt = null_ptr_check(dev);

//      /* Proceed if null check is fine */
//      if (rslt == BME280_OK)
//      {
//          rslt = bme280_get_sensor_mode(&sensor_mode, dev);

//          if ((rslt == BME280_OK) && (sensor_mode != BME280_SLEEP_MODE))
//          {
//              rslt = put_device_to_sleep(dev);
//          }

//          if (rslt == BME280_OK)
//          {
//              /* Check if user wants to change oversampling
//               * settings
//               */
//              if (are_settings_changed(OVERSAMPLING_SETTINGS, desired_settings))
//              {
//                  rslt = set_osr_settings(desired_settings, &dev->settings, dev);
//              }

//              /* Check if user wants to change filter and/or
//               * standby settings
//               */
//              if ((rslt == BME280_OK) && are_settings_changed(FILTER_STANDBY_SETTINGS, desired_settings))
//              {
//                  rslt = set_filter_standby_settings(desired_settings, &dev->settings, dev);
//              }
//          }
//      }

//      return rslt;
//  }

//  /*!
//   * @brief This API gets the oversampling, filter and standby duration
//   * (normal mode) settings from the sensor.
//   */
//  int8_t bme280_get_sensor_settings(struct bme280_dev *dev)
//  {
//      int8_t rslt;
//      uint8_t reg_data[4];

//      /* Check for null pointer in the device structure*/
//      rslt = null_ptr_check(dev);

//      /* Proceed if null check is fine */
//      if (rslt == BME280_OK)
//      {
//          rslt = bme280_get_regs(BME280_CTRL_HUM_ADDR, reg_data, 4, dev);

//          if (rslt == BME280_OK)
//          {
//              parse_device_settings(reg_data, &dev->settings);
//          }
//      }

//      return rslt;
//  }

//  /*!
//   * @brief This API sets the power mode of the sensor.
//   */
//  int8_t bme280_set_sensor_mode(uint8_t sensor_mode, struct bme280_dev *dev)
//  {
//      int8_t rslt;
//      uint8_t last_set_mode;

//      /* Check for null pointer in the device structure*/
//      rslt = null_ptr_check(dev);

//      if (rslt == BME280_OK)
//      {
//          rslt = bme280_get_sensor_mode(&last_set_mode, dev);

//          /* If the sensor is not in sleep mode put the device to sleep
//           * mode
//           */
//          if ((rslt == BME280_OK) && (last_set_mode != BME280_SLEEP_MODE))
//          {
//              rslt = put_device_to_sleep(dev);
//          }

//          /* Set the power mode */
//          if (rslt == BME280_OK)
//          {
//              rslt = write_power_mode(sensor_mode, dev);
//          }
//      }

//      return rslt;
//  }

//  /*!
//   * @brief This API gets the power mode of the sensor.
//   */
//  int8_t bme280_get_sensor_mode(uint8_t *sensor_mode, struct bme280_dev *dev)
//  {
//      int8_t rslt;

//      /* Check for null pointer in the device structure*/
//      rslt = null_ptr_check(dev);

//      if ((rslt == BME280_OK) && (sensor_mode != NULL))
//      {
//          /* Read the power mode register */
//          rslt = bme280_get_regs(BME280_PWR_CTRL_ADDR, sensor_mode, 1, dev);

//          /* Assign the power mode in the device structure */
//          *sensor_mode = BME280_GET_BITS_POS_0(*sensor_mode, BME280_SENSOR_MODE);
//      }
//      else
//      {
//          rslt = BME280_E_NULL_PTR;
//      }

//      return rslt;
//  }

/// This API performs the soft reset of the sensor.
pub fn bme280_soft_reset(dev: Bme280Dev) -> i8 {
    let mut rslt: i8 = BME280_OK;
    let reg_addr = BME280_RESET_ADDR;
    let status_reg: u8 = 0;
    let mut try_run: u8 = 5;

    // 0xB6 is the soft reset command
    let soft_rst_cmd = BME280_SOFT_RESET_COMMAND;

    /* Write the soft reset command in the sensor */
    rslt = bme280_set_regs(&reg_addr, &soft_rst_cmd, 1, dev);

    if (rslt == BME280_OK) {
        /* If NVM not copied yet, Wait for NVM to copy */
        while (rslt == BME280_OK) && try_run != 0 && ((status_reg as usize) & BME280_STATUS_IM_UPDATE) != 0
        {
            /* As per data sheet - Table 1, startup time is 2 ms. */
            dev.delay_us(2000, dev.interface_ptr);
            rslt = bme280_get_regs(BME280_STATUS_REG_ADDR, &status_reg, 1, dev);
            try_run -= 1;
        }

        if (status_reg & BME280_STATUS_IM_UPDATE) {
            rslt = BME280_E_NVM_COPY_FAILED;
        }
    }
    rslt
}

/// This API reads the pressure, temperature and humidity data from the
/// sensor, compensates the data and store it in the bme280_data structure
/// instance passed by the user.
pub fn bme280_get_sensor_data(sensor_comp: u8, comp_data: Bme280Data, dev: Bme280Dev) -> i8 {
    //  int8_t rslt;
    let mut rslt: i8 = BME280_OK;

    /* Array to store the pressure, temperature and humidity data read from
     * the sensor
     */
    //  uint8_t reg_data[BME280_P_T_H_DATA_LEN] = { 0 };
    let reg_data = [0; BME280_P_T_H_DATA_LEN];

    let uncomp_data: Bme280UncompensatedData = Bme280UncompensatedData {
        humidity: 0,
        pressure: 0,
        temperature: 0,
    };

    if rslt == BME280_OK {
        /* Read the pressure and temperature data from the sensor */
        rslt = bme280_get_regs(BME280_DATA_ADDR, reg_data, BME280_P_T_H_DATA_LEN, dev);

        if rslt == BME280_OK {
            /* Parse the read data from the sensor */
            bme280_parse_sensor_data(reg_data, &mut uncomp_data);

            /* Compensate the pressure and/or temperature and/or
             * humidity data from the sensor
             */
            rslt =
                bme280_compensate_data(sensor_comp, uncomp_data, comp_data, dev.calibration_data);
        }
    } else {
        rslt = BME280_E_NULL_PTR;
    }

    return rslt;
}

/// This API is used to parse the pressure, temperature and
/// humidity data and store it in the bme280_uncomp_data structure instance.
pub fn bme280_parse_sensor_data(
    reg_data: [u8; BME280_P_T_H_DATA_LEN],
    uncomp_data: &mut Bme280UncompensatedData,
) {
    /* Variables to store the sensor data */
    let data_xlsb: u32;
    let data_lsb: u32;
    let data_msb: u32;
    /* Store the parsed register values for pressure data */
    data_msb = (reg_data[0] << 12) as u32;
    data_lsb = (reg_data[1] << 4) as u32;
    data_xlsb = (reg_data[2] >> 4) as u32;
    uncomp_data.pressure = data_msb | data_lsb | data_xlsb;

    /* Store the parsed register values for temperature data */
    data_msb = (reg_data[3] << 12) as u32;
    data_lsb = (reg_data[4] << 4) as u32;
    data_xlsb = (reg_data[5] >> 4) as u32;
    uncomp_data.temperature = data_msb | data_lsb | data_xlsb;

    /* Store the parsed register values for humidity data */
    data_msb = (reg_data[6] << 8) as u32;
    data_lsb = reg_data[7] as u32;
    uncomp_data.humidity = data_msb | data_lsb;
}
