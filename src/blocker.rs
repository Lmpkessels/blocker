use std::{time::Duration};
use crate::hosts::{apply_block, clean_block};
use crate::permissions::{lock, unlock};
use clap::ValueEnum;
use crossterm::event::{ read, Event, KeyCode };
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use std::io::Write;

const PASSPHRASE_LEN: u16 = 10000;

#[derive(Debug, Clone, ValueEnum)]
pub enum Unit {
    Min,
    Hour,
}

pub fn set_block(amount: u64, unit: Unit) {
    let seconds = match unit {
        Unit::Min => amount * 60,
        Unit::Hour => amount * 60 * 60,
    };

    apply_block();
    lock();

    std::thread::sleep(Duration::from_secs(seconds));

    unlock();
    clean_block();
}

pub fn set_unblock() {
    println!("Type {} keystrokes to unblock...", PASSPHRASE_LEN);

    enable_raw_mode().unwrap();

    let mut count = 0;

    loop {
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

        if count >= PASSPHRASE_LEN {
            println!("Unblocked");

            disable_raw_mode().unwrap();

            unlock();
            clean_block();

            break;
        }
    }
}