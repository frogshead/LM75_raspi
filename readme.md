
# README

Read LM75 i2c temperature sensor using Raspberry Pi 1 and write time stamp and the value to csv file.

To build for RPi 1

```bash
rustup target add arm-unknown-linux-gnueabi
git clone https://github.com/raspberrypi/tools $HOME/rpi_tools
RUSTFLAGS="-C linker=$HOME/rpi_tools/arm-bcm2708/arm-rpi-4.9.3-linux-gnueabihf/bin/arm-linux-gnueabihf-gcc" cargo build --target arm-unknown-linux-gnueabihf
```
