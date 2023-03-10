use std::{env, net::UdpSocket, collections::HashMap, ops::Deref};
use protohackers_tcp_helper::cli_helper::Args;
use clap::Parser;

#[derive(Debug)]
struct UnusualDatabase {
    key_store: HashMap<String, String>
}

impl UnusualDatabase {
    fn new() -> UnusualDatabase {
        let mut key_store = HashMap::new();
        key_store.insert(String::from("version"), String::from("x kv store"));
        UnusualDatabase {
            key_store
        }
    }
    fn insert_key(&mut self, key: String, value: String) {
        self.key_store.insert(key, value);
    }

    fn retrieve_key(&self, key: &str) -> Option<&String> {
        self.key_store.get(key)
    }
}

fn server_init(port: u16) -> Result<UdpSocket, std::io::Error> {
    UdpSocket::bind(format!("{}:{port}", env::var("FLY_UDP_BIND_ADDR").unwrap_or_else(|_| String::from("0.0.0.0"))))
}

fn find_char_index(buf: &[u8], byte: u8) -> Option<usize> {
    buf.iter().position(|&c| c == byte)
}

fn receive_datagram(socket: &UdpSocket, buf: &mut [u8]) -> Result<(usize, String), std::io::Error> {
    let (num_bytes, src_addr) = socket.recv_from(buf)?;
    println!("Source:{};Num Bytes:{};Buf::{:?}", src_addr, num_bytes, &buf[..(num_bytes)]);
    Ok((num_bytes, src_addr.to_string()))
}

fn send_datagram(socket: &UdpSocket, data: &[u8], addr: String) -> Result<usize, std::io::Error> {
    println!("Sending data: {:?}", data);
    socket.send_to(data, addr)
}
 
fn main() {
    let args = Args::parse();
    let udp_socket = server_init(args.port).expect("Failed to bind to port");
    let mut unusual_database = UnusualDatabase::new();
    loop {
        let mut buf = [0; 1000];
        let (bytes_read, src_addr) = match receive_datagram(&udp_socket, &mut buf) {
            Ok(data) => data,
            Err(_) => (0, String::from("")),
        };
        let buf_length = buf.len();
        let equals_index = find_char_index(&buf, b'=').unwrap_or(buf_length);
        if !src_addr.is_empty() && equals_index == buf_length {
            let mut key = String::from_utf8(buf[..bytes_read].into()).unwrap();
            let response = match unusual_database.retrieve_key(&key) {
                Some(value) => {
                    key.push_str(&format!("={value}"));
                    key
                }
                None => {
                    key.push_str("=");
                    key
                }
            };
            let _ = send_datagram(&udp_socket, response.as_bytes(), src_addr);
            continue;
        }
        
        let key = String::from_utf8(buf[..equals_index].to_vec()).unwrap_or_else(|_| String::from(""));
        let value = String::from_utf8(buf[(equals_index + 1)..(equals_index + (bytes_read - equals_index))].to_vec()).unwrap_or_else(|_| String::from("")); 
        if key != *"version" {
            unusual_database.insert_key(key, value);
        }
    }
}