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
