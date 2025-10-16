use std::collections::{HashMap};
use std::error::Error;
use std::net::IpAddr;
use std::path::PathBuf;
use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use ipnet::IpNet;
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use std::str::FromStr;

use crate::structs::{ASN, IpInfo};
use crate::misc::{download};


// Links to current data on the internet
const ASN_IPV4: &str = "https://thyme.apnic.net/current/data-raw-table";
const ASN_IPV6: &str = "https://thyme.apnic.net/current/ipv6-raw-table";
const AS_NAMES: &str = "https://ftp.ripe.net/ripe/asnames/asn.txt";

pub struct Checker {
    db_v4: HashMap<IpNet, IpInfo>,
    db_v6: HashMap<IpNet, IpInfo>,
    asn: HashMap<u32, ASN>,
}

impl Checker {
    pub fn new() -> Self {
        let db_v4 = HashMap::new();
        let db_v6 = HashMap::new();
        let asn = HashMap::new();
        Checker { db_v4, db_v6, asn }
    }

    async fn build_ip_asn_pair(&self, client: &ClientWithMiddleware, url: &str, asns: &HashMap<u32, ASN>, delimiter: char) -> Result<HashMap<IpNet, IpInfo>, Box<dyn Error>> {
        let raw_asn_ip = download(&client, url).await.unwrap();
        let asn_ip: HashMap<IpNet, IpInfo> = raw_asn_ip.lines().map(|line| {
            let parts: Vec<&str> = line.split(delimiter).collect();
            let subnet= IpNet::from_str(parts.first().unwrap()).unwrap();
            let id = parts.last().unwrap().parse::<u32>().ok().unwrap();
            let Some(asn) = asns.get(&id) else {
                return (subnet, IpInfo {
                    ip: None,
                    net: IpNet::from_str("0.0.0.0/0").unwrap(),
                    asn: ASN {id: 0, name: "Unknown".to_string(), cc: "XX".to_string()}
                }
                )
            };
            {
                (
                    subnet,
                    IpInfo {
                        ip: None,
                        net: subnet,
                        asn: asn.clone()
                    }
                )
            }
        }).collect();
        Ok(asn_ip)
    }

    pub async fn init(&mut self) -> Option<()> {
        // download and parse the ASN database
        // Create cacheable client
        let client = ClientBuilder::new(Client::new())
            .with(Cache(HttpCache {
                mode: CacheMode::Default,
                manager: CACacheManager::new(PathBuf::from("./cache"), true),
                options: HttpCacheOptions::default(),
            }))
            .build();
        let asn_names = download(&client, AS_NAMES).await.unwrap();

        self.asn = Self::extract_asns(asn_names);
        self.db_v4 = self.build_ip_asn_pair(&client, ASN_IPV4, &self.asn, '\t').await.ok()?;
        self.db_v6 = self.build_ip_asn_pair(&client, ASN_IPV6, &self.asn, ' ').await.ok()?;

        Some(())
    }

    fn extract_asns(asn_names: String) -> HashMap<u32, ASN> {
        let _asns: Vec<ASN> = asn_names.lines().filter_map(|line| {
            if line.trim().is_empty() {
                return None;
            }
            let parts: Vec<&str> = line.splitn(3, ' ').collect();
            if parts.len() < 3 {
                return None;
            }
            let id = parts.first()?;
            let name = parts.get(1)?;
            let cc = parts.get(2)?;
            Some(ASN {
                id: id.parse::<u32>().ok()?,
                name: name.to_string(),
                cc: cc.to_string()
            })
        }).collect();
        let asns: HashMap<u32, ASN> = _asns.into_iter().map(|asn| { (asn.id, asn) }).collect();
        asns
    }

    pub async fn search(&self, ip: &IpAddr) -> Option<IpInfo> {
        let db = match ip {
            IpAddr::V4(_) => &self.db_v4,
            IpAddr::V6(_) => &self.db_v6,
        };

        for (net, ipinfo) in db.iter() {
            if net.contains(ip) {
                println!("Found IP {} with asn {:?}", ip, ipinfo);
                let mut result = ipinfo.clone();
                result.ip = Some(*ip);
                return Some(result);
            }
        }
        None
    }

    pub fn get_asn(&self, asn_id: u32) -> Option<&ASN> {
        self.asn.get(&asn_id)
    }
}
