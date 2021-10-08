use super::CodecError;

pub trait TryEncode<'e, 'w, Writer> {
    type Error;

    fn handle(&'e self, writer: &'w mut Writer) -> Result<(), Self::Error>;
}

pub trait TryEncodeWith<'e, 'w, Writer, Context> {
    type Error;

    fn handle(&'e self, writer: &'w mut Writer, ctx: Context) -> Result<(), Self::Error>;
}

pub trait EncodeField<'e> {
    type Bytes: AsRef<[u8]>;

    fn handle(&'e self) -> Self::Bytes;
}

pub trait TryEncodeField<'e> {
    type Bytes: AsRef<[u8]>;
    type Error;

    fn handle(&'e self) -> Result<Self::Bytes, Self::Error>;
}

pub trait EncodeFieldWith<'e, Context> {
    type Bytes: AsRef<[u8]>;

    fn handle(&'e self, ctx: Context) -> Self::Bytes;
}

pub trait TryEncodeFieldWith<'e, Context> {
    type Bytes: AsRef<[u8]>;
    type Error;

    fn handle(&'e self, ctx: Context) -> Result<Self::Bytes, Self::Error>;
}

pub trait BinaryEncode<'w, 'e, Writer> {
    fn try_encode<Encode>(&'w mut self, value: &'e Encode) -> Result<(), Encode::Error>
    where
        Encode: TryEncode<'e, 'w, Writer>;

    fn try_encode_with<Encode, Context>(
        &'w mut self,
        value: &'e Encode,
        ctx: Context,
    ) -> Result<(), Encode::Error>
    where
        Encode: TryEncodeWith<'e, 'w, Writer, Context>;

    fn encode_field<Encode, Bytes>(&'w mut self, value: &'e Encode) -> Result<(), std::io::Error>
    where
        Encode: EncodeField<'e, Bytes = Bytes>,
        Bytes: AsRef<[u8]>;

    fn try_encode_field<Encode, Bytes>(
        &'w mut self,
        value: &'e Encode,
    ) -> Result<(), CodecError<Encode::Error>>
    where
        Encode: TryEncodeField<'e, Bytes = Bytes>,
        Bytes: AsRef<[u8]>;

    fn encode_field_with<Encode, Context, Bytes>(
        &'w mut self,
        value: &'e Encode,
        ctx: Context,
    ) -> Result<(), std::io::Error>
    where
        Encode: EncodeFieldWith<'e, Context, Bytes = Bytes>,
        Bytes: AsRef<[u8]>;

    fn try_encode_field_with<Encode, Context, Bytes>(
        &'w mut self,
        value: &'e Encode,
        ctx: Context,
    ) -> Result<(), CodecError<Encode::Error>>
    where
        Encode: TryEncodeFieldWith<'e, Context, Bytes = Bytes>,
        Bytes: AsRef<[u8]>;
}

