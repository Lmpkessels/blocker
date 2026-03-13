use std::fs::File;
use std::io::Read;
use crate::constants::HOSTS_PATH;

// List domains displays a list of all domains in /etc/hosts so the ones which
// are blocked
pub fn list_domains() {
    // Read /etc/hosts from the local file system
    let mut blocked_domain = File::open(HOSTS_PATH).unwrap();
    
    // Copy domain to /etc/hosts
    let mut domain = String::new();
    blocked_domain.read_to_string(&mut domain).unwrap();

    println!("{}", domain);
}