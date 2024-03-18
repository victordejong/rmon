/**
Name: RMON-Client
Description: A client side collection tool for system metric collection.
Author: Victor de Jong <victor@victordejong.com>
*/

use chrono::{Local};
use std::{thread, time};

mod collectors;

mod data_structs;
use data_structs::{LiveMetrics, init_live_metrics_struct};

const SLEEP_DUR: time::Duration = time::Duration::from_millis(5000);

fn main() {
    println!("Starting RMON-Client on {}", Local::now().format("%Y-%m-%dT%H:%M:%S%Z"));
    println!("Getting CPU info with 5 second intervals...");

    let mut live_metrics: LiveMetrics = init_live_metrics_struct();

    loop {
        live_metrics = collectors::cpu::collect_cpu_data(live_metrics);
        
        println!("CPU: cores: {}, frequency average {} MHz", live_metrics.cpu.amount, live_metrics.cpu.freq);
        thread::sleep(SLEEP_DUR);
    }
}
