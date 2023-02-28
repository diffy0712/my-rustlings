use clap::Parser;
use std::net::IpAddr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Ip address to sniff for open ports on
    #[arg(short, long)]
    ip_address: IpAddr,

    /// Port to start sniffing from
    #[arg(short, long, default_value_t = 0)]
    start_port: u16,

    /// Port to stop sniffing at
    #[arg(short, long, default_value_t = 65535)]
    end_port: u16,
}


fn main() {
    let Args{ip_address, start_port, end_port} = Args::parse();
 
    println!("Sniff for open ports on {} from port {} to {}!", ip_address, start_port, end_port);
}