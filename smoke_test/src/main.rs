use protohackers_tcp_helper::{
    cli_helper::Args,
    errors::ProtoHackersError,
    tcp
};
use std::net::Shutdown;
use clap::Parser;

fn main() {
    let cmd_args = Args::parse();
    let listener = tcp::create_listener(cmd_args.port).unwrap();

    println!("Listening on port {}", cmd_args.port);

    for connection in listener.incoming() {
        // Does incoming keep looping while there are no connections ?
        match connection {
            Ok(mut stream) => {
                let data = tcp::read_stream_all(&stream);

                let (total_bytes, buf) = match data {
                    Err(err) => {
                        println!("Failed to read value from stream to a buffer!. {:?}", err);
                        (0, Vec::from("Failed to parse request"))
                    }
                    Ok(data) => data,
                };

                if let Err(err) = tcp::write_stream_all(&mut stream, &buf) {
                    println!("Failed to write value in buffer to stream!. {:?}", err);
                }

                println!(
                    "Total bytes read: {}\nRaw Bytes: {:?}\n",
                    total_bytes,
                    buf
                );

                match tcp::convert_to_utf8(buf) {
                    Ok(req_str) => println!("Buffer contents: {:?}", req_str),
                    Err(err) => println!("Failed to parse to UTF-8 {:?}", err),
                }

                tcp::shutdown_stream(&stream, Shutdown::Write);
            }
            Err(err) => {
                let err: ProtoHackersError = err.into();
                println!("Connection failed. {:?}", err);
            }
        }
    }
}
