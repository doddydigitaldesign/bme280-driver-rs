use crate::{
    constants::{
        addresses::{BME280_HUM, BME280_PRESS, BME280_TEMP},
        codes::BME280_OK,
    },
    structs::{
        calibration::Bme280CalibrationData,
        data::{Bme280Data, Bme280UncompensatedData},
    },
};

/// This API is used to compensate the pressure and/or
/// temperature and/or humidity data according to the component selected
/// by the user.
pub fn bme280_compensate_data(
    sensor_comp: u8,
    uncomp_data: Bme280UncompensatedData,
    comp_data: Bme280Data,
    calib_data: Bme280CalibrationData,
) -> i8 {
    // int8_t rslt = BME280_OK;
    let rslt = BME280_OK;

    /* Initialize to zero */
    comp_data.temperature = 0;
    comp_data.pressure = 0;
    comp_data.humidity = 0;

    /* If pressure or temperature component is selected */
    if (sensor_comp & (BME280_PRESS | BME280_TEMP | BME280_HUM)) != 0 {
        /* Compensate the temperature data */
        comp_data.temperature = compensate_temperature(uncomp_data, calib_data);
    }

    if (sensor_comp & BME280_PRESS) != 0 {
        /* Compensate the pressure data */
        comp_data.pressure = compensate_pressure(uncomp_data, calib_data);
    }

    if (sensor_comp & BME280_HUM) != 0 {
        /* Compensate the humidity data */
        comp_data.humidity = compensate_humidity(uncomp_data, calib_data);
    }

    rslt
}
