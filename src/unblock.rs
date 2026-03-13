use std::fs::OpenOptions;
use crossterm::event::{ read, Event, KeyCode };

// Unblock domains through typing 500 keystrokes
//
// The keystrokes will be received after pressing enter, then a count takes
// place if the number is >= 500 /etc/hosts is cleared, else the remaining
// strokes need to be typed.
pub fn unblock_domains() {
    println!("To unblock the domains type 500 keystrokes");

    const PASSPHRASE_LEN: u16 = 500;
    let mut keystrokes_recieved = 0;

    loop {
        // Count keystrokes
        if let Event::Key(event) = read().unwrap() {
            match event.code {
                KeyCode::Enter => {}, // 'Enter' is not counted as a keystroke
                _ => keystrokes_recieved += 1
            }
        }

        // Clear /etc/hosts when at least 500 keystrokes are received
        // to unblock domains
        if keystrokes_recieved >= PASSPHRASE_LEN {
            let blocked_domains = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open("data.txt")
                .unwrap();

            println!("Domains unblocked");
            break;
        }
    }
}