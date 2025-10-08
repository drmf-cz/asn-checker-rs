use std::{error::Error, net::IpAddr, str::FromStr};
use std::collections::BTreeMap;
use reqwest;
use http_cache;

// Links to current data on the internet
const ASN_IPV4: &str = "https://thyme.apnic.net/current/data-raw-table";
const ASN_IPV6: &str = "https://thyme.apnic.net/current/ipv6-raw-table";
const AS_NAMES: &str = "https://ftp.ripe.net/ripe/asnames/asn.txt";

#[derive(Debug, Clone, PartialEq, Eq)]
struct IpInfo {
    ip: IpAddr,
    asn: ASN
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ASN {
    id: u32,
    name: String,
    cc: String
}

struct Checker {
    db: BTreeMap<IpAddr, ASN>,
}

impl Checker {
    fn new() -> Self {
        let db = BTreeMap::new();
        Checker { db }
    }
}

impl Checker {
    fn init() -> Result<(), Box<dyn Error>> {
        // download and parse the ASN database


        Ok(())
    }
}

impl Checker {
    fn search(&self, ip: &IpAddr) -> Option<IpInfo> {
        let Some(asn_info) = self.db.get(ip) else {
            return None;
        };
        let result = IpInfo {
            ip: *ip,
            asn: asn_info.clone()
        };
        Some(result)
    }
}


pub fn check(address: IpAddr) -> Result<IpInfo, Box<dyn Error>> {
    let result: IpInfo = IpInfo {
        ip: address,
        asn: 0,
        name: String::from_str("asdf")?,

        cc: String::from_str("cz")?
    };
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _ = check(IpAddr::from_str("127.0.0.1").unwrap());
        assert_eq!(4, 4);
    }
}
