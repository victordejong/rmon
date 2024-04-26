use chrono::Local;
use clap::{arg, command, Parser};
use config::{Config, Environment, File};
use regex::Regex;
use serde::Deserialize;
use std::path::Path;

const DEFAULT_CONFIG_PATH: &str = "/etc/rmon/rmon-server.yaml";
const DEFAULT_LISTEN_HOST: &str = "localhost:54432";

/// RMON-Client: Remote MONitoring client. A simple tool for metrics monitoring.
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CmdArgs {
    /// Path to a config file
    #[arg(short, long, default_value = DEFAULT_CONFIG_PATH)]
    pub config: String,

    /// host:port to listen on [default: localhost:54432]
    #[arg(short, long, value_name = "HOST:PORT")]
    pub listen_host: Option<String>,
}

#[derive(Deserialize)]
// #[allow(unused)]
pub struct FileStruct {
    pub listen_host: Option<String>,
}

pub struct ConfigStruct {
    pub listen_host: Option<String>,
}

trait ConfigFields {
    fn listen_host(&self) -> &Option<String>;
}

impl ConfigFields for FileStruct {
    fn listen_host(&self) -> &Option<String> {
        &self.listen_host
    }
}

impl ConfigFields for CmdArgs {
    fn listen_host(&self) -> &Option<String> {
        &self.listen_host
    }
}

pub fn parse_config_sources() -> ConfigStruct {
    let cmd_args_struct: CmdArgs = CmdArgs::parse();

    println!(
        "Starting RMON-Server on {}",
        Local::now().format("%Y-%m-%dT%H:%M:%S%Z")
    );

    let final_config = merge_config_sources(cmd_args_struct);

    return final_config;
}

fn validate_hostname_port(hostname_port: &String) {
    // Hostname validation based on https://man7.org/linux/man-pages/man7/hostname.7.html
    let re: regex::Regex = Regex::new(r"^[a-z0-9-\.]+\:[0-9]{1,5}$").unwrap();

    if !re.is_match(hostname_port) {
        println!(
            "Error: Hostname/port combination not expected: {}",
            hostname_port
        );
        std::process::exit(1);
    }

    return;
}

// Priority is as follows (lower overrides higher):
// 3. config on disk, 2. cmd line variables, 1. ENV variables
fn merge_config_sources(cmd_args_struct: CmdArgs) -> ConfigStruct {
    // Print warning if given config file does not exist on disk
    if !Path::new(&cmd_args_struct.config).exists() {
        println!(
            "WARNING: configured config file {} does not exist!",
            &cmd_args_struct.config
        );
    }

    // Build the config from disk (if applicable)
    let file_config: FileStruct = Config::builder()
        .add_source(File::from(Path::new(&cmd_args_struct.config)).required(false))
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap();

    // Build the config from possible present ENV variables
    let env_config: FileStruct = Config::builder()
        .add_source(Environment::with_prefix("rmon_server"))
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap();

    let mut final_config = ConfigStruct {
        listen_host: Some(String::from(DEFAULT_LISTEN_HOST)),
    };

    final_config = override_variables(final_config, file_config);
    final_config = override_variables(final_config, cmd_args_struct);
    final_config = override_variables(final_config, env_config);

    return final_config;
}

fn override_variables<T: ConfigFields>(mut final_config: ConfigStruct, config: T) -> ConfigStruct {
    // Configure RHOST
    match config.listen_host() {
        Some(hostname_port) => {
            final_config.listen_host = Some(String::from(hostname_port));
        }
        None => (),
    };

    // Check RHOST validity
    validate_hostname_port(&final_config.listen_host.clone().unwrap());

    return final_config;
}
