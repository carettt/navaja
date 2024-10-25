use clap::Parser;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use tokio::{net::TcpStream, time::timeout};
use ipnet::Ipv4Net;

trait Target {
    async fn port_scan(&self, ports: &Vec<u16>) -> Vec<u16>;
}

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

enum DomainType {
    HOST,
    NETWORK
}

impl Target for Ipv4Addr {
    async fn port_scan(&self, ports: &Vec<u16>) -> Vec<u16> {
        let mut open_ports: Vec<u16> = Vec::new();
        
        for port in ports {
            let socket = SocketAddr::new(IpAddr::V4(*self), *port);

            println!("Attempting to connect to port {} on {}", port, self);

            match timeout(Duration::new(1, 0),
                TcpStream::connect(socket)).await {
                Ok(_) => { open_ports.push(*port) }
                Err(_) => { println!("Port {} closed.", port); }
            }
        }

        open_ports
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mode: DomainType = if args.network_scan { DomainType::NETWORK } else { DomainType::HOST };
    let mut hosts: Vec<Ipv4Addr>;

    match mode {
        DomainType::HOST => {
           hosts = args.target_domain.iter()
               .map(|h| h.parse::<Ipv4Addr>().unwrap())
               .collect();
        }

        DomainType::NETWORK => {
            hosts = Vec::new();
            args.target_domain.iter()
                .for_each(|n| {
                    n.parse::<Ipv4Net>().unwrap()
                        .hosts().into_iter().for_each(|h| hosts.push(h));
                });
        }
    }

    for host in &hosts {
        let open_ports = host.port_scan(&args.ports).await;

        for port in open_ports {
            println!("Port {} open!", port);
        }
    }
}
