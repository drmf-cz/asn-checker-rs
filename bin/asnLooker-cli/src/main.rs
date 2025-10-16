use std::net::IpAddr;

use clap::{Parser, Subcommand};

use asnLookup::checker::Checker;

#[derive(Parser, Debug)]
#[command(version, about)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Lookup by IP address
    Ip {
        ip: Option<IpAddr>,
    },
    /// Lookup by ASN number
    Asn {
        asn: Option<u32>,
    },
}


#[tokio::main(flavor = "current_thread")]
async fn main() {

    let cli = Cli::parse();

    let mut checker = Checker::new();
    checker.init().await;

    match cli.command {
        Commands::Ip { ip } => {
            let result = checker.search(&ip.unwrap()).await.unwrap();
            println!("{:?}", result);
        }
        Commands::Asn { asn } => {
            let result = checker.get_asn(asn.unwrap()).unwrap();
            println!("{:?}", result);
        }
    }


}
