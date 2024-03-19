use clap::{Parser, arg, command};
use regex::Regex;
use exitcode;

/// RMON-Client: Remote MONitoring client. A simple tool for metrics monitoring. 
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CmdArgs {
    /// Path to a config file
    #[arg(short, long, default_value = "/etc/rmon/rmon-client.yaml")]
    pub config: String,

    /// Collection interval in seconds
    #[arg(short, long, default_value_t = 10)]
    pub interval: u64,

    /// Remote collection server
    #[arg(short, long, value_name = "HOST:PORT")]
    pub rhost: Option<String>,
}

pub fn parse_cmd_args() -> CmdArgs {

    let cmd_args_struct: CmdArgs = CmdArgs::parse();

    match cmd_args_struct.rhost {
        Some(ref hostname_port) => validate_hostname_port(&hostname_port),
        None => (),
    }

    return cmd_args_struct;
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