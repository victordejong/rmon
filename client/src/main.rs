/**
Name: RMON-Client
Description: A client side collection tool for system metric collection.
Author: Victor de Jong <victor@victordejong.com>
*/

use chrono::{Local};
use std::{thread, time};
use cpu_freq::CpuFreqs;

const SLEEP_DUR: time::Duration = time::Duration::from_millis(5000);

fn main() {
    println!("Starting RMON-Client on {}", Local::now().format("%Y-%m-%dT%H:%M:%S%Z"));
    println!("Getting CPU info with 5 second intervals...");
    loop {
        let cpus: Vec<CpuFreqs> = cpu_freq::get();
        let mut count: f32 = 0.;
        for (i, e) in cpus.iter().enumerate() {
            let cpu_freq_temp = match e.cur {
                None => panic!("No CPU value for {}", i),
                Some(element) => element,
            };
            count += cpu_freq_temp;
        }
        println!("Current average frequency: {}", count / cpus.len() as f32);
        thread::sleep(SLEEP_DUR);
    }
}
