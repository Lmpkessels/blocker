use std::fs;
use crate::constants::HOSTS_PATH;

// Remove domain, deletes the domain given as the argument to the parameter
pub fn remove_domain(domain: &str) {
    let content = fs::read_to_string(HOSTS_PATH).unwrap();

    // Create a new file of content without the domain given to remove
    let new_content: String = content
        .lines()
        .filter(|line| !line.contains(domain))
        .map(|line| format!("{line}\n"))
        .collect();

    // Write to file
    fs::write(HOSTS_PATH, new_content).unwrap();
}