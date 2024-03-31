use std::process::Command;

use crate::HostFacts;

pub fn get_host_facts(host_facts_struct: &mut HostFacts) {
    host_facts_struct.system.hostname = String::from_utf8(Command::new("sh")
    .arg("-c")
    .arg("cat /etc/hostname")
    .output()
    .expect("ERROR: failed to get hostname").stdout).unwrap().replace("\n", "");

    return;
}