impl<'w, 'e, Writer> BinaryEncode<'w, 'e, Writer> for Writer
where
    Writer: std::io::Write,
{
    fn try_encode<Encode>(&'w mut self, value: &'e Encode) -> Result<(), Encode::Error>
    where
        Encode: TryEncode<'e, 'w, Writer>,
    {
        Encode::handle(value, self)
    }

    fn try_encode_with<Encode, Context>(
        &'w mut self,
        value: &'e Encode,
        ctx: Context,
    ) -> Result<(), Encode::Error>
    where
        Encode: TryEncodeWith<'e, 'w, Writer, Context>,
    {
        Encode::handle(value, self, ctx)
    }

    fn encode_field<Encode, Bytes>(&'w mut self, value: &'e Encode) -> Result<(), std::io::Error>
    where
        Encode: EncodeField<'e, Bytes = Bytes>,
        Bytes: AsRef<[u8]>,
    {
        let bytes = value.handle();
        self.write_all(bytes.as_ref())
    }

    fn try_encode_field<Encode, Bytes>(
        &'w mut self,
        value: &'e Encode,
    ) -> Result<(), CodecError<Encode::Error>>
    where
        Encode: TryEncodeField<'e, Bytes = Bytes>,
        Bytes: AsRef<[u8]>,
    {
        let bytes = value.handle().map_err(CodecError::UserDefined)?;
        self.write_all(bytes.as_ref()).map_err(CodecError::Io)
    }

    fn encode_field_with<Encode, Context, Bytes>(
        &'w mut self,
        value: &'e Encode,
        ctx: Context,
    ) -> Result<(), std::io::Error>
    where
        Encode: EncodeFieldWith<'e, Context, Bytes = Bytes>,
        Bytes: AsRef<[u8]>,
    {
        let bytes = value.handle(ctx);
        self.write_all(bytes.as_ref())
    }

    fn try_encode_field_with<Encode, Context, Bytes>(
        &'w mut self,
        value: &'e Encode,
        ctx: Context,
    ) -> Result<(), CodecError<Encode::Error>>
    where
        Encode: TryEncodeFieldWith<'e, Context, Bytes = Bytes>,
        Bytes: AsRef<[u8]>,
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
    fn equivalent_when_try_encode(value: u16) {
        impl<W> TryEncode<'_, '_, W> for u16
        where
            W: std::io::Write,
        {
            type Error = std::io::Error;

            fn handle(&self, buf: &mut W) -> Result<(), Self::Error> {
                buf.write_all(&self.to_be_bytes())
            }
        }

        let mut buff = vec![];
        buff.try_encode(&value).unwrap();
        let expected = value.to_be_bytes();
        assert_eq!(buff, expected);
    }

    #[quickcheck]
    fn equivalent_when_try_encode_with_ctx(value: u16) {
        impl<W> TryEncodeWith<'_, '_, W, ()> for u16
        where
            W: std::io::Write,
        {
            type Error = std::io::Error;

            fn handle(&self, buf: &mut W, _: ()) -> Result<(), Self::Error> {
                buf.write_all(&self.to_be_bytes())
            }
        }

        let mut buff = vec![];
        buff.try_encode(&value).unwrap();
        let expected = value.to_be_bytes();
        assert_eq!(buff, expected);
    }

    #[quickcheck]
    fn equivalent_when_encode_field(value: u16) {
        impl EncodeField<'_> for u16 {
            type Bytes = [u8; 2];

            fn handle(&self) -> Self::Bytes {
                self.to_be_bytes()
            }
        }

        let mut buff = vec![];
        buff.encode_field(&value).unwrap();
        let expected = value.to_be_bytes();
        assert_eq!(buff, expected);
    }

    #[quickcheck]
    fn equivalent_when_try_encode_field(value: u16) {
        impl TryEncodeField<'_> for u16 {
            type Bytes = [u8; 2];
            type Error = std::convert::Infallible;

            fn handle(&self) -> Result<[u8; 2], Self::Error> {
                Ok(self.to_be_bytes())
            }
        }

        let mut buff = vec![];
        buff.try_encode_field(&value).unwrap();
        let expected = value.to_be_bytes();
        assert_eq!(buff, expected);
    }

    #[quickcheck]
    fn equivalent_when_encode_field_with_ctx(value: u16) {
        impl EncodeFieldWith<'_, ()> for u16 {
            type Bytes = [u8; 2];

            fn handle(&self, _: ()) -> Self::Bytes {
                self.to_be_bytes()
            }
        }

        let mut buff = vec![];
        buff.encode_field_with(&value, ()).unwrap();
        let expected = value.to_be_bytes();
        assert_eq!(buff, expected);
    }

    #[quickcheck]
    fn equivalent_when_try_encode_field_with_ctx(value: u16) {
        impl TryEncodeFieldWith<'_, ()> for u16 {
            type Bytes = [u8; 2];
            type Error = std::convert::Infallible;

            fn handle(&self, _: ()) -> Result<[u8; 2], Self::Error> {
                Ok(self.to_be_bytes())
            }
        }

        let mut buff = vec![];
        buff.try_encode_field_with(&value, ()).unwrap();
        let expected = value.to_be_bytes();
        assert_eq!(buff, expected);
    }
}
