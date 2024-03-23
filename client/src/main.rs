/**
Name: RMON-Client
Description: A client side collection tool for system metric collection.
Author: Victor de Jong <victor@victordejong.com>
*/

use chrono::{Local};
use std::{thread, time};

mod collectors;
use collectors::{cpu, mem, disk};

mod data_structs;
use data_structs::live_metrics::{LiveMetrics, init_live_metrics_struct};
use data_structs::host_facts::{HostFacts, init_host_facts_struct};

mod config;

fn main() {

    let mut config_struct: config::ConfigStruct = config::parse_config_sources();

    println!("Starting RMON-Client on {}", Local::now().format("%Y-%m-%dT%H:%M:%S%Z"));
    println!("Using config options: interval: {}, rhost: {}", config_struct.interval,
        config_struct.rhost.unwrap_or(String::from("NONE")));

    let mut disks_configured: bool = false;

    match config_struct.disks {
        Some(ref conf_disks) => {
            println!("The following disks are configured to be monitored: {:?}", conf_disks);
            disks_configured = true;
        }
        None => {println!("No disks configured in config, disk monitoring disabled");},
    };

    let sleep_duration: time::Duration = time::Duration::from_secs(config_struct.interval);

    println!("Getting system info with {} second intervals...", config_struct.interval);

    let mut live_metrics: LiveMetrics = init_live_metrics_struct(&config_struct.disks);
    // TODO: Load all available disks in the hostfacs,
    //       give warning if disks are configured which are not present.
    let host_facts: HostFacts = init_host_facts_struct();

    loop {
        live_metrics = cpu::collect(live_metrics);
        live_metrics = mem::collect(live_metrics);

        if disks_configured {
            (live_metrics, config_struct.disks) = disk::collect(live_metrics, config_struct.disks.unwrap());
        }
        
        print_to_console(&live_metrics, &host_facts);

        thread::sleep(sleep_duration);
    }
}

fn print_to_console(live_metrics_struct: &LiveMetrics, host_facts_struct: &HostFacts) {

    // Print CPU details
    let mut cpu_freq_ext: String = String::from("MHz");
    let cpu_freq: f32 = if live_metrics_struct.cpu.freq > 1024. { cpu_freq_ext = String::from("GHz"); live_metrics_struct.cpu.freq / 1024. }
        else { live_metrics_struct.cpu.freq };
    let cpu_util: f32 = live_metrics_struct.cpu.cpu_load.load_1m / host_facts_struct.cpu.cores as f32;

    print!("CPU: cores: {}, freq avg {:.2} {}, load {} {} {}, util {:.1}%; ",
        host_facts_struct.cpu.cores, cpu_freq, cpu_freq_ext, live_metrics_struct.cpu.cpu_load.load_1m, live_metrics_struct.cpu.cpu_load.load_5m,
        live_metrics_struct.cpu.cpu_load.load_15m, cpu_util*100.);


    // Print memory details
    let mem_avail: u64 = live_metrics_struct.mem.available;
    let mem_cached: u64 = live_metrics_struct.mem.cached;
    let mem_used: u64 = host_facts_struct.mem.ram_total - live_metrics_struct.mem.free - mem_cached;

    if mem_avail > 10u64.pow(9) {
        if mem_cached > 1024 {
            print!("MEM: used {:.2} Gb, free {:.2} Gb, cached {:.2} Gb; ", mem_used as f32 / 1e9, mem_avail as f32 / 1e9, mem_cached as f32 / 1e9);
        } else{
            print!("MEM: used {:.2} Gb, free {:.2} Gb, cached {:.2} Mb; ", mem_used as f32 / 1e9, mem_avail as f32 / 1e9, mem_cached as f32 / 1e6);
        }
    } else {
        print!("MEM: used {:.2} Kb, free {:.2} Kb, cached {:.2} Kb; ", mem_used, mem_avail, mem_cached);
    }


    // Print disk details
    print!("DISK:");
    for disk in live_metrics_struct.disks.iter() {
        print!(" {}: total writes {}, total reads {}, current IO: {} |", disk.name, disk.total_writes,
                disk.total_reads, disk.current_io);
    }
    print!("; ");

    println!("");
    return;
}
