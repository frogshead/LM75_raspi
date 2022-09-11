extern crate linux_embedded_hal as hal;
extern crate lm75;

use std::{time::SystemTime, thread::sleep_ms};


use hal::I2cdev;
use lm75::{Lm75, Address};
fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = Address::default();
    let mut sensor = Lm75::new(dev, address);

    loop {
        let temperature = sensor.read_temperature().unwrap();
        let unix_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        // printout as influxdb line protocol
        // https://docs.influxdata.com/influxdb/v1.7/write_protocols/line_protocol_reference/
        println!("lm75_measurement, name=lm75_kauhajoki temperature={} {}", temperature, unix_time);
        sleep_ms(1000* 10); //sleep 10s
        
    }
       
}
