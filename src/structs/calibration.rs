
pub struct Bme280CalibrationData {
    /// Calibration coefficient for the temperature sensor
    dig1: u16,

    /// Calibration coefficient for the temperature sensor
    dig2: i16,

    /// Calibration coefficient for the temperature sensor
    dig3: i16,

    /// Calibration coefficient for the pressure sensor
    dig_p1: u16,

    /// Calibration coefficient for the pressure sensor
    dig_p2: i16,

    /// Calibration coefficient for the pressure sensor
    dig_p3: i16,

    /// Calibration coefficient for the pressure sensor
    dig_p4: i16,

    /// Calibration coefficient for the pressure sensor
    dig_p5: i16,

    /// Calibration coefficient for the pressure sensor
    dig_p6: i16,

    /// Calibration coefficient for the pressure sensor
    dig_p7: i16,

    /// Calibration coefficient for the pressure sensor
    dig_p8: i16,

    /// Calibration coefficient for the pressure sensor
    dig_p9: i16,

    /// Calibration coefficient for the humidity sensor
    dig_h1: u8,

    /// Calibration coefficient for the humidity sensor
    dig_h2: i16,

    /// Calibration coefficient for the humidity sensor
    dig_h3: u8,

    /// Calibration coefficient for the humidity sensor
    dig_h4: i16,

    /// Calibration coefficient for the humidity sensor
    dig_h5: i16,

    /// Calibration coefficient for the humidity sensor
    dig_h6: i8,

    /// Variable to store the intermediate temperature coefficient
    t_fine: i32,
}
