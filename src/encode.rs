#[cfg(feature = "core2")]
use core2::io::Write;

#[cfg(feature = "std")]
use std::io::Write;

pub trait TryEncode<Writer> {
    type Error;

    fn handle(&self, writer: &mut Writer) -> Result<(), Self::Error>;
}

pub trait TryEncodeWith<Writer, Context> {
    type Error;

    fn handle(&self, writer: &mut Writer, ctx: Context) -> Result<(), Self::Error>;
}

pub trait BinaryEncode<Writer> {
    fn try_encode<Encode>(&mut self, value: &Encode) -> Result<(), Encode::Error>
    where
        Encode: TryEncode<Writer>;

    fn try_encode_with<Encode, Context>(
        &mut self,
        value: &Encode,
        ctx: Context,
    ) -> Result<(), Encode::Error>
    where
        Encode: TryEncodeWith<Writer, Context>;
}

impl<Writer> BinaryEncode<Writer> for Writer
where
    Writer: Write,
{
    fn try_encode<Encode>(&mut self, value: &Encode) -> Result<(), Encode::Error>
    where
        Encode: TryEncode<Writer>,
    {
        Encode::handle(value, self)
    }

    fn try_encode_with<Encode, Context>(
        &mut self,
        value: &Encode,
        ctx: Context,
    ) -> Result<(), Encode::Error>
    where
        Encode: TryEncodeWith<Writer, Context>,
    {
        Encode::handle(value, self, ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[cfg(feature = "core2")]
    use core2::io::Error;

    #[cfg(feature = "std")]
    use std::io::Error;

    #[cfg(feature = "core2")]
    use alloc::vec;

    #[cfg(feature = "std")]
    use std::vec;

    #[quickcheck]
    fn equivalent_when_try_encode(value: u16) {
        impl<W> TryEncode<W> for u16
        where
            W: Write,
        {
            type Error = Error;

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
        impl<W> TryEncodeWith<W, ()> for u16
        where
            W: Write,
        {
            type Error = Error;

            fn handle(&self, buf: &mut W, _: ()) -> Result<(), Self::Error> {
                buf.write_all(&self.to_be_bytes())
            }
        }

        let mut buff = vec![];
        buff.try_encode(&value).unwrap();
        let expected = value.to_be_bytes();
        assert_eq!(buff, expected);
    }
}
