use std::{time::Duration};
use crate::hosts::{apply_block, clean_block};
use crate::permissions::{lock, unlock};
use clap::ValueEnum;
use crossterm::event::{ read, Event, KeyCode };
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use std::io::Write;

const PASSPHRASE_LEN: u16 = 1000;

#[derive(Debug, Clone, ValueEnum)]
pub enum Unit {
    Min,
    Hour,
}

/// Set block
///
/// Used to activate the block
pub fn set_block(amount: u64, unit: Unit) {
    let seconds = match unit {
        Unit::Min => amount * 60,
        Unit::Hour => amount * 60 * 60,
    };

    // Write to /etc/hosts
    apply_block();
    // Change the mode to read only
    lock();

    // Activate the time to block
    std::thread::sleep(Duration::from_secs(seconds));

    // Change the mode to read/write
    unlock();
    // Clean /etc/hosts
    clean_block();
}

/// Set unblock
///
/// Will unblock /etc/hosts after receiving a 1000 keystroke input
pub fn set_unblock() {
    println!("Type {} keystrokes to unblock...", PASSPHRASE_LEN);

    enable_raw_mode().unwrap();

    let mut count = 0;

    loop {
        // Receive keystrokes and show the count
        if let Event::Key(event) = read().unwrap() {
            match event.code {
                KeyCode::Enter => {}
                KeyCode::Char(_) => {
                    count += 1;
                    print!("\rProgress: {}/{}", count, PASSPHRASE_LEN);
                    std::io::stdout().flush().unwrap();
                }
                _ => {}
            }
        }

        // When count is greater the or equal to passphrase lenght
        if count >= PASSPHRASE_LEN {
            println!("Unblocked");

            disable_raw_mode().unwrap();

            // Change mode to read/write
            unlock();
            // Clean /etc/hosts
            clean_block();

            break;
        }
    }
}