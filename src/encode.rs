use super::CodecError;

pub trait EncodeMutableWrite<'a, 'b, W>
where
    W: std::io::Write,
{
    type Error;

    fn encode(&'a self, writer: &'b mut W) -> Result<(), Self::Error>;
}

pub trait EncodeMutableWriteWithContext<'a, 'b, W, C>
where
    W: std::io::Write,
{
    type Error;

    fn encode(&'a self, writer: &'b mut W, ctx: C) -> Result<(), Self::Error>;
}

pub trait EncodeBytes<'a> {
    type Bytes: AsRef<[u8]>;
    type Error;

    fn encode(&'a self) -> Result<Self::Bytes, Self::Error>;
}

pub trait EncodeBytesWithContext<'a, C> {
    type Bytes: AsRef<[u8]>;
    type Error;

    fn encode(&'a self, ctx: C) -> Result<Self::Bytes, Self::Error>;
}

pub trait BinaryEncode<'a, 'b, W> {
    fn encode_mutable_write<E>(&'a mut self, value: &'b E) -> Result<(), E::Error>
    where
        E: EncodeMutableWrite<'b, 'a, W>,
        W: std::io::Write;

    fn encode_mutable_write_with<E, C>(&'a mut self, value: &'b E, ctx: C) -> Result<(), E::Error>
    where
        E: EncodeMutableWriteWithContext<'b, 'a, W, C>,
        W: std::io::Write;

    fn encode_bytes<E, B>(&'a mut self, value: &'b E) -> Result<(), CodecError<E::Error>>
    where
        E: EncodeBytes<'b, Bytes = B>,
        B: AsRef<[u8]>;

    fn encode_bytes_with<E, C, B>(
        &'a mut self,
        value: &'b E,
        ctx: C,
    ) -> Result<(), CodecError<E::Error>>
    where
        E: EncodeBytesWithContext<'b, C, Bytes = B>,
        B: AsRef<[u8]>;
}

impl<'a, 'b, W> BinaryEncode<'a, 'b, W> for W
where
    W: std::io::Write,
{
    fn encode_mutable_write<E>(&'a mut self, value: &'b E) -> Result<(), E::Error>
    where
        E: EncodeMutableWrite<'b, 'a, W>,
        W: std::io::Write,
    {
        E::encode(value, self)
    }

    fn encode_mutable_write_with<E, C>(&'a mut self, value: &'b E, ctx: C) -> Result<(), E::Error>
    where
        E: EncodeMutableWriteWithContext<'b, 'a, W, C>,
        W: std::io::Write,
    {
        E::encode(value, self, ctx)
    }

    fn encode_bytes<E, B>(&'a mut self, value: &'b E) -> Result<(), CodecError<E::Error>>
    where
        E: EncodeBytes<'b, Bytes = B>,
        B: AsRef<[u8]>,
    {
        let bytes = value.encode().map_err(CodecError::UserDefined)?;
        self.write_all(bytes.as_ref()).map_err(CodecError::Io)
    }

    fn encode_bytes_with<E, C, B>(
        &'a mut self,
        value: &'b E,
        ctx: C,
    ) -> Result<(), CodecError<E::Error>>
    where
        E: EncodeBytesWithContext<'b, C, Bytes = B>,
        B: AsRef<[u8]>,
    {
        let bytes = value.encode(ctx).map_err(CodecError::UserDefined)?;
        self.write_all(bytes.as_ref()).map_err(CodecError::Io)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn equivalent_when_encode_mut_write(value: u16) {
        impl<W> EncodeMutableWrite<'_, '_, W> for u16
        where
            W: std::io::Write,
        {
            type Error = std::io::Error;

            fn encode(&self, buf: &mut W) -> Result<(), Self::Error> {
                buf.write_all(&self.to_be_bytes())
            }
        }

        let mut buff = vec![];
        buff.encode_mutable_write(&value).unwrap();
        let expected = value.to_be_bytes();
        assert_eq!(buff, expected);
    }

    #[quickcheck]
    fn equivalent_when_encode_mut_write_with_ctx(value: u16) {
        impl<W> EncodeMutableWriteWithContext<'_, '_, W, ()> for u16
        where
            W: std::io::Write,
        {
            type Error = std::io::Error;

            fn encode(&self, buf: &mut W, _: ()) -> Result<(), Self::Error> {
                buf.write_all(&self.to_be_bytes())
            }
        }

        let mut buff = vec![];
        buff.encode_mutable_write(&value).unwrap();
        let expected = value.to_be_bytes();
        assert_eq!(buff, expected);
    }

    #[quickcheck]
    fn equivalent_when_encode(value: u16) {
        impl EncodeBytes<'_> for u16 {
            type Bytes = [u8; 2];
            type Error = std::convert::Infallible;

            fn encode(&self) -> Result<[u8; 2], Self::Error> {
                Ok(self.to_be_bytes())
            }
        }

        let mut buff = vec![];
        buff.encode_bytes(&value).unwrap();
        let expected = value.to_be_bytes();
        assert_eq!(buff, expected);
    }

    #[quickcheck]
    fn equivalent_when_encode_with_ctx(value: u16) {
        impl EncodeBytesWithContext<'_, ()> for u16 {
            type Bytes = [u8; 2];
            type Error = std::convert::Infallible;

            fn encode(&self, _: ()) -> Result<[u8; 2], Self::Error> {
                Ok(self.to_be_bytes())
            }
        }

        let mut buff = vec![];
        buff.encode_bytes_with(&value, ()).unwrap();
        let expected = value.to_be_bytes();
        assert_eq!(buff, expected);
    }
}
