use std::fmt::Pointer;

pub enum Bme280Interface {
    /// SPI Interface
    Bme280SpiInterface,
    /// I2C Interface
    Bme280I2cInterface,
}

pub struct Bme280Settings {
    /// pressure oversampling 
    osr_p: u8,

    /// temperature oversampling 
    osr_t: u8,

    /// humidity oversampling 
    osr_h: u8,

    /// filter coefficient 
    filter: u8,

    /// standby time 
    standby_time: u8,
}

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

pub struct Bme280Dev {
    /// Chip Id
    chip_id: u8,
    /// Interface function pointer used to enable the device address for I2C and chip selection for SPI
    interface_ptr: *const dyn Pointer,

    /// Interface selection
    /// For SPI: Bme280SpiInterface
    interface: Bme280Interface,

    calibration_data: Bme280CalibrationData,
    settings: Bme280Settings,
    interface_result: i8,
}

impl Bme280Dev {
    /// Bus communication function pointer which should be mapped to
    /// the platform specific read functions of the user
    /// * @param [in] reg_addr      :Register address from which data is read.
    /// * @param [in] reg_data     : Pointer to data buffer where read data is stored.
    /// * @param [in] len           : Number of bytes of data to be read.
    /// * @param [in, out] intf_ptr : Void pointer that can enable the linking of descriptors
    ///                               for interface related call backs
    ///
    /// * @retval   0   -> Success.
    /// * @retval Non zero value -> Fail.
    pub fn read(reg_addr: u8, reg_data: u8, len: u32, interface_ptr: usize) -> usize {
        0
    }
    /// Bus communication function pointer which should be mapped to
    /// the platform specific write functions of the user
    ///
    /// * @param [in] reg_addr      : Register address to which the data is written.
    /// * @param [in] reg_data     : Pointer to data buffer in which data to be written
    ///                            is stored.
    /// * @param [in] len           : Number of bytes of data to be written.
    /// * @param [in, out] intf_ptr : Void pointer that can enable the linking of descriptors
    ///                            for interface related call backs
    ///
    /// * @retval   0   -> Success.
    /// * @retval Non zero value -> Fail.
    pub fn write(reg_addr: u8, reg_data: u8, len: u32, interface_ptr: usize) -> usize {
        0
    }
    /// Delay function pointer which should be mapped to
    /// delay function of the user
    ///
    /// * @param[in] period              : Delay in microseconds.
    /// * @param[in, out] intf_ptr       : Void pointer that can enable the linking of descriptors
    ///                                    for interface related call backs
    /// * @retval Non zero value -> Fail.
    pub fn delay(period: u32, interface_pointer: usize) -> usize {
        0
    }
}
pub fn bme280_init() {}
