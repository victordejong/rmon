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
        live_metrics = collectors::cpu::collect(live_metrics);
        live_metrics = collectors::mem::collect(live_metrics);
        
        print_to_console(&live_metrics);

        thread::sleep(SLEEP_DUR);
    }
}

fn print_to_console(live_metrics_struct: &LiveMetrics) {
    print!("CPU: cores: {}, freq avg {} MHz, load {} {} {}; ",
        live_metrics_struct.cpu.amount, live_metrics_struct.cpu.freq, live_metrics_struct.cpu.cpu_load.load_1m, live_metrics_struct.cpu.cpu_load.load_5m,
        live_metrics_struct.cpu.cpu_load.load_15m);

    if live_metrics_struct.mem.free > 10u64.pow(9) && live_metrics_struct.mem.cached > 10u64.pow(6) {
        print!("MEM: free {} Gb, cached {} Mb; ", live_metrics_struct.mem.free as f32 / 1e9, live_metrics_struct.mem.cached as f32 / 1e6);
    } else {
        print!("MEM: free {} Kb, cached {} Kb; ", live_metrics_struct.mem.free, live_metrics_struct.mem.cached);
    }
    println!("");
}
