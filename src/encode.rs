use super::CodecError;

pub trait TryEncode<'e, 'w, W>
where
    W: std::io::Write,
{
    type Error;

    fn handle(&'e self, writer: &'w mut W) -> Result<(), Self::Error>;
}

pub trait TryEncodeWithContext<'e, 'w, W, C>
where
    W: std::io::Write,
{
    type Error;

    fn handle(&'e self, writer: &'w mut W, ctx: C) -> Result<(), Self::Error>;
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

pub trait EncodeFieldWithContext<'e, C> {
    type Bytes: AsRef<[u8]>;

    fn handle(&'e self, ctx: C) -> Self::Bytes;
}

pub trait TryEncodeFieldWithContext<'e, C> {
    type Bytes: AsRef<[u8]>;
    type Error;

    fn handle(&'e self, ctx: C) -> Result<Self::Bytes, Self::Error>;
}

pub trait BinaryEncode<'w, 'e, W> {
    fn try_encode<E>(&'w mut self, value: &'e E) -> Result<(), E::Error>
    where
        E: TryEncode<'e, 'w, W>,
        W: std::io::Write;

    fn try_encode_with<E, C>(&'w mut self, value: &'e E, ctx: C) -> Result<(), E::Error>
    where
        E: TryEncodeWithContext<'e, 'w, W, C>,
        W: std::io::Write;

    fn encode_field<E, B>(&'w mut self, value: &'e E) -> Result<(), std::io::Error>
    where
        E: EncodeField<'e, Bytes = B>,
        B: AsRef<[u8]>;

    fn try_encode_field<E, B>(&'w mut self, value: &'e E) -> Result<(), CodecError<E::Error>>
    where
        E: TryEncodeField<'e, Bytes = B>,
        B: AsRef<[u8]>;

    fn encode_field_with<E, C, B>(&'w mut self, value: &'e E, ctx: C) -> Result<(), std::io::Error>
    where
        E: EncodeFieldWithContext<'e, C, Bytes = B>,
        B: AsRef<[u8]>;

    fn try_encode_field_with<E, C, B>(
        &'w mut self,
        value: &'e E,
        ctx: C,
    ) -> Result<(), CodecError<E::Error>>
    where
        E: TryEncodeFieldWithContext<'e, C, Bytes = B>,
        B: AsRef<[u8]>;
}

impl<'w, 'e, W> BinaryEncode<'w, 'e, W> for W
where
    W: std::io::Write,
{
    fn try_encode<E>(&'w mut self, value: &'e E) -> Result<(), E::Error>
    where
        E: TryEncode<'e, 'w, W>,
        W: std::io::Write,
    {
        E::handle(value, self)
    }

    fn try_encode_with<E, C>(&'w mut self, value: &'e E, ctx: C) -> Result<(), E::Error>
    where
        E: TryEncodeWithContext<'e, 'w, W, C>,
        W: std::io::Write,
    {
        E::handle(value, self, ctx)
    }

    fn encode_field<E, B>(&'w mut self, value: &'e E) -> Result<(), std::io::Error>
    where
        E: EncodeField<'e, Bytes = B>,
        B: AsRef<[u8]>,
    {
        let bytes = value.handle();
        self.write_all(bytes.as_ref())
    }

    fn try_encode_field<E, B>(&'w mut self, value: &'e E) -> Result<(), CodecError<E::Error>>
    where
        E: TryEncodeField<'e, Bytes = B>,
        B: AsRef<[u8]>,
    {
        let bytes = value.handle().map_err(CodecError::UserDefined)?;
        self.write_all(bytes.as_ref()).map_err(CodecError::Io)
    }

    fn encode_field_with<E, C, B>(&'w mut self, value: &'e E, ctx: C) -> Result<(), std::io::Error>
    where
        E: EncodeFieldWithContext<'e, C, Bytes = B>,
        B: AsRef<[u8]>,
    {
        let bytes = value.handle(ctx);
        self.write_all(bytes.as_ref())
    }

    fn try_encode_field_with<E, C, B>(
        &'w mut self,
        value: &'e E,
        ctx: C,
    ) -> Result<(), CodecError<E::Error>>
    where
        E: TryEncodeFieldWithContext<'e, C, Bytes = B>,
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
        impl<W> TryEncodeWithContext<'_, '_, W, ()> for u16
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
        impl EncodeFieldWithContext<'_, ()> for u16 {
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
        impl TryEncodeFieldWithContext<'_, ()> for u16 {
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
