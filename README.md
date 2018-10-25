# Rust SHT31-D Driver

Interface to the SHT31-D temperature and humidity sensor, based on the
[embedded-hal](https://github.com/japaric/embedded-hal) traits

# The Device

Sensirion Temperature/Humidity sensors are some of the finest & highest-accuracy
devices you can get. And, finally we have some that have a true I2C interface
for easy reading. The SHT31-D sensor has an excellent ±2% relative humidity and
±0.3°C accuracy for most uses.

Unlike earlier SHT sensors, this sensor has a true I2C interface, with two
address options. It also is 3V or 5V compliant, so you can power and
communicate with it using any microcontroller or microcomputer.

Datasheet: https://cdn-shop.adafruit.com/product-files/2857/Sensirion_Humidity_SHT3x_Datasheet_digital-767294.pdf

# License

Licensed under either of

+ Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
+ MIT License ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT) at your option.

# Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
