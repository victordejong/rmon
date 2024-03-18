/**
Name: RMON-Client
Description: A client side collection tool for system metric collection.
Author: Victor de Jong <victor@victordejong.com>
*/

use chrono::{Local};
use std::{thread, time};

mod collectors;
use collectors::{cpu, mem};

mod data_structs;
use data_structs::live_metrics::{LiveMetrics, init_live_metrics_struct};

const SLEEP_DUR: time::Duration = time::Duration::from_millis(5000);

fn main() {
    println!("Starting RMON-Client on {}", Local::now().format("%Y-%m-%dT%H:%M:%S%Z"));
    println!("Getting CPU info with 5 second intervals...");

    let mut live_metrics: LiveMetrics = init_live_metrics_struct();

    loop {
        live_metrics = cpu::collect(live_metrics);
        live_metrics = mem::collect(live_metrics);
        
        print_to_console(&live_metrics);

        thread::sleep(SLEEP_DUR);
    }
}

fn print_to_console(live_metrics_struct: &LiveMetrics) {

    let mut cpu_freq_ext: String = "MHz".to_string();
    let cpu_freq: f32 = if live_metrics_struct.cpu.freq > 1024. { cpu_freq_ext = "GHz".to_string(); live_metrics_struct.cpu.freq / 1024. } else { live_metrics_struct.cpu.freq };

    print!("CPU: cores: {}, freq avg {} {}, load {} {} {}; ",
        live_metrics_struct.cpu.amount, cpu_freq, cpu_freq_ext, live_metrics_struct.cpu.cpu_load.load_1m, live_metrics_struct.cpu.cpu_load.load_5m,
        live_metrics_struct.cpu.cpu_load.load_15m);


    let mem_free: u64 = live_metrics_struct.mem.free;
    let mem_cached: u64 = live_metrics_struct.mem.cached;

    if mem_free > 10u64.pow(9) && mem_cached > 10u64.pow(6) {
        print!("MEM: free {} Gb, cached {} Mb; ", mem_free as f32 / 1e9, mem_cached as f32 / 1e6);
    } else {
        print!("MEM: free {} Kb, cached {} Kb; ", mem_free, mem_cached);
    }
    println!("");
}
