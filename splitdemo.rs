use std::io::{BufRead, Write};

use tcpsplit::*;

fn main() {
    let addr = "127.0.0.1:31750";

    let a = std::net::TcpListener::bind(addr).unwrap();
    let server = std::thread::spawn(move || {
        let (s, _) = a.accept().unwrap();
        let (mut r, mut w) = tcp_stream_pair(s).unwrap();
        let mut m = String::new();
        let _ = r.read_line(&mut m).unwrap();
        drop(r);
        let _ = m.pop();
        m += "!\n";
        w.write_all(m.as_bytes()).unwrap();
    });

    let s = std::net::TcpStream::connect(addr).unwrap();
    let (mut r, mut w) = tcp_stream_pair(s).unwrap();
    w.write_all(b"hello\n").unwrap();
    drop(w);
    let mut m = String::new();
    let _ = r.read_line(&mut m).unwrap();
    print!("{}", m);

    server.join().unwrap();
}
