use std::fs;
use std::time::Duration;
use std::thread::sleep;
use std::os::unix::fs::PermissionsExt;
use crate::constants::HOSTS_PATH;
use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum Unit {
    Min,
    Hour
}

// Block domains asks for a time in minutes or hours depending on user input,
// Then the file is made writable only for the given amount of time, when the
// timer has run out of time the file becomes writable again.
pub fn block_domains(time: u64, unit: Unit) {
    let time_to_block = match unit {
        Unit::Min => time * 60,
        Unit::Hour => time * 60 * 60,
    };

    if time_to_block == 0 {
        return;
    }

    let unwritable = fs::Permissions::from_mode(0o444);
    let writable = fs::Permissions::from_mode(0o644);   

    fs::set_permissions(HOSTS_PATH, unwritable).unwrap(); // Set permission to
                                                          // read write

    sleep(Duration::from_secs(time_to_block)); // Run the timer

    fs::set_permissions(HOSTS_PATH, writable).unwrap();   // Set permision to 
                                                          // write only
}