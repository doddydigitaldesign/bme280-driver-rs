pub mod functions;
pub mod constants;
pub mod structs;

#[no_mangle]
pub extern "system" fn driver_entry() -> i8 {
    constants::codes::BME280_OK
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
