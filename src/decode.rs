use super::CodecError;

pub trait DecodeMutableRead<'a, R>: Sized
where
    R: std::io::Read,
{
    type Error;

    fn handle(reader: &'a mut R) -> Result<Self, Self::Error>;
}

pub trait DecodeMutableReadWithContext<'a, R, C>: Sized
where
    R: std::io::Read,
{
    type Error;

    fn handle(reader: &'a mut R, ctx: C) -> Result<Self, Self::Error>;
}

pub trait DecodeFixedLengthField<const N: usize>: Sized {
    type Error;

    fn handle(bytes: [u8; N]) -> Result<Self, Self::Error>;
}

pub trait DecodeFixedLengthFieldWithContext<C, const N: usize>: Sized {
    type Error;

    fn handle(bytes: [u8; N], ctx: C) -> Result<Self, Self::Error>;
}

pub trait DecodeVariableLengthField: Sized {
    type Error;

    fn handle(bytes: Vec<u8>) -> Result<Self, Self::Error>;
}

pub trait DecodeVariableLengthFieldWithContext<C>: Sized {
    type Error;

    fn handle(bytes: Vec<u8>, ctx: C) -> Result<Self, Self::Error>;
}

pub trait DecodeByteLength<const N: usize> {
    fn handle(bytes: [u8; N]) -> usize;
}

pub trait BinaryDecode<'a, R> {
    fn decode<D>(&'a mut self) -> Result<D, D::Error>
    where
        D: DecodeMutableRead<'a, R>,
        R: std::io::Read;

    fn decode_with<D, C>(&'a mut self, ctx: C) -> Result<D, D::Error>
    where
        D: DecodeMutableReadWithContext<'a, R, C>,
        R: std::io::Read;

    fn decode_fixed_length_field<D, const N: usize>(
        &'a mut self,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeFixedLengthField<N>;

    fn decode_fixed_length_field_with<D, C, const N: usize>(
        &'a mut self,
        ctx: C,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeFixedLengthFieldWithContext<C, N>;

    fn decode_variable_length_field<D, const N: usize>(
        &'a mut self,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeByteLength<N> + DecodeVariableLengthField;

    fn decode_variable_length_field_with<D, C, const N: usize>(
        &'a mut self,
        ctx: C,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeByteLength<N> + DecodeVariableLengthFieldWithContext<C>;

    fn decode_variable_length_field_with_length<D>(
        &'a mut self,
        length: usize,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeVariableLengthField;

    fn decode_variable_length_field_with_length_and<D, C>(
        &'a mut self,
        length: usize,
        ctx: C,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeVariableLengthFieldWithContext<C>;
}

impl<'a, R> BinaryDecode<'a, R> for R
where
    R: std::io::Read,
{
    fn decode<D>(&'a mut self) -> Result<D, D::Error>
    where
        D: DecodeMutableRead<'a, R>,
    {
        D::handle(self)
    }

    fn decode_with<D, C>(&'a mut self, ctx: C) -> Result<D, D::Error>
    where
        D: DecodeMutableReadWithContext<'a, R, C>,
        R: std::io::Read,
    {
        D::handle(self, ctx)
    }

    fn decode_fixed_length_field<D, const N: usize>(&'a mut self) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeFixedLengthField<N>,
    {
        let mut buf = [0; N];
        self.read_exact(&mut buf).map_err(CodecError::Io)?;
        D::handle(buf).map_err(CodecError::UserDefined)
    }

    fn decode_fixed_length_field_with<D, C, const N: usize>(
        &'a mut self,
        ctx: C,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeFixedLengthFieldWithContext<C, N>,
    {
        let mut buf = [0; N];
        self.read_exact(&mut buf).map_err(CodecError::Io)?;
        D::handle(buf, ctx).map_err(CodecError::UserDefined)
    }

    fn decode_variable_length_field<D, const N: usize>(
        &'a mut self,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeByteLength<N> + DecodeVariableLengthField,
    {
        let mut buf = [0; N];
        self.read_exact(&mut buf).map_err(CodecError::Io)?;
        let length: usize = <D as DecodeByteLength<N>>::handle(buf);

        let mut buf = vec![0; length];
        self.read_exact(&mut buf).map_err(CodecError::Io)?;
        <D as DecodeVariableLengthField>::handle(buf).map_err(CodecError::UserDefined)
    }

    fn decode_variable_length_field_with<D, C, const N: usize>(
        &'a mut self,
        ctx: C,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeByteLength<N> + DecodeVariableLengthFieldWithContext<C>,
    {
        let mut buf = [0; N];
        self.read_exact(&mut buf).map_err(CodecError::Io)?;
        let length: usize = <D as DecodeByteLength<N>>::handle(buf);

        let mut buf = vec![0; length];
        self.read_exact(&mut buf).map_err(CodecError::Io)?;
        <D as DecodeVariableLengthFieldWithContext<C>>::handle(buf, ctx)
            .map_err(CodecError::UserDefined)
    }

    fn decode_variable_length_field_with_length<D>(
        &'a mut self,
        length: usize,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeVariableLengthField,
    {
        let mut buf = vec![0; length];
        self.read_exact(&mut buf).map_err(CodecError::Io)?;
        D::handle(buf).map_err(CodecError::UserDefined)
    }

    fn decode_variable_length_field_with_length_and<D, C>(
        &'a mut self,
        length: usize,
        ctx: C,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeVariableLengthFieldWithContext<C>,
    {
        let mut buf = vec![0; length];
        self.read_exact(&mut buf).map_err(CodecError::Io)?;
        D::handle(buf, ctx).map_err(CodecError::UserDefined)
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

            fn handle(bytes: &'a mut R) -> Result<Self, Self::Error> {
                let mut buf = [0; 2];
                bytes.read_exact(&mut buf)?;
                Ok(u16::from_be_bytes(buf))
            }
        }

        let mut bytes = std::io::Cursor::new(value.to_be_bytes());
        let actual: u16 = bytes.decode().unwrap();
        assert_eq!(actual, value)
    }

    #[quickcheck]
    fn equivalent_when_decode_mut_read_with_ctx(value: u16) {
        impl<'a, R> DecodeMutableReadWithContext<'a, R, ()> for u16
        where
            R: std::io::Read,
        {
            type Error = std::io::Error;

            fn handle(bytes: &'a mut R, _: ()) -> Result<Self, Self::Error> {
                let mut buf = [0; 2];
                bytes.read_exact(&mut buf)?;
                Ok(u16::from_be_bytes(buf))
            }
        }

        let mut bytes = std::io::Cursor::new(value.to_be_bytes());
        let actual: u16 = bytes.decode_with(()).unwrap();
        assert_eq!(actual, value)
    }

    #[quickcheck]
    fn equivalent_when_decode_fixed_length_field(value: u16) {
        impl DecodeFixedLengthField<2> for u16 {
            type Error = std::convert::Infallible;

            fn handle(bytes: [u8; 2]) -> Result<Self, Self::Error> {
                Ok(u16::from_be_bytes(bytes))
            }
        }

        let mut bytes = std::io::Cursor::new(value.to_be_bytes());
        let actual: u16 = bytes.decode_fixed_length_field().unwrap();
        assert_eq!(actual, value)
    }

    #[quickcheck]
    fn equivalent_when_decode_fixed_length_field_with_ctx(value: u16) {
        impl DecodeFixedLengthFieldWithContext<(), 2> for u16 {
            type Error = std::convert::Infallible;

            fn handle(bytes: [u8; 2], _: ()) -> Result<Self, Self::Error> {
                Ok(u16::from_be_bytes(bytes))
            }
        }

        let mut bytes = std::io::Cursor::new(value.to_be_bytes());
        let actual: u16 = bytes.decode_fixed_length_field_with(()).unwrap();
        assert_eq!(actual, value)
    }

    #[quickcheck]
    fn equivalent_when_decode_variable_length_field_with_length(value: u16) {
        impl DecodeVariableLengthField for u16 {
            type Error = Vec<u8>;

            fn handle(bytes: Vec<u8>) -> Result<Self, Self::Error> {
                let bytes: [u8; 2] = bytes.try_into()?;
                Ok(u16::from_be_bytes(bytes))
            }
        }

        let mut bytes = std::io::Cursor::new(value.to_be_bytes());
        let actual: u16 = bytes.decode_variable_length_field_with_length(2).unwrap();
        assert_eq!(actual, value)
    }

    #[quickcheck]
    fn equivalent_when_decode_variable_length_field_with_length_ctx(value: u16) {
        impl DecodeVariableLengthFieldWithContext<()> for u16 {
            type Error = Vec<u8>;

            fn handle(bytes: Vec<u8>, _: ()) -> Result<Self, Self::Error> {
                let bytes: [u8; 2] = bytes.try_into()?;
                Ok(u16::from_be_bytes(bytes))
            }
        }

        let mut bytes = std::io::Cursor::new(value.to_be_bytes());
        let actual: u16 = bytes
            .decode_variable_length_field_with_length_and(2, ())
            .unwrap();
        assert_eq!(actual, value)
    }

    #[quickcheck]
    fn equivalent_when_decode_variable_length_field(bytes: Vec<u8>) {
        #[derive(Debug, PartialEq)]
        struct Payload(Vec<u8>);

        impl DecodeByteLength<4> for Payload {
            fn handle(bytes: [u8; 4]) -> usize {
                u32::from_be_bytes(bytes) as usize
            }
        }

        impl DecodeVariableLengthField for Payload {
            type Error = std::convert::Infallible;

            fn handle(bytes: Vec<u8>) -> Result<Self, Self::Error> {
                Ok(Self(bytes))
            }
        }

        let length = bytes.len() as u32;
        let mut read =
            std::io::Cursor::new(vec![length.to_be_bytes().to_vec(), bytes.clone()].concat());
        let actual: Payload = read.decode_variable_length_field().unwrap();
        let expected = Payload(bytes);
        assert_eq!(actual, expected);
    }

    #[quickcheck]
    fn equivalent_when_decode_variable_length_field_with_ctx(bytes: Vec<u8>) {
        #[derive(Debug, PartialEq)]
        struct Payload(Vec<u8>);

        impl DecodeByteLength<4> for Payload {
            fn handle(bytes: [u8; 4]) -> usize {
                u32::from_be_bytes(bytes) as usize
            }
        }

        impl DecodeVariableLengthFieldWithContext<()> for Payload {
            type Error = std::convert::Infallible;

            fn handle(bytes: Vec<u8>, _: ()) -> Result<Self, Self::Error> {
                Ok(Self(bytes))
            }
        }

        let length = bytes.len() as u32;
        let mut read =
            std::io::Cursor::new(vec![length.to_be_bytes().to_vec(), bytes.clone()].concat());
        let actual: Payload = read.decode_variable_length_field_with(()).unwrap();
        let expected = Payload(bytes);
        assert_eq!(actual, expected);
    }
}
