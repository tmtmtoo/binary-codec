use super::CodecError;

pub trait DecodeMutableRead<'a, R>: Sized
where
    R: std::io::Read,
{
    type Error;

    fn decode(bytes: &'a mut R) -> Result<Self, Self::Error>;
}

pub trait DecodeMutableReadWithContext<'a, R, C>: Sized
where
    R: std::io::Read,
{
    type Error;

    fn decode(bytes: &'a mut R, ctx: C) -> Result<Self, Self::Error>;
}

pub trait DecodeFixedArray<const N: usize>: Sized {
    type Error;

    fn decode(bytes: [u8; N]) -> Result<Self, Self::Error>;
}

pub trait DecodeFixedArrayWithContext<C, const N: usize>: Sized {
    type Error;

    fn decode(bytes: [u8; N], ctx: C) -> Result<Self, Self::Error>;
}

pub trait DecodeVector: Sized {
    type Error;

    fn decode(bytes: Vec<u8>) -> Result<Self, Self::Error>;
}

pub trait DecodeVectorWithContext<C>: Sized {
    type Error;

    fn decode(bytes: Vec<u8>, ctx: C) -> Result<Self, Self::Error>;
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

    fn decode_fixed_array<D, const N: usize>(&'a mut self) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeFixedArray<N>;

    fn decode_fixed_array_with<D, C, const N: usize>(
        &'a mut self,
        ctx: C,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeFixedArrayWithContext<C, N>;

    fn decode_vector<D>(&'a mut self, length: usize) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeVector;

    fn decode_vector_with<D, C>(
        &'a mut self,
        length: usize,
        ctx: C,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeVectorWithContext<C>;
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

    fn decode_fixed_array<D, const N: usize>(&'a mut self) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeFixedArray<N>,
    {
        let mut buf = [0; N];
        self.read_exact(&mut buf).map_err(CodecError::Io)?;
        D::decode(buf).map_err(CodecError::UserDefined)
    }

    fn decode_fixed_array_with<D, C, const N: usize>(
        &'a mut self,
        ctx: C,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeFixedArrayWithContext<C, N>,
    {
        let mut buf = [0; N];
        self.read_exact(&mut buf).map_err(CodecError::Io)?;
        D::decode(buf, ctx).map_err(CodecError::UserDefined)
    }

    fn decode_vector<D>(&'a mut self, length: usize) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeVector,
    {
        let mut buf = vec![0; length];
        self.read_exact(&mut buf).map_err(CodecError::Io)?;
        D::decode(buf).map_err(CodecError::UserDefined)
    }

    fn decode_vector_with<D, C>(
        &'a mut self,
        length: usize,
        ctx: C,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeVectorWithContext<C>,
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
        impl DecodeFixedArray<2> for u16 {
            type Error = std::convert::Infallible;

            fn decode(bytes: [u8; 2]) -> Result<Self, Self::Error> {
                Ok(u16::from_be_bytes(bytes))
            }
        }

        let mut bytes = std::io::Cursor::new(value.to_be_bytes());
        let actual: u16 = bytes.decode_fixed_array().unwrap();
        assert_eq!(actual, value)
    }

    #[quickcheck]
    fn equivalent_when_decode_fixe_array_with_ctx(value: u16) {
        impl DecodeFixedArrayWithContext<(), 2> for u16 {
            type Error = std::convert::Infallible;

            fn decode(bytes: [u8; 2], _: ()) -> Result<Self, Self::Error> {
                Ok(u16::from_be_bytes(bytes))
            }
        }

        let mut bytes = std::io::Cursor::new(value.to_be_bytes());
        let actual: u16 = bytes.decode_fixed_array_with(()).unwrap();
        assert_eq!(actual, value)
    }

    #[quickcheck]
    fn equivalent_when_decode_vector(value: u16) {
        impl DecodeVector for u16 {
            type Error = Vec<u8>;

            fn decode(bytes: Vec<u8>) -> Result<Self, Self::Error> {
                let bytes: [u8; 2] = bytes.try_into()?;
                Ok(u16::from_be_bytes(bytes))
            }
        }

        let mut bytes = std::io::Cursor::new(value.to_be_bytes());
        let actual: u16 = bytes.decode_vector(2).unwrap();
        assert_eq!(actual, value)
    }

    #[quickcheck]
    fn equivalent_when_decode_vector_with_ctx(value: u16) {
        impl DecodeVectorWithContext<()> for u16 {
            type Error = Vec<u8>;

            fn decode(bytes: Vec<u8>, _: ()) -> Result<Self, Self::Error> {
                let bytes: [u8; 2] = bytes.try_into()?;
                Ok(u16::from_be_bytes(bytes))
            }
        }

        let mut bytes = std::io::Cursor::new(value.to_be_bytes());
        let actual: u16 = bytes.decode_vector_with(2, ()).unwrap();
        assert_eq!(actual, value)
    }
}
