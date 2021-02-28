/// I2C addresses
pub const BME280_I2C_ADDR_PRIM: u8 = 0x76;
pub const BME280_I2C_ADDR_SEC: u8 = 0x77;

/// BME280 chip identifier
pub const BME280_CHIP_ID: u8 = 0x60;

/// Register address
pub const BME280_CHIP_ID_ADDR: u8 = 0xD0;
pub const BME280_RESET_ADDR: u8 = 0xE0;
pub const BME280_TEMP_PRESS_CALIB_DATA_ADDR: u8 = 0x88;
pub const BME280_HUMIDITY_CALIB_DATA_ADDR: u8 = 0xE1;
pub const BME280_PWR_CTRL_ADDR: u8 = 0xF4;
pub const BME280_CTRL_HUM_ADDR: u8 = 0xF2;
pub const BME280_CTRL_MEAS_ADDR: u8 = 0xF4;
pub const BME280_CONFIG_ADDR: u8 = 0xF5;
pub const BME280_DATA_ADDR: u8 = 0xF7;

/// API error codes
pub const BME280_E_NULL_PTR: i8 = -1;
pub const BME280_E_DEV_NOT_FOUND: i8 = -2;
pub const BME280_E_INVALID_LEN: i8 = -3;
pub const BME280_E_COMM_FAIL: i8 = -4;
pub const BME280_E_SLEEP_MODE_FAIL: i8 = -5;
pub const BME280_E_NVM_COPY_FAILED: i8 = -6;

/// API warning codes
pub const BME280_W_INVALID_OSR_MACRO: i8 = 1;

/// Macros related to size
pub const BME280_TEMP_PRESS_CALIB_DATA_LEN: u8 = 26;
pub const BME280_HUMIDITY_CALIB_DATA_LEN: u8 = 7;
pub const BME280_P_T_H_DATA_LEN: usize = 8;

/// Sensor power modes
pub const BME280_SLEEP_MODE: u8 = 0x00;
pub const BME280_FORCED_MODE: u8 = 0x01;
pub const BME280_NORMAL_MODE: u8 = 0x03;

/// Macros for bit masking
pub const BME280_SENSOR_MODE_MSK: u8 = 0x03;
pub const BME280_SENSOR_MODE_POS: u8 = 0x00;

pub const BME280_CTRL_HUM_MSK: u8 = 0x07;
pub const BME280_CTRL_HUM_POS: u8 = 0x00;

pub const BME280_CTRL_PRESS_MSK: u8 = 0x1C;
pub const BME280_CTRL_PRESS_POS: u8 = 0x02;

pub const BME280_CTRL_TEMP_MSK: u8 = 0xE0;
pub const BME280_CTRL_TEMP_POS: u8 = 0x05;

pub const BME280_FILTER_MSK: u8 = 0x1C;
pub const BME280_FILTER_POS: u8 = 0x02;

pub const BME280_STANDBY_MSK: u8 = 0xE0;
pub const BME280_STANDBY_POS: u8 = 0x05;

/// Sensor component selection macros
/// These values are internal for API implementation.
/// Don't relate this to data sheet.
pub const BME280_PRESS: u8 = 1;
pub const BME280_TEMP: u8 = 1 << 1;
pub const BME280_HUM: u8 = 1 << 2;
pub const BME280_ALL: u8 = 0x07;

/// Settings selection macros
pub const BME280_OSR_PRESS_SEL: u8 = 1;
pub const BME280_OSR_TEMP_SEL: u8 = 1 << 1;
pub const BME280_OSR_HUM_SEL: u8 = 1 << 2;
pub const BME280_FILTER_SEL: u8 = 1 << 3;
pub const BME280_STANDBY_SEL: u8 = 1 << 4;
pub const BME280_ALL_SETTINGS_SEL: u8 = 0x1F;

/// Oversampling macros
pub const BME280_NO_OVERSAMPLING: u8 = 0x00;
pub const BME280_OVERSAMPLING_1X: u8 = 0x01;
pub const BME280_OVERSAMPLING_2X: u8 = 0x02;
pub const BME280_OVERSAMPLING_4X: u8 = 0x03;
pub const BME280_OVERSAMPLING_8X: u8 = 0x04;
pub const BME280_OVERSAMPLING_16X: u8 = 0x05;

/// Measurement delay calculation macros
pub const BME280_MEAS_OFFSET: u16 = 1250;
pub const BME280_MEAS_DUR: u16 = 2300;
pub const BME280_PRES_HUM_MEAS_OFFSET: u16 = 575;
pub const BME280_MEAS_SCALING_FACTOR: u16 = 1000;

/// Standby duration selection macros
pub const BME280_STANDBY_TIME_0_5_MS: usize = 0x00;
pub const BME280_STANDBY_TIME_62_5_MS: usize = 0x01;
pub const BME280_STANDBY_TIME_125_MS: usize = 0x02;
pub const BME280_STANDBY_TIME_250_MS: usize = 0x03;
pub const BME280_STANDBY_TIME_500_MS: usize = 0x04;
pub const BME280_STANDBY_TIME_1000_MS: usize = 0x05;
pub const BME280_STANDBY_TIME_10_MS: usize = 0x06;
pub const BME280_STANDBY_TIME_20_MS: usize = 0x07;

/// Filter coefficient selection macros
pub const BME280_FILTER_COEFF_OFF: usize = 0x00;
pub const BME280_FILTER_COEFF_2: usize = 0x01;
pub const BME280_FILTER_COEFF_4: usize = 0x02;
pub const BME280_FILTER_COEFF_8: usize = 0x03;
pub const BME280_FILTER_COEFF_16: usize = 0x04;

pub const BME280_STATUS_REG_ADDR: usize = 0xF3;
pub const BME280_SOFT_RESET_COMMAND: usize = 0xB6;
pub const BME280_STATUS_IM_UPDATE: usize = 0x01;
