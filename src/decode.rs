use super::CodecError;

pub trait TryDecode<'r, Reader>: Sized {
    type Error;

    fn handle(reader: &'r mut Reader) -> Result<Self, Self::Error>;
}

pub trait TryDecodeWith<'r, Reader, Context>: Sized {
    type Error;

    fn handle(reader: &'r mut Reader, ctx: Context) -> Result<Self, Self::Error>;
}

pub trait DecodeFixedLengthField<const N: usize>: Sized {
    fn handle(bytes: [u8; N]) -> Self;
}

pub trait TryDecodeFixedLengthField<const N: usize>: Sized {
    type Error;

    fn handle(bytes: [u8; N]) -> Result<Self, Self::Error>;
}

pub trait DecodeFixedLengthFieldWith<Context, const N: usize>: Sized {
    fn handle(bytes: [u8; N], ctx: Context) -> Self;
}

pub trait TryDecodeFixedLengthFieldWith<Context, const N: usize>: Sized {
    type Error;

    fn handle(bytes: [u8; N], ctx: Context) -> Result<Self, Self::Error>;
}

pub trait DecodeVariableLengthField: Sized {
    fn handle(bytes: Vec<u8>) -> Self;
}

pub trait TryDecodeVariableLengthField: Sized {
    type Error;

    fn handle(bytes: Vec<u8>) -> Result<Self, Self::Error>;
}

pub trait DecodeVariableLengthFieldWith<Context>: Sized {
    fn handle(bytes: Vec<u8>, ctx: Context) -> Self;
}

pub trait TryDecodeVariableLengthFieldWith<Context>: Sized {
    type Error;

    fn handle(bytes: Vec<u8>, ctx: Context) -> Result<Self, Self::Error>;
}

pub trait DecodeByteLength<const N: usize> {
    fn handle(bytes: [u8; N]) -> usize;
}

