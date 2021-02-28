/// BME280 sensor structure which comprises of uncompensated temperature,
/// pressure and humidity data
pub struct Bme280UncompensatedData {
    /// Un-compensated pressure
    pub pressure: u32,

    /// Un-compensated temperature
    pub temperature: u32,

    /// Un-compensated humidity
    pub humidity: u32,
}
/// BME280 sensor structure which comprises of temperature, pressure and
/// humidity data
pub struct Bme280Data {
    /// Compensated pressure
    pub pressure: u32,

    /// Compensated temperature
    pub temperature: i32,
    
    /// Compensated humidity
    pub humidity: u32,
}
