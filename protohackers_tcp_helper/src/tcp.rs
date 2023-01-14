use std::{
  io::{Read, Write},
  net::{Shutdown, TcpListener, TcpStream},
};

use crate::errors::ProtoHackersError;

pub fn create_listener(port: u16) -> Result<TcpListener, ProtoHackersError> {
  Ok(TcpListener::bind(format!("0.0.0.0:{port}"))?)
}

pub fn read_stream_all(stream: &mut TcpStream) -> Result<(usize, Vec<u8>), ProtoHackersError> {
  let mut buf = vec![];
  let total_bytes_read = stream.read_to_end(&mut buf)?;
  Ok((total_bytes_read, buf))
}

pub fn write_stream_all(stream: &mut TcpStream, buf: &[u8]) -> Result<(), ProtoHackersError> {
  Ok(stream.write_all(buf)?)
}

pub fn shutdown_stream(stream: &TcpStream, how: Shutdown) {
  stream.shutdown(how).unwrap();
}

pub fn convert_to_utf8(buf: Vec<u8>) -> Result<String, ProtoHackersError> {
  Ok(String::from_utf8(buf)?)
}