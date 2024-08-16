use std::{
  net::{IpAddr, Ipv4Addr, TcpListener, TcpStream},
  io::{prelude::*, BufReader},
};
use local_ip_address::local_ip;
use std::collections::HashMap;
use url::form_urlencoded;
use clap::Parser;

mod t5;

fn main() {
  let ip_addr = local_ip().unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
  let server = format!("{}:{}", ip_addr, 9071);
  println!("Http service: {}", server);
  let listener = TcpListener::bind(server).unwrap();

  for stream in listener.incoming() {
      let stream = stream.unwrap();

      println!("Connection established!");
      handle_connection(stream);
  }
}

fn parse_query_string(query: &str) -> HashMap<String, String> {
  let mut params = HashMap::new();
  for (key, value) in form_urlencoded::parse(query.as_bytes()) {
      params.insert(key.to_string(), value.to_string());
  }
  params
}

fn handle_connection(mut stream: TcpStream) {
  let buf_reader = BufReader::new(&mut stream);
  let request_line = buf_reader.lines().next().unwrap().unwrap();

  let mut response = String::new();
  response.push_str("Content-Type: text/plain\n\n");
  if request_line.starts_with("GET") {
      let status_line = "HTTP/1.1 200 OK";
      let url = request_line.split_whitespace().collect::<Vec<&str>>()[1];
      let query_string = url.split('?').skip(1).next().unwrap_or_default();
      let params = parse_query_string(query_string);
      if let Some(value) = params.get("prompt") {
          let args = t5::Args::parse();
          let prompt = value.clone();
          println!("prompt: {:?}", prompt);
          let contents = t5::run(args, prompt).expect("invalid run model");
          let length = contents.len();
          response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
      } else {
          response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n".to_string();
      }
  } else {
      let status_line = "HTTP/1.1 404 NOT FOUND";
      let contents = "404 not found";
      let length = contents.len();

      response =
          format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
  }
  stream.write_all(response.as_bytes()).unwrap();
}
