use std::net::UdpSocket;
use protohackers_tcp_helper::{
    cli_helper::Args
};
use clap::Parser;

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let udp_socket = UdpSocket::bind(format!("0.0.0.0:{}", args.port)).expect("Failed to bind to provided port");
    loop {
        let mut buf = [0; 2048];
        let _ = udp_socket.recv(&mut buf)?;
        if buf.len() > 0 {
            println!("{:?}", buf);
        }
    }
}