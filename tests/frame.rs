extern crate binary_codec;

use binary_codec::*;

#[derive(Debug)]
struct LengthHeader(u32);

#[derive(Debug)]
struct Payload(Vec<u8>);

#[derive(Debug)]
struct Frame {
    length_header: LengthHeader,
    payload: Payload,
}

impl DecodeFixedArray<4> for LengthHeader {
    type Error = std::convert::Infallible;

    fn decode(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(LengthHeader(u32::from_be_bytes(bytes)))
    }
}

impl EncodeBytes<'_> for LengthHeader {
    type Bytes = [u8; 4];
    type Error = std::convert::Infallible;

    fn encode(&self) -> Result<[u8; 4], Self::Error> {
        Ok(self.0.to_be_bytes())
    }
}

impl DecodeVector for Payload {
    type Error = std::convert::Infallible;

    fn decode(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self(bytes))
    }
}

impl<'a> EncodeBytes<'a> for Payload {
    type Bytes = &'a [u8];
    type Error = std::convert::Infallible;

    fn encode(&'a self) -> Result<&'a [u8], Self::Error> {
        Ok(&self.0)
    }
}

impl<R> DecodeMutableRead<'_, R> for Frame
where
    R: std::io::Read,
{
    type Error = CodecError<std::convert::Infallible>;

    fn decode(reader: &mut R) -> Result<Self, Self::Error> {
        let length_header: LengthHeader = reader.decode_fixed_array()?;

        let payload = reader.decode_vector(length_header.0 as usize)?;

        Ok(Self {
            length_header,
            payload,
        })
    }
}

impl<W> EncodeMutableWrite<'_, '_, W> for Frame
where
    W: std::io::Write,
{
    type Error = CodecError<std::convert::Infallible>;

    fn encode(&self, buf: &mut W) -> Result<(), Self::Error> {
        buf.encode_bytes(&self.length_header)?;
        buf.encode_bytes(&self.payload)
    }
}

#[test]
fn equivalent_when_decode_and_encode() {
    let bytes = vec![0x00, 0x00, 0x00, 0x04, 0x11, 0x22, 0x33, 0x44];

    let mut read = std::io::Cursor::new(&bytes);

    let frame: Frame = read.decode_mutable_read().unwrap();

    let mut buf: Vec<u8> = Vec::new();

    buf.encode_mutable_write(&frame).unwrap();

    assert_eq!(buf, bytes);
}
