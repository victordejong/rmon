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
}

#[derive(Deserialize)]
#[allow(unused)]
pub struct ConfigStruct {

    pub config: Option<String>,

    pub interval: Option<u64>,

    pub rhost: Option<String>,
}

pub fn parse_config_sources() -> ConfigStruct {

    let cmd_args_struct: CmdArgs = CmdArgs::parse();

    match cmd_args_struct.rhost {
        Some(ref hostname_port) => validate_hostname_port(&hostname_port),
        None => (),
    }

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

fn merge_config_sources(cmd_args_struct: CmdArgs) -> ConfigStruct {

    let mut file_config: ConfigStruct = Config::builder()
        .add_source(Environment::with_prefix("rmon_client"))
        .add_source(File::from(Path::new(&cmd_args_struct.config)).required(false))
        .build().unwrap().try_deserialize().unwrap();

    if cmd_args_struct.interval != DEFAULT_INTERVAL {
        file_config.interval = Some(cmd_args_struct.interval);
    }

    // Ensure a default is always set for file.config.interval, even if the type is Option
    file_config.interval = Some(file_config.interval.unwrap_or(DEFAULT_INTERVAL));
    file_config.config = Some(file_config.config.unwrap_or(DEFAULT_CONFIG_PATH.to_string()));

    // TODO: validate rhost from file_config
    match cmd_args_struct.rhost {
        Some(hostname_port) => file_config.rhost = Some(hostname_port),
        None => (),
    };

    return file_config;
}