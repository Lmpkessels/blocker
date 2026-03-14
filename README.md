# Blocker

![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)
![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-red.svg)
![Status: in Progress](https://img.shields.io/badge/Status-in%20Progress-orange.svg)

Blocker is a CLI website blocker written in Rust for Linux-OS to block out domains for a given amount of time and remain focused.

## Installation

Make sure Rust is installed else follow the guide [Rust installation - The Rust Book](https://doc.rust-lang.org/book/ch01-01-installation.html).

```bash
git clone
cd blocker
```

## Usage

```bash
# Commands for the blocker
cargo run -- add domain
cargo run -- remove domain
cargo run -- list
cargo run -- block time (min or hour)
cargo run -- unblock
```

- When you **add a domain** through _cargo run -- add domain-name_ 127.0.0.1 domainname, and 127.0.0.1 domainname.com will be added to /etc/hosts

- When you **remove a domain** through _cargo run -- remove domain-name_ the file /etc/hosts will be overwritten

- When you **list** _through cargo run -- list domain-name_ the domain name /etc/hosts is shown so you can see all domains that are blocked

- When you use **block** you type the amount of time you want to block with the unit of time so for example if you want to block for 30 min you write _cargo run -- block 30 min_ if you want to block 2 hour then you write cargo -- run block 2 hour

- When you want to unblock the domains you've blocked you simply run _cargo run -- unblock_ then you type a 500 word passphrase to unblock the system

## Future features

Making sure this runs after rebooting the system and killing the running process becomes harder for the given amount of time.

## Contribution

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

## License

Licensed under [MIT License](./LICENSE-MIT). <br/>
© 2026 Luuk Kessels
