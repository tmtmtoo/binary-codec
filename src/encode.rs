use super::CodecError;

pub trait EncodeMutableWrite<'e, 'w, W>
where
    W: std::io::Write,
{
    type Error;

    fn handle(&'e self, writer: &'w mut W) -> Result<(), Self::Error>;
}

pub trait EncodeMutableWriteWithContext<'e, 'w, W, C>
where
    W: std::io::Write,
{
    type Error;

    fn handle(&'e self, writer: &'w mut W, ctx: C) -> Result<(), Self::Error>;
}

pub trait EncodeField<'e> {
    type Bytes: AsRef<[u8]>;
    type Error;

    fn handle(&'e self) -> Result<Self::Bytes, Self::Error>;
}

pub trait EncodeFieldWithContext<'e, C> {
    type Bytes: AsRef<[u8]>;
    type Error;

    fn handle(&'e self, ctx: C) -> Result<Self::Bytes, Self::Error>;
}

pub trait BinaryEncode<'w, 'e, W> {
    fn encode<E>(&'w mut self, value: &'e E) -> Result<(), E::Error>
    where
        E: EncodeMutableWrite<'e, 'w, W>,
        W: std::io::Write;

    fn encode_with<E, C>(&'w mut self, value: &'e E, ctx: C) -> Result<(), E::Error>
    where
        E: EncodeMutableWriteWithContext<'e, 'w, W, C>,
        W: std::io::Write;

    fn encode_field<E, B>(&'w mut self, value: &'e E) -> Result<(), CodecError<E::Error>>
    where
        E: EncodeField<'e, Bytes = B>,
        B: AsRef<[u8]>;

    fn encode_field_with<E, C, B>(
        &'w mut self,
        value: &'e E,
        ctx: C,
    ) -> Result<(), CodecError<E::Error>>
    where
        E: EncodeFieldWithContext<'e, C, Bytes = B>,
        B: AsRef<[u8]>;
}

impl<'w, 'e, W> BinaryEncode<'w, 'e, W> for W
where
    W: std::io::Write,
{
    fn encode<E>(&'w mut self, value: &'e E) -> Result<(), E::Error>
    where
        E: EncodeMutableWrite<'e, 'w, W>,
        W: std::io::Write,
    {
        E::handle(value, self)
    }

    fn encode_with<E, C>(&'w mut self, value: &'e E, ctx: C) -> Result<(), E::Error>
    where
        E: EncodeMutableWriteWithContext<'e, 'w, W, C>,
        W: std::io::Write,
    {
        E::handle(value, self, ctx)
    }

    fn encode_field<E, B>(&'w mut self, value: &'e E) -> Result<(), CodecError<E::Error>>
    where
        E: EncodeField<'e, Bytes = B>,
        B: AsRef<[u8]>,
    {
        let bytes = value.handle().map_err(CodecError::UserDefined)?;
        self.write_all(bytes.as_ref()).map_err(CodecError::Io)
    }

    fn encode_field_with<E, C, B>(
        &'w mut self,
        value: &'e E,
        ctx: C,
    ) -> Result<(), CodecError<E::Error>>
    where
        E: EncodeFieldWithContext<'e, C, Bytes = B>,
        B: AsRef<[u8]>,
    {
        let bytes = value.handle(ctx).map_err(CodecError::UserDefined)?;
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

            fn handle(&self, buf: &mut W) -> Result<(), Self::Error> {
                buf.write_all(&self.to_be_bytes())
            }
        }

        let mut buff = vec![];
        buff.encode(&value).unwrap();
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

            fn handle(&self, buf: &mut W, _: ()) -> Result<(), Self::Error> {
                buf.write_all(&self.to_be_bytes())
            }
        }

        let mut buff = vec![];
        buff.encode(&value).unwrap();
        let expected = value.to_be_bytes();
        assert_eq!(buff, expected);
    }

    #[quickcheck]
    fn equivalent_when_encode_field(value: u16) {
        impl EncodeField<'_> for u16 {
            type Bytes = [u8; 2];
            type Error = std::convert::Infallible;

            fn handle(&self) -> Result<[u8; 2], Self::Error> {
                Ok(self.to_be_bytes())
            }
        }

        let mut buff = vec![];
        buff.encode_field(&value).unwrap();
        let expected = value.to_be_bytes();
        assert_eq!(buff, expected);
    }

    #[quickcheck]
    fn equivalent_when_encode_field_with_ctx(value: u16) {
        impl EncodeFieldWithContext<'_, ()> for u16 {
            type Bytes = [u8; 2];
            type Error = std::convert::Infallible;

            fn handle(&self, _: ()) -> Result<[u8; 2], Self::Error> {
                Ok(self.to_be_bytes())
            }
        }

        let mut buff = vec![];
        buff.encode_field_with(&value, ()).unwrap();
        let expected = value.to_be_bytes();
        assert_eq!(buff, expected);
    }
}
