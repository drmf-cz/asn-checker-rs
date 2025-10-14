use std::net::IpAddr;
use ipnet::IpNet;
use crate::structs::ASN;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct IpInfo {
    pub ip: Option<IpAddr>,
    pub net: IpNet,
    pub asn: ASN
}