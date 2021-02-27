extern crate linux_embedded_hal as hal;
extern crate lm75;

use hal::I2cdev;
use lm75::{Lm75, SlaveAddr};
fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = SlaveAddr::default();
    let mut sensor = Lm75::new(dev, address);
    let temperature = sensor.read_temperature().unwrap();
    println!("Temperature:  {}", temperature);
}
