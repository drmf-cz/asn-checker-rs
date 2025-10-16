# asn-checker-rs

A Rust library for looking up Autonomous System Numbers (ASN) and related information for IPv4 and IPv6 addresses.
It fetches, parses, and caches ASN and IP mapping data from public sources, providing fast and convenient lookups for network applications.
Library was inspired by [asn-checker](https://github.com/ArcHound/asn-check).

## Features
- Lookup ASN information for IPv4 and IPv6 addresses
- Downloads and caches ASN and IP mapping data from public sources (APNIC, RIPE)
- Async API using `tokio`
- Structs for ASN and IP information with `serde` support
- Designed for integration into other Rust projects

## Installation
Add the following to your `Cargo.toml`:

```toml
[dependencies]
asn-checker-rs = { path = "." }
```

## Usage

```rust
use asn_checker_rs::checker::Checker;
use std::net::IpAddr;
use tokio;

#[tokio::main]
async fn main() {
    let mut checker = Checker::new();
    checker.init().await;
    let ip: IpAddr = "1.1.1.1".parse().unwrap();
    if let Some(info) = checker.search(&ip).await {
        println!("ASN: {} Name: {} Country: {} Net: {}", info.asn.id, info.asn.name, info.asn.cc, info.net);
    } else {
        println!("No ASN info found for this IP");
    }
}
```

## API Overview

### Checker
- `Checker::new()`: Create a new checker instance.
- `Checker::init(&mut self)`: Download and parse ASN data (async).
- `Checker::search(&self, ip: &IpAddr) -> Option<IpInfo>`: Lookup ASN info for an IP address.

### IpInfo
- `ip`: Option<IpAddr> — The queried IP address.
- `net`: IpNet — The network/subnet matched.
- `asn`: ASN — ASN information.

### ASN
- `id`: u32 — ASN number.
- `name`: String — ASN name.
- `cc`: String — Country code.

## Data Sources
- [APNIC IPV4](https://thyme.apnic.net/current/data-raw-table)
- [APNIC IPV6](https://thyme.apnic.net/current/ipv6-raw-table)
- [RIPE NCC](https://ftp.ripe.net/ripe/asnames/asn.txt)

## Testing
Run tests with:

```sh
cargo test
```

## Contributing
Contributions are welcome! Please open issues or pull requests.


