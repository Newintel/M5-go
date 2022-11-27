# M5-GO - ESP32

This crate provides a wrapper for M5Go devices.
It was made for an embedded development class and might not be updated.

You might however want to use it to implement your own driver for any M5Stack material, or only ESP32.

## Features

* Side LED bars handling (using the [Ws2812-Esp32-RmtDriver](https://github.com/cat-in-136/ws2812-esp32-rmt-driver) crate)
* Minimalist speaker use to play tones
* Screen handling (using the [Ili9341](https://github.com/yuri91/ili9341-rs) crate)
* Buttons handling
* Port C (UART Driver)

## Incoming features

* Bluetooth (Bluetooth Low Energy ?)
* Wifi ?

## How to use

This crate uses the ESP toolchain, for M5Stack systems are ESP32 systems.
If you have not installed it yet, please follow the [ESP Rust Book](https://esp-rs.github.io/book/).
