use bytes::{Buf, BytesMut};
use sending_bytes::{Error, Frame};
use std::io::Cursor;
use tokio::io::{self, AsyncReadExt};
use tokio::net::TcpStream;

fn parse_frame(buffer: &mut BytesMut) -> Result<Option<Frame>, Error> {
    let mut src = Cursor::new(&buffer[..]);

    match Frame::check(&mut src) {
        Ok(_) => {
            let len = src.position() as usize;
            src.set_position(0);
            let frame = Frame::parse(&mut src)?;
            buffer.advance(len);
            Ok(Some(frame))
        }
        Err(Error::Incomplete) => Ok(None),
        Err(e) => Err(e),
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut buffer = BytesMut::with_capacity(2);
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    let mut n;

    loop {
        n = stream.read_buf(&mut buffer).await?;

        if n > 0 {
            match parse_frame(&mut buffer) {
                Ok(Some(frame)) => {
                    println!(" Received {:?}", frame.bytes)
                }
                Ok(None) => {
                    println!(" --- Incomplete frame")
                }
                Err(e) => eprintln!("{}", e),
            }
        } else {
            return Ok(());
        }
    }
}
