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

impl DecodeFixedLengthField<4> for LengthHeader {
    fn handle(bytes: [u8; 4]) -> Self {
        Self(u32::from_be_bytes(bytes))
    }
}

impl EncodeField<'_> for LengthHeader {
    type Bytes = [u8; 4];

    fn handle(&self) -> [u8; 4] {
        self.0.to_be_bytes()
    }
}

impl DecodeVariableLengthField for Payload {
    fn handle(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}

impl<'a> EncodeField<'a> for Payload {
    type Bytes = &'a [u8];

    fn handle(&'a self) -> &'a [u8] {
        &self.0
    }
}

impl<R> TryDecode<'_, R> for Frame
where
    R: std::io::Read,
{
    type Error = std::io::Error;

    fn handle(reader: &mut R) -> Result<Self, Self::Error> {
        let length_header: LengthHeader = reader.decode_fixed_length_field()?;

        let payload = reader.decode_variable_length_field_with_length(length_header.0 as usize)?;

        Ok(Self {
            length_header,
            payload,
        })
    }
}

impl<W> TryEncode<'_, '_, W> for Frame
where
    W: std::io::Write,
{
    type Error = std::io::Error;

    fn handle(&self, buf: &mut W) -> Result<(), Self::Error> {
        buf.encode_field(&self.length_header)?;
        buf.encode_field(&self.payload)
    }
}

#[test]
fn equivalent_when_decode_and_encode() {
    let bytes = vec![0x00, 0x00, 0x00, 0x04, 0x11, 0x22, 0x33, 0x44];

    let mut read = std::io::Cursor::new(&bytes);

    let frame: Frame = read.try_decode().unwrap();

    let mut buf = Vec::new();

    buf.try_encode(&frame).unwrap();

    assert_eq!(buf, bytes);
}
