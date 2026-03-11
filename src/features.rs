use std::fs::File;
use std::io::Read;

fn list_websites() {
    // Read a /etc/hosts from the local file system
    let mut blocked_websites = File::open("/etc/hosts").unwrap();
    
    // Create a empty mutable string to copy contents of /etc/hosts to
    let mut file_content = String::new();
    
    // Copy contents of /etc/hosts to mutable string
    blocked_websites.read_to_string(&mut file_content).unwrap();

    println!("{}", file_content);
}

fn main() {
    list_websites();
}