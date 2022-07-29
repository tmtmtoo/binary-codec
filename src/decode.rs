#[cfg(feature = "core2")]
use core2::io::{Error, Read};

#[cfg(feature = "core2")]
use alloc::{vec, vec::Vec};

#[cfg(feature = "std")]
use std::io::{Error, Read};

#[cfg(feature = "std")]
use std::{vec, vec::Vec};

pub trait TryDecode<Reader>: Sized {
    type Error;

    fn handle(reader: &mut Reader) -> Result<Self, Self::Error>;
}

pub trait TryDecodeWith<Reader, Context>: Sized {
    type Error;

    fn handle(reader: &mut Reader, ctx: Context) -> Result<Self, Self::Error>;
}

pub trait BinaryDecode<Reader> {
    fn try_decode<Decode>(&mut self) -> Result<Decode, Decode::Error>
    where
        Decode: TryDecode<Reader>;

    fn try_decode_with<Decode, Context>(&mut self, ctx: Context) -> Result<Decode, Decode::Error>
    where
        Decode: TryDecodeWith<Reader, Context>;
}

pub trait BinaryRead<Reader> {
    type Error;

    fn read_fixed_length<const N: usize>(&mut self) -> Result<[u8; N], Self::Error>;

    fn read_variable_length(&mut self, length: usize) -> Result<Vec<u8>, Self::Error>;
}

impl<Reader> BinaryDecode<Reader> for Reader
where
    Reader: Read,
{
    fn try_decode<Decode>(&mut self) -> Result<Decode, Decode::Error>
    where
        Decode: TryDecode<Reader>,
    {
        Decode::handle(self)
    }

    fn try_decode_with<Decode, Context>(&mut self, ctx: Context) -> Result<Decode, Decode::Error>
    where
        Decode: TryDecodeWith<Reader, Context>,
    {
        Decode::handle(self, ctx)
    }
}

impl<Reader> BinaryRead<Reader> for Reader
where
    Reader: Read,
{
    type Error = Error;

    fn read_fixed_length<const N: usize>(&mut self) -> Result<[u8; N], Self::Error> {
        let mut buf = [0; N];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn read_variable_length(&mut self, length: usize) -> Result<Vec<u8>, Self::Error> {
        let mut buf = vec![0; length];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[cfg(feature = "core2")]
    use core2::io::Cursor;

    #[cfg(feature = "std")]
    use std::io::Cursor;

    #[quickcheck]
    fn equivalent_when_read_fixed_length(value: u16) {
        let mut bytes = Cursor::new(value.to_be_bytes());
        let actual = bytes.read_fixed_length().map(u16::from_be_bytes).unwrap();
        assert_eq!(actual, value)
    }

    #[quickcheck]
    fn equivalent_when_read_variable_length(value: u16) {
        let mut bytes = Cursor::new(value.to_be_bytes());
        let actual = bytes
            .read_variable_length(2)
            .map(|bytes| u16::from_be_bytes([bytes[0], bytes[1]]))
            .unwrap();
        assert_eq!(actual, value)
    }

    #[quickcheck]
    fn equivalent_when_try_decode(value: u16) {
        impl<R> TryDecode<R> for u16
        where
            R: Read,
        {
            type Error = Error;

            fn handle(bytes: &mut R) -> Result<Self, Self::Error> {
                bytes.read_fixed_length().map(u16::from_be_bytes)
            }
        }

        let mut bytes = Cursor::new(value.to_be_bytes());
        let actual: u16 = bytes.try_decode().unwrap();
        assert_eq!(actual, value)
    }

    #[quickcheck]
    fn equivalent_when_try_decode_with_ctx(value: u16) {
        impl<R> TryDecodeWith<R, ()> for u16
        where
            R: Read,
        {
            type Error = Error;

            fn handle(bytes: &mut R, _: ()) -> Result<Self, Self::Error> {
                bytes.read_fixed_length().map(u16::from_be_bytes)
            }
        }

        let mut bytes = Cursor::new(value.to_be_bytes());
        let actual: u16 = bytes.try_decode_with(()).unwrap();
        assert_eq!(actual, value)
    }
}
