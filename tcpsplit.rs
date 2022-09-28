pub use std::io::{self, BufReader, BufWriter};
pub use std::net::TcpStream;

pub fn tcp_stream_pair(s: TcpStream) -> io::Result<(BufReader<TcpStream>, BufWriter<TcpStream>)> {
    let t = s.try_clone()?;
    Ok((BufReader::new(s), BufWriter::new(t)))
}
