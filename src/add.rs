use std::fs::OpenOptions;
use std::io::Write;
use crate::constants::HOSTS_PATH;

// Add domain, receives the input of the user with the domain that he/she wants
// to be blocked, then the IPs will be placed in front of it and the domain-
// name will be written itself and one with .com appended to /etc/hosts
pub fn add_domain(domain: &str) {
    // TODO: Get root privaliges to write to /etc/hosts
    // Give the privaliges to append data to the right file, and raise an
    // error if appending is not possible
    let mut blocked_domains = OpenOptions::new()
        .append(true)
        .open(HOSTS_PATH)
        .expect("Cannot open file");

    // Concatenate the string into the right string order
    let formated_domain = format!(
        "\n127.0.0.1 {}\n127.0.0.1 {}.com",
        domain,
        domain
    );

    // Write the domain to the file
    blocked_domains
        .write_all(formated_domain.as_bytes())
        .expect("Writing to /etc/hosts failed");    

    println!("Appended domain(s) to /etc/hosts");
}