pub trait BinaryDecode<'r, Reader> {
    fn try_decode<Decode>(&'r mut self) -> Result<Decode, Decode::Error>
    where
        Decode: TryDecode<'r, Reader>;

    fn try_decode_with<Decode, Context>(
        &'r mut self,
        ctx: Context,
    ) -> Result<Decode, Decode::Error>
    where
        Decode: TryDecodeWith<'r, Reader, Context>;

    fn decode_fixed_length_field<Decode, const N: usize>(
        &'r mut self,
    ) -> Result<Decode, std::io::Error>
    where
        Decode: DecodeFixedLengthField<N>;

    fn try_decode_fixed_length_field<Decode, const N: usize>(
        &'r mut self,
    ) -> Result<Decode, CodecError<Decode::Error>>
    where
        Decode: TryDecodeFixedLengthField<N>;

    fn decode_fixed_length_field_with<Decode, Context, const N: usize>(
        &'r mut self,
        ctx: Context,
    ) -> Result<Decode, std::io::Error>
    where
        Decode: DecodeFixedLengthFieldWith<Context, N>;

    fn try_decode_fixed_length_field_with<Decode, Context, const N: usize>(
        &'r mut self,
        ctx: Context,
    ) -> Result<Decode, CodecError<Decode::Error>>
    where
        Decode: TryDecodeFixedLengthFieldWith<Context, N>;

    fn decode_variable_length_field<Decode, const N: usize>(
        &'r mut self,
    ) -> Result<Decode, std::io::Error>
    where
        Decode: DecodeByteLength<N> + DecodeVariableLengthField;

    fn try_decode_variable_length_field<Decode, const N: usize>(
        &'r mut self,
    ) -> Result<Decode, CodecError<Decode::Error>>
    where
        Decode: DecodeByteLength<N> + TryDecodeVariableLengthField;

    fn decode_variable_length_field_with<Decode, Context, const N: usize>(
        &'r mut self,
        ctx: Context,
    ) -> Result<Decode, std::io::Error>
    where
        Decode: DecodeByteLength<N> + DecodeVariableLengthFieldWith<Context>;

    fn try_decode_variable_length_field_with<Decode, Context, const N: usize>(
        &'r mut self,
        ctx: Context,
    ) -> Result<Decode, CodecError<Decode::Error>>
    where
        Decode: DecodeByteLength<N> + TryDecodeVariableLengthFieldWith<Context>;

    fn decode_variable_length_field_with_length<Decode>(
        &'r mut self,
        length: usize,
    ) -> Result<Decode, std::io::Error>
    where
        Decode: DecodeVariableLengthField;

    fn try_decode_variable_length_field_with_length<Decode>(
        &'r mut self,
        length: usize,
    ) -> Result<Decode, CodecError<Decode::Error>>
    where
        Decode: TryDecodeVariableLengthField;

    fn decode_variable_length_field_with_length_and<Decode, Context>(
        &'r mut self,
        length: usize,
        ctx: Context,
    ) -> Result<Decode, std::io::Error>
    where
        Decode: DecodeVariableLengthFieldWith<Context>;

    fn try_decode_variable_length_field_with_length_and<Decode, Context>(
        &'r mut self,
        length: usize,
        ctx: Context,
    ) -> Result<Decode, CodecError<Decode::Error>>
    where
        Decode: TryDecodeVariableLengthFieldWith<Context>;

    fn read_fixed_length<const N: usize>(&'r mut self) -> Result<[u8; N], std::io::Error>;

    fn read_variable_length(&'r mut self, length: usize) -> Result<Vec<u8>, std::io::Error>;
}

impl<'r, Reader> BinaryDecode<'r, Reader> for Reader
where
    Reader: std::io::Read,
{
    fn try_decode<Decode>(&'r mut self) -> Result<Decode, Decode::Error>
    where
        Decode: TryDecode<'r, Reader>,
    {
        Decode::handle(self)
    }

    fn try_decode_with<Decode, Context>(&'r mut self, ctx: Context) -> Result<Decode, Decode::Error>
    where
        Decode: TryDecodeWith<'r, Reader, Context>,
    {
        Decode::handle(self, ctx)
    }

    fn decode_fixed_length_field<Decode, const N: usize>(
        &'r mut self,
    ) -> Result<Decode, std::io::Error>
    where
        Decode: DecodeFixedLengthField<N>,
    {
        self.read_fixed_length().map(Decode::handle)
    }

    fn try_decode_fixed_length_field<Decode, const N: usize>(
        &'r mut self,
    ) -> Result<Decode, CodecError<Decode::Error>>
    where
        Decode: TryDecodeFixedLengthField<N>,
    {
        let bytes = self.read_fixed_length().map_err(CodecError::Io)?;
        Decode::handle(bytes).map_err(CodecError::UserDefined)
    }

    fn decode_fixed_length_field_with<Decode, Context, const N: usize>(
        &'r mut self,
        ctx: Context,
    ) -> Result<Decode, std::io::Error>
    where
        Decode: DecodeFixedLengthFieldWith<Context, N>,
    {
        let bytes = self.read_fixed_length()?;
        let field = Decode::handle(bytes, ctx);
        Ok(field)
    }

    fn try_decode_fixed_length_field_with<Decode, Context, const N: usize>(
        &'r mut self,
        ctx: Context,
    ) -> Result<Decode, CodecError<Decode::Error>>
    where
        Decode: TryDecodeFixedLengthFieldWith<Context, N>,
    {
        let bytes = self.read_fixed_length().map_err(CodecError::Io)?;
        Decode::handle(bytes, ctx).map_err(CodecError::UserDefined)
    }

    fn decode_variable_length_field<Decode, const N: usize>(
        &'r mut self,
    ) -> Result<Decode, std::io::Error>
    where
        Decode: DecodeByteLength<N> + DecodeVariableLengthField,
    {
        let length = self
            .read_fixed_length()
            .map(<Decode as DecodeByteLength<N>>::handle)?;

        self.read_variable_length(length)
            .map(<Decode as DecodeVariableLengthField>::handle)
    }

    fn try_decode_variable_length_field<Decode, const N: usize>(
        &'r mut self,
    ) -> Result<Decode, CodecError<Decode::Error>>
    where
        Decode: DecodeByteLength<N> + TryDecodeVariableLengthField,
    {
        let length = self
            .read_fixed_length()
            .map_err(CodecError::Io)
            .map(<Decode as DecodeByteLength<N>>::handle)?;

        let bytes = self.read_variable_length(length).map_err(CodecError::Io)?;

        <Decode as TryDecodeVariableLengthField>::handle(bytes).map_err(CodecError::UserDefined)
    }

    fn decode_variable_length_field_with<Decode, Context, const N: usize>(
        &'r mut self,
        ctx: Context,
    ) -> Result<Decode, std::io::Error>
    where
        Decode: DecodeByteLength<N> + DecodeVariableLengthFieldWith<Context>,
    {
        let length = self
            .read_fixed_length()
            .map(<Decode as DecodeByteLength<N>>::handle)?;

        let bytes = self.read_variable_length(length)?;

        let field = <Decode as DecodeVariableLengthFieldWith<Context>>::handle(bytes, ctx);
        Ok(field)
    }

    fn try_decode_variable_length_field_with<Decode, Context, const N: usize>(
        &'r mut self,
        ctx: Context,
    ) -> Result<Decode, CodecError<Decode::Error>>
    where
        Decode: DecodeByteLength<N> + TryDecodeVariableLengthFieldWith<Context>,
    {
        let length = self
            .read_fixed_length()
            .map_err(CodecError::Io)
            .map(<Decode as DecodeByteLength<N>>::handle)?;

        let bytes = self.read_variable_length(length).map_err(CodecError::Io)?;

        <Decode as TryDecodeVariableLengthFieldWith<Context>>::handle(bytes, ctx)
            .map_err(CodecError::UserDefined)
    }

    fn decode_variable_length_field_with_length<Decode>(
        &'r mut self,
        length: usize,
    ) -> Result<Decode, std::io::Error>
    where
        Decode: DecodeVariableLengthField,
    {
        self.read_variable_length(length).map(Decode::handle)
    }

    fn try_decode_variable_length_field_with_length<Decode>(
        &'r mut self,
        length: usize,
    ) -> Result<Decode, CodecError<Decode::Error>>
    where
        Decode: TryDecodeVariableLengthField,
    {
        let bytes = self.read_variable_length(length).map_err(CodecError::Io)?;
        Decode::handle(bytes).map_err(CodecError::UserDefined)
    }

    fn decode_variable_length_field_with_length_and<Decode, Context>(
        &'r mut self,
        length: usize,
        ctx: Context,
    ) -> Result<Decode, std::io::Error>
    where
        Decode: DecodeVariableLengthFieldWith<Context>,
    {
        let bytes = self.read_variable_length(length)?;
        let field = Decode::handle(bytes, ctx);
        Ok(field)
    }

    fn try_decode_variable_length_field_with_length_and<Decode, Context>(
        &'r mut self,
        length: usize,
        ctx: Context,
    ) -> Result<Decode, CodecError<Decode::Error>>
    where
        Decode: TryDecodeVariableLengthFieldWith<Context>,
    {
        let bytes = self.read_variable_length(length).map_err(CodecError::Io)?;
        Decode::handle(bytes, ctx).map_err(CodecError::UserDefined)
    }

    fn read_fixed_length<const N: usize>(&'r mut self) -> Result<[u8; N], std::io::Error> {
        let mut buf = [0; N];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn read_variable_length(&'r mut self, length: usize) -> Result<Vec<u8>, std::io::Error> {
        let mut buf = vec![0; length];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;
    use std::convert::TryInto;

    #[quickcheck]
    fn equivalent_when_try_decode(value: u16) {
        impl<'r, R> TryDecode<'r, R> for u16
        where
            R: std::io::Read,
        {
            type Error = std::io::Error;

            fn handle(bytes: &'r mut R) -> Result<Self, Self::Error> {
                bytes.read_fixed_length().map(u16::from_be_bytes)
            }
        }

        let mut bytes = std::io::Cursor::new(value.to_be_bytes());
        let actual: u16 = bytes.try_decode().unwrap();
        assert_eq!(actual, value)
    }

    #[quickcheck]
    fn equivalent_when_try_decode_with_ctx(value: u16) {
        impl<'r, R> TryDecodeWith<'r, R, ()> for u16
        where
            R: std::io::Read,
        {
            type Error = std::io::Error;

            fn handle(bytes: &'r mut R, _: ()) -> Result<Self, Self::Error> {
                bytes.read_fixed_length().map(u16::from_be_bytes)
            }
        }

        let mut bytes = std::io::Cursor::new(value.to_be_bytes());
        let actual: u16 = bytes.try_decode_with(()).unwrap();
        assert_eq!(actual, value)
    }

    #[quickcheck]
    fn equivalent_when_decode_fixed_length_field(value: u16) {
        impl DecodeFixedLengthField<2> for u16 {
            fn handle(bytes: [u8; 2]) -> Self {
                u16::from_be_bytes(bytes)
            }
        }

        let mut bytes = std::io::Cursor::new(value.to_be_bytes());
        let actual: u16 = bytes.decode_fixed_length_field().unwrap();
        assert_eq!(actual, value)
    }

    #[quickcheck]
    fn equivalent_when_try_decode_fixed_length_field(value: u16) {
        impl TryDecodeFixedLengthField<2> for u16 {
            type Error = std::convert::Infallible;

            fn handle(bytes: [u8; 2]) -> Result<Self, Self::Error> {
                Ok(u16::from_be_bytes(bytes))
            }
        }

        let mut bytes = std::io::Cursor::new(value.to_be_bytes());
        let actual: u16 = bytes.try_decode_fixed_length_field().unwrap();
        assert_eq!(actual, value)
    }

    #[quickcheck]
    fn equivalent_when_decode_fixed_length_field_with_ctx(value: u16) {
        impl DecodeFixedLengthFieldWith<(), 2> for u16 {
            fn handle(bytes: [u8; 2], _: ()) -> Self {
                u16::from_be_bytes(bytes)
            }
        }

        let mut bytes = std::io::Cursor::new(value.to_be_bytes());
        let actual: u16 = bytes.decode_fixed_length_field_with(()).unwrap();
        assert_eq!(actual, value)
    }

    #[quickcheck]
    fn equivalent_when_try_decode_fixed_length_field_with_ctx(value: u16) {
        impl TryDecodeFixedLengthFieldWith<(), 2> for u16 {
            type Error = std::convert::Infallible;

            fn handle(bytes: [u8; 2], _: ()) -> Result<Self, Self::Error> {
                Ok(u16::from_be_bytes(bytes))
            }
        }

        let mut bytes = std::io::Cursor::new(value.to_be_bytes());
        let actual: u16 = bytes.try_decode_fixed_length_field_with(()).unwrap();
        assert_eq!(actual, value)
    }

    #[quickcheck]
    fn equivalent_when_decode_variable_length_field_with_length(value: u16) {
        impl DecodeVariableLengthField for u16 {
            fn handle(bytes: Vec<u8>) -> Self {
                u16::from_be_bytes([bytes[0], bytes[1]])
            }
        }

        let mut bytes = std::io::Cursor::new(value.to_be_bytes());
        let actual: u16 = bytes.decode_variable_length_field_with_length(2).unwrap();
        assert_eq!(actual, value)
    }

    #[quickcheck]
    fn equivalent_when_try_decode_variable_length_field_with_length(value: u16) {
        impl TryDecodeVariableLengthField for u16 {
            type Error = Vec<u8>;

            fn handle(bytes: Vec<u8>) -> Result<Self, Self::Error> {
                let bytes: [u8; 2] = bytes.try_into()?;
                Ok(u16::from_be_bytes(bytes))
            }
        }

        let mut bytes = std::io::Cursor::new(value.to_be_bytes());
        let actual: u16 = bytes
            .try_decode_variable_length_field_with_length(2)
            .unwrap();
        assert_eq!(actual, value)
    }

    #[quickcheck]
    fn equivalent_when_decode_variable_length_field_with_ctx(value: u16) {
        impl DecodeVariableLengthFieldWith<()> for u16 {
            fn handle(bytes: Vec<u8>, _: ()) -> Self {
                u16::from_be_bytes([bytes[0], bytes[1]])
            }
        }

        let mut bytes = std::io::Cursor::new(value.to_be_bytes());
        let actual: u16 = bytes
            .decode_variable_length_field_with_length_and(2, ())
            .unwrap();
        assert_eq!(actual, value)
    }

    #[quickcheck]
    fn equivalent_when_try_decode_variable_length_field_with_ctx(value: u16) {
        impl TryDecodeVariableLengthFieldWith<()> for u16 {
            type Error = Vec<u8>;

            fn handle(bytes: Vec<u8>, _: ()) -> Result<Self, Self::Error> {
                let bytes: [u8; 2] = bytes.try_into()?;
                Ok(u16::from_be_bytes(bytes))
            }
        }

        let mut bytes = std::io::Cursor::new(value.to_be_bytes());
        let actual: u16 = bytes
            .try_decode_variable_length_field_with_length_and(2, ())
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
            fn handle(bytes: Vec<u8>) -> Self {
                Self(bytes)
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
    fn equivalent_when_try_decode_variable_length_field(bytes: Vec<u8>) {
        #[derive(Debug, PartialEq)]
        struct Payload(Vec<u8>);

        impl DecodeByteLength<4> for Payload {
            fn handle(bytes: [u8; 4]) -> usize {
                u32::from_be_bytes(bytes) as usize
            }
        }

        impl TryDecodeVariableLengthField for Payload {
            type Error = std::convert::Infallible;

            fn handle(bytes: Vec<u8>) -> Result<Self, Self::Error> {
                Ok(Self(bytes))
            }
        }

        let length = bytes.len() as u32;
        let mut read =
            std::io::Cursor::new(vec![length.to_be_bytes().to_vec(), bytes.clone()].concat());
        let actual: Payload = read.try_decode_variable_length_field().unwrap();
        let expected = Payload(bytes);
        assert_eq!(actual, expected);
    }

    #[quickcheck]
    fn equivalent_when_decode_variable_length_field_with_ctx2(bytes: Vec<u8>) {
        #[derive(Debug, PartialEq)]
        struct Payload(Vec<u8>);

        impl DecodeByteLength<4> for Payload {
            fn handle(bytes: [u8; 4]) -> usize {
                u32::from_be_bytes(bytes) as usize
            }
        }

        impl DecodeVariableLengthFieldWith<()> for Payload {
            fn handle(bytes: Vec<u8>, _: ()) -> Self {
                Self(bytes)
            }
        }

        let length = bytes.len() as u32;
        let mut read =
            std::io::Cursor::new(vec![length.to_be_bytes().to_vec(), bytes.clone()].concat());
        let actual: Payload = read.decode_variable_length_field_with(()).unwrap();
        let expected = Payload(bytes);
        assert_eq!(actual, expected);
    }

    #[quickcheck]
    fn equivalent_when_try_decode_variable_length_field_with_ctx2(bytes: Vec<u8>) {
        #[derive(Debug, PartialEq)]
        struct Payload(Vec<u8>);

        impl DecodeByteLength<4> for Payload {
            fn handle(bytes: [u8; 4]) -> usize {
                u32::from_be_bytes(bytes) as usize
            }
        }

        impl TryDecodeVariableLengthFieldWith<()> for Payload {
            type Error = std::convert::Infallible;

            fn handle(bytes: Vec<u8>, _: ()) -> Result<Self, Self::Error> {
                Ok(Self(bytes))
            }
        }

        let length = bytes.len() as u32;
        let mut read =
            std::io::Cursor::new(vec![length.to_be_bytes().to_vec(), bytes.clone()].concat());
        let actual: Payload = read.try_decode_variable_length_field_with(()).unwrap();
        let expected = Payload(bytes);
        assert_eq!(actual, expected);
    }
}
