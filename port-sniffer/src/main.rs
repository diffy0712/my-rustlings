use std::io::{self, Write};
use clap::Parser;
use std::sync::mpsc::{channel, Sender};
use std::net::IpAddr;
use tokio::net::TcpStream;
use tokio::task;

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

async fn scan(tx: Sender<u16>, port: u16, ip_addr: IpAddr) {
    match TcpStream::connect(format!("{}:{}", ip_addr, port)).await {
        Ok(_) => {
            println!(".");
            io::stdout().flush().unwrap();
            tx.send(port).unwrap();
        }
        Err(_) => {}
    }
}

#[tokio::main]
async fn main() {
    let Args{ip_address, start_port, end_port} = Args::parse();
 
    println!("Sniff for open ports on {} from port {} to {}!", ip_address, start_port, end_port);

    let (tx, rx) = channel();

    for i in start_port..end_port {
        let tx = tx.clone();

        task::spawn(async move { scan(tx, i, ip_address).await });
    }
    let mut out = vec![];
    drop(tx);

    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}