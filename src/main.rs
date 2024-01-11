use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }  
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf: Vec<u8> = vec![];
    stream.read(&mut buf).expect("not ok");
    let req = String::from_utf8(buf).unwrap();
    println!("req={:?}", req);
    let ok: Vec<&str> = req.lines().collect();
    println!("{:?}", ok);
    // loop {
    //     let buf_reader = BufReader::new(&mut stream);
    //     let http_request: Vec<_> = buf_reader
    //         .lines()
    //         .map(|result| result.expect("not ok"))
    //         .take_while(|line| !line.is_empty())
    //         .collect();
    //     println!("Request: {:#?}", http_request);

    //     stream.write(b"ok\n").expect("failed to write");
    //}
 }
