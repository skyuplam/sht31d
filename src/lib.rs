//! A platform agnostic Rust driver for the Sensirion SGT3x-DIS Humidity and Temperature Sensor

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

const CRC8_POLYNOMIAL: u8 = 0x31;

/// Calculate the CRC8 checksum.
///
/// Implementation based on the reference implementation by Sensirion.
fn crc8(data: &[u8]) -> u8 {
    let mut crc: u8 = 0xff;
    for byte in data {
        crc ^= byte;
        for _ in 0..8 {
            if (crc & 0x80) > 0 {
                crc = (crc << 1) ^ CRC8_POLYNOMIAL;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc8_test_value() {
        assert_eq!(crc8(&[0xbe, 0xef]), 0x92);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
