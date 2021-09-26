use super::CodecError;

pub trait DecodeMutableRead<'a, R>: Sized
where
    R: std::io::Read,
{
    type Error;

    fn decode(reader: &'a mut R) -> Result<Self, Self::Error>;
}

pub trait DecodeMutableReadWithContext<'a, R, C>: Sized
where
    R: std::io::Read,
{
    type Error;

    fn decode(reader: &'a mut R, ctx: C) -> Result<Self, Self::Error>;
}

pub trait DecodeFixedLengthBytes<const N: usize>: Sized {
    type Error;

    fn decode(bytes: [u8; N]) -> Result<Self, Self::Error>;
}

pub trait DecodeFixedLengthBytesWithContext<C, const N: usize>: Sized {
    type Error;

    fn decode(bytes: [u8; N], ctx: C) -> Result<Self, Self::Error>;
}

pub trait DecodeVariableLengthBytes: Sized {
    type Error;

    fn decode(bytes: Vec<u8>) -> Result<Self, Self::Error>;
}

pub trait DecodeVariableLengthBytesWithContext<C>: Sized {
    type Error;

    fn decode(bytes: Vec<u8>, ctx: C) -> Result<Self, Self::Error>;
}

pub trait DecodeByteLength<const N: usize> {
    fn decode(bytes: [u8; N]) -> usize;
}

pub trait BinaryDecode<'a, R> {
    fn decode_mutable_read<D>(&'a mut self) -> Result<D, D::Error>
    where
        D: DecodeMutableRead<'a, R>,
        R: std::io::Read;

    fn decode_mutable_read_with<D, C>(&'a mut self, ctx: C) -> Result<D, D::Error>
    where
        D: DecodeMutableReadWithContext<'a, R, C>,
        R: std::io::Read;

    fn decode_fixed_length_bytes<D, const N: usize>(
        &'a mut self,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeFixedLengthBytes<N>;

    fn decode_fixed_length_bytes_with<D, C, const N: usize>(
        &'a mut self,
        ctx: C,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeFixedLengthBytesWithContext<C, N>;

    fn decode_variable_length_bytes<D, const N: usize>(
        &'a mut self,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeByteLength<N> + DecodeVariableLengthBytes;

    fn decode_variable_length_bytes_with<D, C, const N: usize>(
        &'a mut self,
        ctx: C,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeByteLength<N> + DecodeVariableLengthBytesWithContext<C>;

    fn decode_variable_length_bytes_with_length<D>(
        &'a mut self,
        length: usize,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeVariableLengthBytes;

    fn decode_variable_length_bytes_with_length_and<D, C>(
        &'a mut self,
        length: usize,
        ctx: C,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeVariableLengthBytesWithContext<C>;
}

impl<'a, R> BinaryDecode<'a, R> for R
where
    R: std::io::Read,
{
    fn decode_mutable_read<D>(&'a mut self) -> Result<D, D::Error>
    where
        D: DecodeMutableRead<'a, R>,
    {
        D::decode(self)
    }

    fn decode_mutable_read_with<D, C>(&'a mut self, ctx: C) -> Result<D, D::Error>
    where
        D: DecodeMutableReadWithContext<'a, R, C>,
        R: std::io::Read,
    {
        D::decode(self, ctx)
    }

    fn decode_fixed_length_bytes<D, const N: usize>(&'a mut self) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeFixedLengthBytes<N>,
    {
        let mut buf = [0; N];
        self.read_exact(&mut buf).map_err(CodecError::Io)?;
        D::decode(buf).map_err(CodecError::UserDefined)
    }

    fn decode_fixed_length_bytes_with<D, C, const N: usize>(
        &'a mut self,
        ctx: C,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeFixedLengthBytesWithContext<C, N>,
    {
        let mut buf = [0; N];
        self.read_exact(&mut buf).map_err(CodecError::Io)?;
        D::decode(buf, ctx).map_err(CodecError::UserDefined)
    }

    fn decode_variable_length_bytes<D, const N: usize>(
        &'a mut self,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeByteLength<N> + DecodeVariableLengthBytes,
    {
        let mut buf = [0; N];
        self.read_exact(&mut buf).map_err(CodecError::Io)?;
        let length: usize = <D as DecodeByteLength<N>>::decode(buf);

        let mut buf = vec![0; length];
        self.read_exact(&mut buf).map_err(CodecError::Io)?;
        <D as DecodeVariableLengthBytes>::decode(buf).map_err(CodecError::UserDefined)
    }

    fn decode_variable_length_bytes_with<D, C, const N: usize>(
        &'a mut self,
        ctx: C,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeByteLength<N> + DecodeVariableLengthBytesWithContext<C>,
    {
        let mut buf = [0; N];
        self.read_exact(&mut buf).map_err(CodecError::Io)?;
        let length: usize = <D as DecodeByteLength<N>>::decode(buf);

        let mut buf = vec![0; length];
        self.read_exact(&mut buf).map_err(CodecError::Io)?;
        <D as DecodeVariableLengthBytesWithContext<C>>::decode(buf, ctx)
            .map_err(CodecError::UserDefined)
    }

    fn decode_variable_length_bytes_with_length<D>(
        &'a mut self,
        length: usize,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeVariableLengthBytes,
    {
        let mut buf = vec![0; length];
        self.read_exact(&mut buf).map_err(CodecError::Io)?;
        D::decode(buf).map_err(CodecError::UserDefined)
    }

    fn decode_variable_length_bytes_with_length_and<D, C>(
        &'a mut self,
        length: usize,
        ctx: C,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeVariableLengthBytesWithContext<C>,
    {
        let mut buf = vec![0; length];
        self.read_exact(&mut buf).map_err(CodecError::Io)?;
        D::decode(buf, ctx).map_err(CodecError::UserDefined)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;
    use std::convert::TryInto;

    #[quickcheck]
    fn equivalent_when_decode_mut_read(value: u16) {
        impl<'a, R> DecodeMutableRead<'a, R> for u16
        where
            R: std::io::Read,
        {
            type Error = std::io::Error;

            fn decode(bytes: &'a mut R) -> Result<Self, Self::Error> {
                let mut buf = [0; 2];
                bytes.read_exact(&mut buf)?;
                Ok(u16::from_be_bytes(buf))
            }
        }

        let mut bytes = std::io::Cursor::new(value.to_be_bytes());
        let actual: u16 = bytes.decode_mutable_read().unwrap();
        assert_eq!(actual, value)
    }

    #[quickcheck]
    fn equivalent_when_decode_mut_read_with_ctx(value: u16) {
        impl<'a, R> DecodeMutableReadWithContext<'a, R, ()> for u16
        where
            R: std::io::Read,
        {
            type Error = std::io::Error;

            fn decode(bytes: &'a mut R, _: ()) -> Result<Self, Self::Error> {
                let mut buf = [0; 2];
                bytes.read_exact(&mut buf)?;
                Ok(u16::from_be_bytes(buf))
            }
        }

        let mut bytes = std::io::Cursor::new(value.to_be_bytes());
        let actual: u16 = bytes.decode_mutable_read_with(()).unwrap();
        assert_eq!(actual, value)
    }

    #[quickcheck]
    fn equivalent_when_decode_fixe_array(value: u16) {
        impl DecodeFixedLengthBytes<2> for u16 {
            type Error = std::convert::Infallible;

            fn decode(bytes: [u8; 2]) -> Result<Self, Self::Error> {
                Ok(u16::from_be_bytes(bytes))
            }
        }

        let mut bytes = std::io::Cursor::new(value.to_be_bytes());
        let actual: u16 = bytes.decode_fixed_length_bytes().unwrap();
        assert_eq!(actual, value)
    }

    #[quickcheck]
    fn equivalent_when_decode_fixe_array_with_ctx(value: u16) {
        impl DecodeFixedLengthBytesWithContext<(), 2> for u16 {
            type Error = std::convert::Infallible;

            fn decode(bytes: [u8; 2], _: ()) -> Result<Self, Self::Error> {
                Ok(u16::from_be_bytes(bytes))
            }
        }

        let mut bytes = std::io::Cursor::new(value.to_be_bytes());
        let actual: u16 = bytes.decode_fixed_length_bytes_with(()).unwrap();
        assert_eq!(actual, value)
    }

    #[quickcheck]
    fn equivalent_when_decode_variable_length_bytes_with_ctx(value: u16) {
        impl DecodeVariableLengthBytes for u16 {
            type Error = Vec<u8>;

            fn decode(bytes: Vec<u8>) -> Result<Self, Self::Error> {
                let bytes: [u8; 2] = bytes.try_into()?;
                Ok(u16::from_be_bytes(bytes))
            }
        }

        let mut bytes = std::io::Cursor::new(value.to_be_bytes());
        let actual: u16 = bytes.decode_variable_length_bytes_with_length(2).unwrap();
        assert_eq!(actual, value)
    }

    #[quickcheck]
    fn equivalent_when_decode_variable_length_bytes_with_length_ctx(value: u16) {
        impl DecodeVariableLengthBytesWithContext<()> for u16 {
            type Error = Vec<u8>;

            fn decode(bytes: Vec<u8>, _: ()) -> Result<Self, Self::Error> {
                let bytes: [u8; 2] = bytes.try_into()?;
                Ok(u16::from_be_bytes(bytes))
            }
        }

        let mut bytes = std::io::Cursor::new(value.to_be_bytes());
        let actual: u16 = bytes
            .decode_variable_length_bytes_with_length_and(2, ())
            .unwrap();
        assert_eq!(actual, value)
    }
}
