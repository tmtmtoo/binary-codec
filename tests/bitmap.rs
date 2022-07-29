#![no_std]

extern crate binary_codec;

#[cfg(feature = "core2")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use binary_codec::*;

#[cfg(feature = "core2")]
use core2::io::{Cursor, Error, Read, Write};

#[cfg(feature = "core2")]
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::{
    io::{Cursor, Error, Read, Write},
    vec::Vec,
};

struct BitmapFileHeader {
    signature: [u8; 2],
    file_size: u32,
    reserved_1: u16,
    reserved_2: u16,
    offset: u32,
}

enum BitmapFileHeaderError {
    InvalidSignature([u8; 2]),
    Io(Error),
}

impl<R> TryDecode<R> for BitmapFileHeader
where
    R: Read,
{
    type Error = BitmapFileHeaderError;

    fn handle(reader: &mut R) -> Result<Self, Self::Error> {
        Ok(Self {
            signature: reader
                .read_fixed_length()
                .map_err(BitmapFileHeaderError::Io)
                .and_then(|bytes| match bytes {
                    [0x42, 0x4d] => Ok(bytes),
                    n => Err(BitmapFileHeaderError::InvalidSignature(n)),
                })?,
            file_size: reader
                .read_fixed_length()
                .map_err(BitmapFileHeaderError::Io)
                .map(u32::from_le_bytes)?,
            reserved_1: reader
                .read_fixed_length()
                .map_err(BitmapFileHeaderError::Io)
                .map(u16::from_le_bytes)?,
            reserved_2: reader
                .read_fixed_length()
                .map_err(BitmapFileHeaderError::Io)
                .map(u16::from_le_bytes)?,
            offset: reader
                .read_fixed_length()
                .map_err(BitmapFileHeaderError::Io)
                .map(u32::from_le_bytes)?,
        })
    }
}

impl<W> TryEncode<W> for BitmapFileHeader
where
    W: Write,
{
    type Error = Error;

    fn handle(&self, writer: &mut W) -> Result<(), Self::Error> {
        writer.write_all(&self.signature)?;
        writer.write_all(&self.file_size.to_le_bytes())?;
        writer.write_all(&self.reserved_1.to_le_bytes())?;
        writer.write_all(&self.reserved_2.to_le_bytes())?;
        writer.write_all(&self.offset.to_le_bytes())?;
        Ok(())
    }
}

struct BitmapInfoHeader {
    size: u32,
    width: i32,
    height: i32,
    planes: u16,
    bit_count: u16,
    compression: u32,
    image_size: u32,
    x_pixels_per_meter: i32,
    y_pixels_per_meter: i32,
    colors_used: u32,
    colors_important: u32,
}

enum BitmapInfoHeaderError {
    UnsupportedInfoHeaderSize(u32),
    Io(Error),
}

impl<R> TryDecode<R> for BitmapInfoHeader
where
    R: Read,
{
    type Error = BitmapInfoHeaderError;

    fn handle(reader: &mut R) -> Result<Self, Self::Error> {
        Ok(Self {
            size: reader
                .read_fixed_length()
                .map_err(BitmapInfoHeaderError::Io)
                .map(u32::from_le_bytes)
                .and_then(|size| match size {
                    40 => Ok(size),
                    n => Err(BitmapInfoHeaderError::UnsupportedInfoHeaderSize(n)),
                })?,
            width: reader
                .read_fixed_length()
                .map_err(BitmapInfoHeaderError::Io)
                .map(i32::from_le_bytes)?,
            height: reader
                .read_fixed_length()
                .map_err(BitmapInfoHeaderError::Io)
                .map(i32::from_le_bytes)?,
            planes: reader
                .read_fixed_length()
                .map_err(BitmapInfoHeaderError::Io)
                .map(u16::from_le_bytes)?,
            bit_count: reader
                .read_fixed_length()
                .map_err(BitmapInfoHeaderError::Io)
                .map(u16::from_le_bytes)?,
            compression: reader
                .read_fixed_length()
                .map_err(BitmapInfoHeaderError::Io)
                .map(u32::from_le_bytes)?,
            image_size: reader
                .read_fixed_length()
                .map_err(BitmapInfoHeaderError::Io)
                .map(u32::from_le_bytes)?,
            x_pixels_per_meter: reader
                .read_fixed_length()
                .map_err(BitmapInfoHeaderError::Io)
                .map(i32::from_le_bytes)?,
            y_pixels_per_meter: reader
                .read_fixed_length()
                .map_err(BitmapInfoHeaderError::Io)
                .map(i32::from_le_bytes)?,
            colors_used: reader
                .read_fixed_length()
                .map_err(BitmapInfoHeaderError::Io)
                .map(u32::from_le_bytes)?,
            colors_important: reader
                .read_fixed_length()
                .map_err(BitmapInfoHeaderError::Io)
                .map(u32::from_le_bytes)?,
        })
    }
}

