use clap::Parser;
use protohackers_tcp_helper::{
    cli_helper::Args,
    tcp, errors::ProtoHackersError
};
use std::{thread, io::{Read, Write}};
struct AssetPrice {
    timestamp: i32,
    price: i32
}

#[derive(Debug)]
struct Query {
    min_time: i32,
    max_time: i32
}

#[derive(Debug)]
struct AssetPrices {
    prices: Vec<(i32, i32)>
}

impl AssetPrice {
    fn new(timestamp:i32, price: i32) -> AssetPrice {
        AssetPrice { timestamp, price }
    }
}

impl Query {
    fn new(min_time: i32, max_time: i32) -> Query {
        Query {
            min_time,
            max_time
        }
    }
}

impl AssetPrices {
    fn new() -> AssetPrices {
        AssetPrices { prices: vec![] }
    }

    fn insert(&mut self, asset_price: AssetPrice) {
        self.prices.push((asset_price.timestamp, asset_price.price));
    }

    fn find_mean(&self, query: Query) -> i32 {
        let (min_time, max_time) = (query.min_time, query.max_time);
        if min_time > max_time {
            return 0
        }
        let (price, count) = self.prices
        .iter()
        .filter(|(timestamp, _)| *timestamp >= min_time && *timestamp <= max_time)
        .fold((0i64,0i64),|acc, (_, price)| (acc.0 + *price as i64, acc.1 + 1));
        if count > 0 { (price / count) as i32 } else { 0 } 
    }
}


fn main() {
    let args = Args::parse();
    let listener = tcp::create_listener(args.port).expect("Failed to bind to the provided port");

    for connection in listener.incoming() {
        thread::spawn(move || {
            match connection {
                Ok(mut tcp_stream) => {
                    let mut asset_prices = AssetPrices::new();
                    let mut buf = [0; 9];
                    // let mut tcp_reader = tcp::create_buf_reader(&tcp_stream);
                    // let mut tcp_writer = tcp::create_buf_writer(&tcp_stream);
                    loop {
                        let bytes_read = tcp_stream.read_exact(&mut buf);
                        if bytes_read.is_err() {
                            println!("Failed to read bytes");
                            break;
                        }

                        let req_type = buf[0];
                        let first_value: [u8; 4] = buf[1..5].try_into().unwrap();
                        let second_value: [u8; 4] = buf[5..].try_into().unwrap();
                        let first_value = i32::from_be_bytes(first_value);
                        let second_value = i32::from_be_bytes(second_value);
                        match req_type {
                            b'I' => {
                                let asset_price = AssetPrice::new(first_value, second_value);
                                asset_prices.insert(asset_price);
                            },
                            b'Q' => {
                                let query = Query::new(first_value, second_value);
                                let mean = asset_prices.find_mean(query);
                                let _ = tcp_stream.write(mean.to_be_bytes().as_slice());
                            },
                            _ => {}
                        }
                    }
                }
                Err(err) => {
                    let err: ProtoHackersError = err.into();
                    println!("Failed to connect to the client: {:?}", err);
                }
            }
        });
    }
}