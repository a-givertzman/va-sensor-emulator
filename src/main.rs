mod buffer;
mod angle;
mod amplitude;
mod stuff;
mod complex;
mod main_service;

#[cfg(test)]
mod tests;

use angle::Angle;
use buffer::Buffer;
//
//
fn main() {

    let buffer_len = 1024;      // to be loaded from the config.yaml
    let sample_freq = 320_000;  // to be loaded from the config.yaml
    let phase = 0.0;  // to be loaded from the config.yaml
    let mut buffer = Buffer::new(buffer_len);
    let mut angle = Angle::new(sample_freq, phase);
    loop {
        let angle_value = angle.add();
        match buffer.add(angle_value) {
            Some(_) => { // Some(_)
                // send buffer
                //println!("Created array is {:?}", buffer.array);
                //buffer.array.clear();
            }
            None => println!("Added value is {}", angle_value)
        };
    }
}
