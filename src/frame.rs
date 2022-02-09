use bytes::Buf;
use std::io::Cursor;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] tokio::io::Error),
    #[error("Incomplete frame")]
    Incomplete,
}

// Format of a frame
// frame => <n><byte_1>...<byte_n>
// frames => <frame_1>...<frame_m>, where m is an integer greater than 1
#[derive(Debug)]
pub struct Frame {
    pub bytes: Vec<u8>,
}

impl Frame {
    pub fn check(src: &mut Cursor<&[u8]>) -> Result<(), Error> {
        let n = get_u8(src)?;

        for _ in 0..n {
            get_u8(src)?;
        }

        Ok(())
    }

    pub fn parse(src: &mut Cursor<&[u8]>) -> Result<Frame, Error> {
        let n = get_u8(src)?;
        let mut bytes = vec![];

        for _ in 0..n {
            bytes.push(get_u8(src)?);
        }

        Ok(Frame { bytes })
    }
}

fn get_u8(src: &mut Cursor<&[u8]>) -> Result<u8, Error> {
    if !src.has_remaining() {
        return Err(Error::Incomplete);
    }

    Ok(src.get_u8())
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_err {
        ($expression:expr, $($pattern:tt)+) => {
            match $expression {
                $($pattern)+ => (),
                ref e => panic!("expected `{}` but got `{:?}`", stringify!($($pattern)+), e),
            }
        }
    }

    #[test]
    fn it_returns_single_byte_frame() {
        let bytes = [1u8, 2];
        let mut src = Cursor::new(&bytes[..]);
        let result = Frame::check(&mut src);

        assert!(result.is_ok());
        assert_eq!(bytes.len() as u64, src.position());
    }

    #[test]
    fn it_returns_three_byte_frame() {
        let bytes = [3u8, 1, 2, 3];
        let mut src = Cursor::new(&bytes[..]);
        let result = Frame::check(&mut src);

        assert!(result.is_ok());
        assert_eq!(bytes.len() as u64, src.position());
    }

    #[test]
    fn it_returns_error() {
        let bytes = [2u8, 2];
        let mut src = Cursor::new(&bytes[..]);
        let result = Frame::check(&mut src);

        assert_err!(result.unwrap_err(), Error::Incomplete);
    }

    #[test]
    fn it_returns_error_when_array_is_empty() {
        let bytes = [];
        let mut src = Cursor::new(&bytes[..]);
        let result = Frame::check(&mut src);

        assert_err!(result.unwrap_err(), Error::Incomplete);
    }
}
