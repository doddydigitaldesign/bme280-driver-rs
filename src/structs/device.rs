use std::fmt::Pointer;

use crate::functions::init::Bme280Interface;

use super::{calibration::Bme280CalibrationData, settings::Bme280Settings};

pub struct Bme280Dev {
    /// Chip Id
    pub chip_id: u8,
    /// Interface function pointer used to enable the device address for I2C and chip selection for SPI
    pub interface_ptr: *const dyn Pointer,

    /// Interface selection
    /// For SPI: Bme280SpiInterface
    pub interface: Bme280Interface,

    pub calibration_data: Bme280CalibrationData,
    pub settings: Bme280Settings,
    pub interface_result: i8,
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
