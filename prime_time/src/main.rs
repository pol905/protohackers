use std::{net::Shutdown, thread};
use serde::{Serialize, Deserialize};
use protohackers_tcp_helper::{cli_helper::Args, tcp, errors::ProtoHackersError};
use clap::Parser;
use serde_json;

#[derive(Deserialize, Debug)]
struct PrimaryRequest {
  #[serde(default)]
  method: Option<String>,
  #[serde(default)]
  number: Option<f64>
}

#[derive(Serialize, Debug)]
struct PrimaryResponse {
  method: String,
  prime: bool
}

fn is_prime(num: f64) -> bool {
  if num.trunc() != num || num <= 1.0 {
    return false;
  }
  let trunc_num = num as u64;
  let end = num.sqrt();
  let trunc_end = end as u64;

  for div in 2..(trunc_end + 1) {
    if trunc_num % div == 0 {
      return false
    }
  }
  true
}

fn parse_json(request: &str) -> Result<PrimaryRequest, ProtoHackersError> {
  Ok(serde_json::from_str(request)?)
}

fn main() {
  let cmd_args = Args::parse();
  let listener = tcp::create_listener(cmd_args.port).unwrap();

  println!("Listening on port {}", cmd_args.port);

  for connection in listener.incoming() {
    thread::spawn(move || {
      match connection {
        Ok(mut tcp_stream)  => {
          println!("Connected to client {}", tcp_stream.peer_addr().unwrap());
          while let Ok(request) = tcp::read_stream(&tcp_stream, b'\n') {
            let (bytes_read, bytes) = request;
            
            if bytes_read == 0 {
              break;
            }
  
            let decoded_request = std::str::from_utf8(&bytes).map(|req| {
              if req == "null\n" {
                return "{}\n";
              }
              req
            }).unwrap_or("{}\n");
            println!("Decoded request: {}", decoded_request);
            
            let parsed_request: Result<PrimaryRequest, ProtoHackersError> = parse_json(decoded_request);
  
            if let Err(err) = parsed_request {
              println!("Failed to parse JSON payload: {:?}", err);
              let _ = tcp::write_stream(&mut tcp_stream, &[b'a', b'\n']);
              tcp::shutdown_stream(&tcp_stream, Shutdown::Both);
              break;
            }
  
            let normalize = parsed_request.unwrap();
            println!("Parsed JSON request: {:?}", normalize);
            
            match &normalize.method {
              Some(x) if *x == String::from("isPrime") => {}
              _ => {
                println!("method is not \"isPrime\"/was not defined");
                let _ = tcp::write_stream(&mut tcp_stream, &[b'a', b'\n']);
                tcp::shutdown_stream(&tcp_stream, Shutdown::Both);
                break;
              }
            }
            
            if let None = normalize.number {
              println!("number was not defined");
              let _ = tcp::write_stream(&mut tcp_stream, &[b'a', b'\n']);
              tcp::shutdown_stream(&tcp_stream, Shutdown::Both);
              break;
            }
  
            let response = PrimaryResponse {
              method: String::from("isPrime"),
              prime: is_prime(normalize.number.unwrap()),
            };
  
            let mut encoded_response = serde_json::to_vec(&response).unwrap();
            encoded_response.push(b'\n');
            let _ = tcp::write_stream(&mut tcp_stream, &encoded_response);
          }
        },
        Err(err) => {
          let err: ProtoHackersError = err.into();
          println!("Connection failed. {:?}", err);
        }
      }
    });
  }
}



