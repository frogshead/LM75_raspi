extern crate linux_embedded_hal as hal;
extern crate lm75;

use std::fs::OpenOptions;

use chrono::prelude::*;
use csv::WriterBuilder;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Temperature {
    temperature: f32,
    date_time: String,
}
use hal::I2cdev;
use lm75::{Lm75, SlaveAddr};
fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = SlaveAddr::default();
    let mut sensor = Lm75::new(dev, address);
    let temperature = sensor.read_temperature().unwrap();
    // println!("Temperature:  {}", temperature);
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("/home/mikko/temperatures.csv")
        .unwrap();
    let mut writer = WriterBuilder::new().has_headers(false).from_writer(file); // Writer::from_writer(file);
    let dt = Local::now().to_string();
    let temp = Temperature {
        temperature,
        date_time: dt,
    };
    writer.serialize(temp).unwrap();
}