impl<W> TryEncode<W> for BitmapInfoHeader
where
    W: Write,
{
    type Error = Error;

    fn handle(&self, writer: &mut W) -> Result<(), Self::Error> {
        writer.write_all(&self.size.to_le_bytes())?;
        writer.write_all(&self.width.to_le_bytes())?;
        writer.write_all(&self.height.to_le_bytes())?;
        writer.write_all(&self.planes.to_le_bytes())?;
        writer.write_all(&self.bit_count.to_le_bytes())?;
        writer.write_all(&self.compression.to_le_bytes())?;
        writer.write_all(&self.image_size.to_le_bytes())?;
        writer.write_all(&self.x_pixels_per_meter.to_le_bytes())?;
        writer.write_all(&self.y_pixels_per_meter.to_le_bytes())?;
        writer.write_all(&self.colors_used.to_le_bytes())?;
        writer.write_all(&self.colors_important.to_le_bytes())?;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
struct RgbTriple {
    r: u8,
    g: u8,
    b: u8,
}

impl<R> TryDecode<R> for RgbTriple
where
    R: Read,
{
    type Error = Error;

    fn handle(reader: &mut R) -> Result<Self, Self::Error> {
        let [b, g, r] = reader.read_fixed_length()?;
        Ok(Self { r, g, b })
    }
}

impl<W> TryEncode<W> for RgbTriple
where
    W: Write,
{
    type Error = Error;

    fn handle(&self, writer: &mut W) -> Result<(), Self::Error> {
        writer.write_all(&[self.b, self.g, self.r])
    }
}

#[derive(Debug, PartialEq)]
struct RgbQuad {
    r: u8,
    g: u8,
    b: u8,
    reserved: u8,
}

impl<R> TryDecode<R> for RgbQuad
where
    R: Read,
{
    type Error = Error;

    fn handle(reader: &mut R) -> Result<Self, Self::Error> {
        let [b, g, r, reserved] = reader.read_fixed_length()?;
        Ok(Self { r, g, b, reserved })
    }
}

impl<W> TryEncode<W> for RgbQuad
where
    W: Write,
{
    type Error = Error;

    fn handle(&self, writer: &mut W) -> Result<(), Self::Error> {
        writer.write_all(&[self.b, self.g, self.r, self.reserved])
    }
}

#[derive(Debug, PartialEq)]
enum ColorTable {
    RgbQuad(Vec<RgbQuad>),
    RgbTriple(Vec<RgbTriple>),
}

#[derive(Debug)]
enum ColorTableError {
    UnsupportedColorPalette(u16),
    InvalidDataLength(u32),
    Io(Error),
}

struct ColorTableContext {
    length: u32,
    bit_count: u16,
}

impl ColorTableContext {
    pub fn from_headers(file_header: &BitmapFileHeader, info_header: &BitmapInfoHeader) -> Self {
        Self {
            length: file_header.file_size - file_header.offset,
            bit_count: info_header.bit_count,
        }
    }
}

impl<R> TryDecodeWith<R, ColorTableContext> for ColorTable
where
    R: Read,
{
    type Error = ColorTableError;

    fn handle(reader: &mut R, ctx: ColorTableContext) -> Result<Self, Self::Error> {
        match ctx.bit_count {
            24 => {
                if ctx.length % 3 != 0 {
                    return Err(ColorTableError::InvalidDataLength(ctx.length));
                }

                let mut list = Vec::new();
                for _ in 0..ctx.length / 3 {
                    list.push(reader.try_decode().map_err(ColorTableError::Io)?)
                }
                Ok(ColorTable::RgbTriple(list))
            }
            32 => {
                if ctx.length % 4 != 0 {
                    return Err(ColorTableError::InvalidDataLength(ctx.length));
                }

                let mut list = Vec::new();
                for _ in 0..ctx.length / 4 {
                    list.push(reader.try_decode().map_err(ColorTableError::Io)?)
                }
                Ok(ColorTable::RgbQuad(list))
            }
            n => Err(ColorTableError::UnsupportedColorPalette(n)),
        }
    }
}

impl<W> TryEncode<W> for ColorTable
where
    W: Write,
{
    type Error = Error;

    fn handle(&self, writer: &mut W) -> Result<(), Self::Error> {
        match self {
            ColorTable::RgbTriple(list) => {
                for rgb_triple in list.iter() {
                    writer.try_encode(rgb_triple)?;
                }
            }
            ColorTable::RgbQuad(list) => {
                for rgb_quad in list.iter() {
                    writer.try_encode(rgb_quad)?;
                }
            }
        }
        Ok(())
    }
}

struct ToyBitmap {
    file_header: BitmapFileHeader,
    info_header: BitmapInfoHeader,
    color_table: ColorTable,
}

#[derive(Debug)]
enum BitmapError {
    NotBitmap([u8; 2]),
    UnsupportedInfoHeaderSize(u32),
    UnsupportedColorPalette(u16),
    InvalidColorTableDataLength(u32),
    Io(Error),
}

impl<R> TryDecode<R> for ToyBitmap
where
    R: Read,
{
    type Error = BitmapError;

    fn handle(reader: &mut R) -> Result<Self, Self::Error> {
        let file_header = reader.try_decode().map_err(|e| match e {
            BitmapFileHeaderError::InvalidSignature(b) => BitmapError::NotBitmap(b),
            BitmapFileHeaderError::Io(e) => BitmapError::Io(e),
        })?;

        let info_header = reader.try_decode().map_err(|e| match e {
            BitmapInfoHeaderError::UnsupportedInfoHeaderSize(n) => {
                BitmapError::UnsupportedInfoHeaderSize(n)
            }
            BitmapInfoHeaderError::Io(e) => BitmapError::Io(e),
        })?;

        let color_table = reader
            .try_decode_with(ColorTableContext::from_headers(&file_header, &info_header))
            .map_err(|e| match e {
                ColorTableError::UnsupportedColorPalette(n) => {
                    BitmapError::UnsupportedColorPalette(n)
                }
                ColorTableError::InvalidDataLength(n) => {
                    BitmapError::InvalidColorTableDataLength(n)
                }
                ColorTableError::Io(e) => BitmapError::Io(e),
            })?;

        Ok(Self {
            file_header,
            info_header,
            color_table,
        })
    }
}

impl<W> TryEncode<W> for ToyBitmap
where
    W: Write,
{
    type Error = Error;

    fn handle(&self, writer: &mut W) -> Result<(), Self::Error> {
        writer.try_encode(&self.file_header)?;
        writer.try_encode(&self.info_header)?;
        writer.try_encode(&self.color_table)?;
        Ok(())
    }
}

#[test]
fn example() {
    let bytes = include_bytes!("./example.bmp");

    let bitmap: ToyBitmap = {
        let mut rdr = Cursor::new(bytes);
        rdr.try_decode().unwrap()
    };

    let mut buf = Vec::new();
    buf.try_encode(&bitmap).unwrap();

    assert_eq!(bytes.to_vec(), buf);
}
