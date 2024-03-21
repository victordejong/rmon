use clap::{Parser, arg, command};
use regex::Regex;
use exitcode;
use config::{Config, File, Environment};
use std::path::Path;
use serde::Deserialize;

// Randomly picked interval
// Guaranteed to be random, chosen by fair D12 dice roll
const DEFAULT_INTERVAL: u64 = 10;
const DEFAULT_CONFIG_PATH: &str = "/etc/rmon/rmon-client.yaml";

/// RMON-Client: Remote MONitoring client. A simple tool for metrics monitoring. 
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CmdArgs {
    /// Path to a config file
    #[arg(short, long, default_value = DEFAULT_CONFIG_PATH)]
    pub config: String,

    /// Collection interval in seconds
    #[arg(short, long, default_value_t = DEFAULT_INTERVAL)]
    pub interval: u64,

    /// Remote collection server
    #[arg(short, long, value_name = "HOST:PORT")]
    pub rhost: Option<String>,

    /// Disks to be monitored, given as a comma separated list
    #[arg(short, long)]
    pub disks: Option<String>,
}

#[derive(Deserialize)]
// #[allow(unused)]
pub struct FileStruct {

    pub interval: Option<u64>,

    pub rhost: Option<String>,

    pub disks: Option<String>,
}

pub struct ConfigStruct {
    pub interval: u64,

    pub rhost: Option<String>,

    pub disks: Option<String>,
}

pub fn parse_config_sources() -> ConfigStruct {

    let cmd_args_struct: CmdArgs = CmdArgs::parse();

    let final_config = merge_config_sources(cmd_args_struct);

    return final_config;
}

fn validate_hostname_port(hostname_port: &String) {
    // Hostname validation based on https://man7.org/linux/man-pages/man7/hostname.7.html
    let re: regex::Regex = Regex::new(r"^[a-z0-9-\.]+\:[0-9]{1,5}$").unwrap();

    if !re.is_match(hostname_port) {
        println!("Error: Hostname/port combination not expected: {}", hostname_port);
        std::process::exit(exitcode::DATAERR);
    }

    return;
}


// Priority is as follows (lower overrides higher):
// 3. config on disk, 2. cmd line variables, 1. ENV variables
fn merge_config_sources(cmd_args_struct: CmdArgs) -> ConfigStruct {

    // Build the config from disk (if applicable)
    let file_config: FileStruct = Config::builder()
        .add_source(File::from(Path::new(&cmd_args_struct.config)).required(false))
        .build().unwrap().try_deserialize().unwrap();

    // Build the config from possible present ENV variables
    let env_config: FileStruct = Config::builder()
        .add_source(Environment::with_prefix("rmon_client"))
        .build().unwrap().try_deserialize().unwrap();

    let mut final_config = ConfigStruct {
        interval: DEFAULT_INTERVAL,
        rhost: None,
        disks: None,
    };

    final_config = override_variables(final_config, file_config, cmd_args_struct, env_config);

    return final_config;
}

fn override_variables(mut final_config: ConfigStruct, file_config: FileStruct,
    cmd_args_struct: CmdArgs, env_config: FileStruct) -> ConfigStruct {

    // Configure interval
    match file_config.interval {
        Some(interval) => { final_config.interval = interval; }
        None => (),
    };

    if cmd_args_struct.interval != DEFAULT_INTERVAL {
        final_config.interval = cmd_args_struct.interval;
    }

    match env_config.interval {
        Some(interval) => { final_config.interval = interval; }
        None => (),
    };

    // Configure RHOST
    match file_config.rhost {
        Some(hostname_port) => { final_config.rhost = Some(hostname_port); },
        None => (),
    };

     match cmd_args_struct.rhost {
        Some(hostname_port) => { final_config.rhost = Some(hostname_port); },
        None => (),
    };

    match env_config.rhost {
        Some(hostname_port) => { final_config.rhost = Some(hostname_port); },
        None => (),
    };

    // Check RHOST validity, if set
    match final_config.rhost {
        Some(ref hostname_port) => {validate_hostname_port(&hostname_port);}
        None => (),
    };

    // Configure disks
    match file_config.disks {
        Some(disks) => { final_config.disks = Some(disks); },
        None => (),
    };

     match cmd_args_struct.disks {
        Some(disks) => { final_config.disks = Some(disks); },
        None => (),
    };

    match env_config.disks {
        Some(disks) => { final_config.disks = Some(disks); },
        None => (),
    };

    return final_config;
}
