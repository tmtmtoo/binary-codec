use super::CodecError;

pub trait TryDecode<'r, R>: Sized
where
    R: std::io::Read,
{
    type Error;

    fn handle(reader: &'r mut R) -> Result<Self, Self::Error>;
}

pub trait TryDecodeWithContext<'r, R, C>: Sized
where
    R: std::io::Read,
{
    type Error;

    fn handle(reader: &'r mut R, ctx: C) -> Result<Self, Self::Error>;
}

pub trait DecodeFixedLengthField<const N: usize>: Sized {
    fn handle(bytes: [u8; N]) -> Self;
}

pub trait TryDecodeFixedLengthField<const N: usize>: Sized {
    type Error;

    fn handle(bytes: [u8; N]) -> Result<Self, Self::Error>;
}

pub trait DecodeFixedLengthFieldWithContext<C, const N: usize>: Sized {
    fn handle(bytes: [u8; N], ctx: C) -> Self;
}

pub trait TryDecodeFixedLengthFieldWithContext<C, const N: usize>: Sized {
    type Error;

    fn handle(bytes: [u8; N], ctx: C) -> Result<Self, Self::Error>;
}

pub trait DecodeVariableLengthField: Sized {
    fn handle(bytes: Vec<u8>) -> Self;
}

pub trait TryDecodeVariableLengthField: Sized {
    type Error;

    fn handle(bytes: Vec<u8>) -> Result<Self, Self::Error>;
}

pub trait DecodeVariableLengthFieldWithContext<C>: Sized {
    fn handle(bytes: Vec<u8>, ctx: C) -> Self;
}

pub trait TryDecodeVariableLengthFieldWithContext<C>: Sized {
    type Error;

    fn handle(bytes: Vec<u8>, ctx: C) -> Result<Self, Self::Error>;
}

pub trait DecodeByteLength<const N: usize> {
    fn handle(bytes: [u8; N]) -> usize;
}

pub trait BinaryDecode<'r, R> {
    fn try_decode<D>(&'r mut self) -> Result<D, D::Error>
    where
        D: TryDecode<'r, R>,
        R: std::io::Read;

    fn try_decode_with<D, C>(&'r mut self, ctx: C) -> Result<D, D::Error>
    where
        D: TryDecodeWithContext<'r, R, C>,
        R: std::io::Read;

    fn decode_fixed_length_field<D, const N: usize>(&'r mut self) -> Result<D, std::io::Error>
    where
        D: DecodeFixedLengthField<N>;

    fn try_decode_fixed_length_field<D, const N: usize>(
        &'r mut self,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: TryDecodeFixedLengthField<N>;

    fn decode_fixed_length_field_with<D, C, const N: usize>(
        &'r mut self,
        ctx: C,
    ) -> Result<D, std::io::Error>
    where
        D: DecodeFixedLengthFieldWithContext<C, N>;

    fn try_decode_fixed_length_field_with<D, C, const N: usize>(
        &'r mut self,
        ctx: C,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: TryDecodeFixedLengthFieldWithContext<C, N>;

    fn decode_variable_length_field<D, const N: usize>(&'r mut self) -> Result<D, std::io::Error>
    where
        D: DecodeByteLength<N> + DecodeVariableLengthField;

    fn try_decode_variable_length_field<D, const N: usize>(
        &'r mut self,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeByteLength<N> + TryDecodeVariableLengthField;

    fn decode_variable_length_field_with<D, C, const N: usize>(
        &'r mut self,
        ctx: C,
    ) -> Result<D, std::io::Error>
    where
        D: DecodeByteLength<N> + DecodeVariableLengthFieldWithContext<C>;

    fn try_decode_variable_length_field_with<D, C, const N: usize>(
        &'r mut self,
        ctx: C,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeByteLength<N> + TryDecodeVariableLengthFieldWithContext<C>;

    fn decode_variable_length_field_with_length<D>(
        &'r mut self,
        length: usize,
    ) -> Result<D, std::io::Error>
    where
        D: DecodeVariableLengthField;

    fn try_decode_variable_length_field_with_length<D>(
        &'r mut self,
        length: usize,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: TryDecodeVariableLengthField;

    fn decode_variable_length_field_with_length_and<D, C>(
        &'r mut self,
        length: usize,
        ctx: C,
    ) -> Result<D, std::io::Error>
    where
        D: DecodeVariableLengthFieldWithContext<C>;

    fn try_decode_variable_length_field_with_length_and<D, C>(
        &'r mut self,
        length: usize,
        ctx: C,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: TryDecodeVariableLengthFieldWithContext<C>;
}

impl<'r, R> BinaryDecode<'r, R> for R
where
    R: std::io::Read,
{
    fn try_decode<D>(&'r mut self) -> Result<D, D::Error>
    where
        D: TryDecode<'r, R>,
    {
        D::handle(self)
    }

    fn try_decode_with<D, C>(&'r mut self, ctx: C) -> Result<D, D::Error>
    where
        D: TryDecodeWithContext<'r, R, C>,
        R: std::io::Read,
    {
        D::handle(self, ctx)
    }

    fn decode_fixed_length_field<D, const N: usize>(&'r mut self) -> Result<D, std::io::Error>
    where
        D: DecodeFixedLengthField<N>,
    {
        let mut buf = [0; N];
        self.read_exact(&mut buf)?;

        let field = D::handle(buf);
        Ok(field)
    }

    fn try_decode_fixed_length_field<D, const N: usize>(
        &'r mut self,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: TryDecodeFixedLengthField<N>,
    {
        let mut buf = [0; N];
        self.read_exact(&mut buf).map_err(CodecError::Io)?;

        D::handle(buf).map_err(CodecError::UserDefined)
    }

    fn decode_fixed_length_field_with<D, C, const N: usize>(
        &'r mut self,
        ctx: C,
    ) -> Result<D, std::io::Error>
    where
        D: DecodeFixedLengthFieldWithContext<C, N>,
    {
        let mut buf = [0; N];
        self.read_exact(&mut buf)?;

        let field = D::handle(buf, ctx);
        Ok(field)
    }

    fn try_decode_fixed_length_field_with<D, C, const N: usize>(
        &'r mut self,
        ctx: C,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: TryDecodeFixedLengthFieldWithContext<C, N>,
    {
        let mut buf = [0; N];
        self.read_exact(&mut buf).map_err(CodecError::Io)?;

        D::handle(buf, ctx).map_err(CodecError::UserDefined)
    }

    fn decode_variable_length_field<D, const N: usize>(&'r mut self) -> Result<D, std::io::Error>
    where
        D: DecodeByteLength<N> + DecodeVariableLengthField,
    {
        let mut buf = [0; N];
        self.read_exact(&mut buf)?;
        let length: usize = <D as DecodeByteLength<N>>::handle(buf);

        let mut buf = vec![0; length];
        self.read_exact(&mut buf)?;

        let field = <D as DecodeVariableLengthField>::handle(buf);
        Ok(field)
    }

    fn try_decode_variable_length_field<D, const N: usize>(
        &'r mut self,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeByteLength<N> + TryDecodeVariableLengthField,
    {
        let mut buf = [0; N];
        self.read_exact(&mut buf).map_err(CodecError::Io)?;
        let length: usize = <D as DecodeByteLength<N>>::handle(buf);

        let mut buf = vec![0; length];
        self.read_exact(&mut buf).map_err(CodecError::Io)?;

        <D as TryDecodeVariableLengthField>::handle(buf).map_err(CodecError::UserDefined)
    }

    fn decode_variable_length_field_with<D, C, const N: usize>(
        &'r mut self,
        ctx: C,
    ) -> Result<D, std::io::Error>
    where
        D: DecodeByteLength<N> + DecodeVariableLengthFieldWithContext<C>,
    {
        let mut buf = [0; N];
        self.read_exact(&mut buf)?;
        let length: usize = <D as DecodeByteLength<N>>::handle(buf);

        let mut buf = vec![0; length];
        self.read_exact(&mut buf)?;

        let field = <D as DecodeVariableLengthFieldWithContext<C>>::handle(buf, ctx);
        Ok(field)
    }

    fn try_decode_variable_length_field_with<D, C, const N: usize>(
        &'r mut self,
        ctx: C,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: DecodeByteLength<N> + TryDecodeVariableLengthFieldWithContext<C>,
    {
        let mut buf = [0; N];
        self.read_exact(&mut buf).map_err(CodecError::Io)?;
        let length: usize = <D as DecodeByteLength<N>>::handle(buf);

        let mut buf = vec![0; length];
        self.read_exact(&mut buf).map_err(CodecError::Io)?;

        <D as TryDecodeVariableLengthFieldWithContext<C>>::handle(buf, ctx)
            .map_err(CodecError::UserDefined)
    }

    fn decode_variable_length_field_with_length<D>(
        &'r mut self,
        length: usize,
    ) -> Result<D, std::io::Error>
    where
        D: DecodeVariableLengthField,
    {
        let mut buf = vec![0; length];
        self.read_exact(&mut buf)?;

        let field = D::handle(buf);
        Ok(field)
    }

    fn try_decode_variable_length_field_with_length<D>(
        &'r mut self,
        length: usize,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: TryDecodeVariableLengthField,
    {
        let mut buf = vec![0; length];
        self.read_exact(&mut buf).map_err(CodecError::Io)?;

        D::handle(buf).map_err(CodecError::UserDefined)
    }

    fn decode_variable_length_field_with_length_and<D, C>(
        &'r mut self,
        length: usize,
        ctx: C,
    ) -> Result<D, std::io::Error>
    where
        D: DecodeVariableLengthFieldWithContext<C>,
    {
        let mut buf = vec![0; length];
        self.read_exact(&mut buf)?;

        let field = D::handle(buf, ctx);
        Ok(field)
    }

    fn try_decode_variable_length_field_with_length_and<D, C>(
        &'r mut self,
        length: usize,
        ctx: C,
    ) -> Result<D, CodecError<D::Error>>
    where
        D: TryDecodeVariableLengthFieldWithContext<C>,
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
    fn equivalent_when_try_decode(value: u16) {
        impl<'r, R> TryDecode<'r, R> for u16
        where
            R: std::io::Read,
        {
            type Error = std::io::Error;

            fn handle(bytes: &'r mut R) -> Result<Self, Self::Error> {
                let mut buf = [0; 2];
                bytes.read_exact(&mut buf)?;
                Ok(u16::from_be_bytes(buf))
            }
        }

        let mut bytes = std::io::Cursor::new(value.to_be_bytes());
        let actual: u16 = bytes.try_decode().unwrap();
        assert_eq!(actual, value)
    }

    #[quickcheck]
    fn equivalent_when_try_decode_with_ctx(value: u16) {
        impl<'r, R> TryDecodeWithContext<'r, R, ()> for u16
        where
            R: std::io::Read,
        {
            type Error = std::io::Error;

            fn handle(bytes: &'r mut R, _: ()) -> Result<Self, Self::Error> {
                let mut buf = [0; 2];
                bytes.read_exact(&mut buf)?;
                Ok(u16::from_be_bytes(buf))
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
        impl DecodeFixedLengthFieldWithContext<(), 2> for u16 {
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
        impl TryDecodeFixedLengthFieldWithContext<(), 2> for u16 {
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
    fn equivalent_when_decode_variable_length_field_with_length_ctx(value: u16) {
        impl DecodeVariableLengthFieldWithContext<()> for u16 {
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
    fn equivalent_when_try_decode_variable_length_field_with_length_ctx(value: u16) {
        impl TryDecodeVariableLengthFieldWithContext<()> for u16 {
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
    fn equivalent_when_decode_variable_length_field_with_ctx(bytes: Vec<u8>) {
        #[derive(Debug, PartialEq)]
        struct Payload(Vec<u8>);

        impl DecodeByteLength<4> for Payload {
            fn handle(bytes: [u8; 4]) -> usize {
                u32::from_be_bytes(bytes) as usize
            }
        }

        impl DecodeVariableLengthFieldWithContext<()> for Payload {
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
    fn equivalent_when_try_decode_variable_length_field_with_ctx(bytes: Vec<u8>) {
        #[derive(Debug, PartialEq)]
        struct Payload(Vec<u8>);

        impl DecodeByteLength<4> for Payload {
            fn handle(bytes: [u8; 4]) -> usize {
                u32::from_be_bytes(bytes) as usize
            }
        }

        impl TryDecodeVariableLengthFieldWithContext<()> for Payload {
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
