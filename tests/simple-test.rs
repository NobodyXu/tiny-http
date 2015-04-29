extern crate tiny_http;

use std::io::{Read, Write};

#[allow(dead_code)]
mod support;

#[test]
fn basic_handling() {
    let (server, mut stream) = support::new_one_server_one_client();
    write!(stream, "GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n").unwrap();

    let request = server.recv().unwrap();
    assert!(request.get_method().equiv(&"get"));
    //assert!(request.get_url() == "/");
    request.respond(tiny_http::Response::from_string(format!("hello world")));

    server.try_recv().unwrap();

    let mut content = String::new();
    stream.read_to_string(&mut content).unwrap();
    assert!(content.ends_with("hello world"));
}
