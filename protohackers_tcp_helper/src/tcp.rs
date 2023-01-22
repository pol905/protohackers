use std::io::{BufReader, BufRead};
use std::{
  io::{Read, Write},
  net::{Shutdown, TcpListener, TcpStream},
};

use crate::errors::ProtoHackersError;

pub fn create_listener(port: u16) -> Result<TcpListener, ProtoHackersError> {
  Ok(TcpListener::bind(format!("0.0.0.0:{port}"))?)
}

pub fn create_buf_reader(stream: &TcpStream) -> BufReader<&TcpStream> {
  BufReader::new(stream)
}

pub fn read_stream(stream: &TcpStream, delimiter: u8) -> Result<(usize, Vec<u8>), ProtoHackersError> {
  let mut buf: Vec<u8> = vec![];
  let mut reader = BufReader::new(stream);
  let total_bytes_read = reader.read_until(delimiter, &mut buf)?;
  Ok((total_bytes_read, buf))
}

pub fn read_stream_exact(reader: &mut BufReader<&TcpStream>, buf: &mut [u8]) -> Result<usize, ProtoHackersError> {
  Ok(reader.read(buf)?)
}

pub fn read_stream_all(stream: &TcpStream) -> Result<(usize, Vec<u8>), ProtoHackersError> {
  let mut buf = vec![];
  let mut reader = BufReader::new(stream);
  let total_bytes_read = reader.read_to_end(&mut buf)?;
  Ok((total_bytes_read, buf))
}


pub fn write_stream(stream: &mut TcpStream, buf: &[u8]) -> Result<usize, ProtoHackersError> {
  Ok(stream.write(buf)?)
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