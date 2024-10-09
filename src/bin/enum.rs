use clap::Parser;
use std::net::{Ipv4Addr, SocketAddr, ToSocketAddrs};
use iprange::IpRange;
use ipnet::Ipv4Net;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_delimiter=',')]
    ports: Vec<u16>,
    #[arg(short='N', long="network-scan", group="network", conflicts_with="host")]
    network_scan: bool,
    #[arg(short='H', long="host-scan", group="host", conflicts_with="network")]
    host_scan: bool,

    target_domain: Vec<String>,
}

struct NetworkRange {
    ip_range: IpRange<Ipv4Net>,
    ports: Vec<u16>,
}

enum DomainType {
    HOST,
    NETWORK
}

fn main() {
    let args = Args::parse();
    let scope = NetworkRange {
        ip_range: args.target_domain.iter()
            .map(|n| n.parse::<Ipv4Net>().unwrap())
            .collect(),
        ports: args.ports.to_owned(),
    };

    let networks: Vec<Vec<Ipv4Addr>> = scope.ip_range.into_iter()
        .map(|n| n.hosts().collect::<Vec<Ipv4Addr>>())
        .collect();

    for network in &networks {
        for host in network {
            println!("host: {}", host);
        }
    }

    print!("[");
    for i in 0..scope.ports.len()-1 {
        print!("{},", scope.ports[i]);
    }
    print!("{}", scope.ports[scope.ports.len()-1]);
    println!("]");

    if args.host_scan {
        println!("Finished host scan!");
    } else if args.network_scan {
        println!("Finished network scan!");
    }
}
