mod checker;
use crate::checker::Checker;
mod structs;
mod misc;

use crate::structs::{IpInfo, ASN};
use tokio::sync::OnceCell;

use std::{net::IpAddr, str::FromStr};
use reqwest_middleware::{Result, ClientWithMiddleware};
use ipnet::IpNet;

static CHECKER: OnceCell<Checker> = OnceCell::const_new();

async fn get_checker() -> &'static Checker {
    CHECKER.get_or_init(|| async {
        let mut checker = Checker::new();
        checker.init().await;
        checker
    }).await
}

pub async fn check(address: IpAddr) -> Option<IpInfo> {
    let mut checker = Checker::new();
    checker.init().await;
    let r = checker.search(&address).await;
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn verify_ip_data(ip: &IpAddr, net: &IpNet, real: &IpInfo) -> Result<_> {
        let checker = get_checker().await;

        // Get the data from the internet
        let Some(found) = checker.search(ip).await else {
            panic!("Not found");
        };
        assert_eq!(found, *real);
        Ok(())
    }

    #[tokio::test]
    async fn check_ipv4() {
        // Set valid input
        let cfn_ip = IpAddr::from_str("1.1.1.1").unwrap();
        let cfn_net = IpNet::from_str("1.1.1.0/24").unwrap();

        let real = IpInfo {
            ip: Some(cfn_ip),
            net: cfn_net,
            asn: ASN {
                id: 13335,
                name: "CLOUDFLARENET,".to_string(),
                cc: "US".to_string()
            }
        };

        let _ = verify_ip_data(&cfn_ip, &cfn_net, &real).await;
    }

    #[tokio::test]
    async fn check_ipv6() {
        // Set valid input
        let cfn_ip = IpAddr::from_str("2001:678:19c::1").unwrap();
        let cfn_net = IpNet::from_str("2001:678:19c::/48").unwrap();

        let real = IpInfo {
            ip: Some(cfn_ip),
            net: cfn_net,
            asn: ASN {
                id: 13335,
                name: "CLOUDFLARENET,".to_string(),
                cc: "US".to_string()
            }
        };

        let _ = verify_ip_data(&cfn_ip, &cfn_net, &real).await;
    }
}
