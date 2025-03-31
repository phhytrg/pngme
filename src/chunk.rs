use crc::CRC_32_ISO_HDLC;
use std::{array::TryFromSliceError, fmt::Display};

use crate::chunk_type::{ChunkType, ParseChunkTypeError};

#[derive(Debug)]
pub struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>,
    length: u32,
    crc: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum ParseChunkError {
    #[error("Length not found")]
    LengthNotFound,
    #[error("Chunk type not found")]
    ChunkTypeNotFound,
    #[error("Message not found")]
    MessageNotFound,
    #[error("Crc not found")]
    CrcNotFound,
    #[error("Invalid Crc")]
    InvalidCrc,
    #[error("Parse slice error")]
    ParseSliceError(#[from] TryFromSliceError),
    #[error("Parse chunk type error")]
    ParseChunkTypeError(#[from] ParseChunkTypeError),
}

impl TryFrom<&Vec<u8>> for Chunk {
    type Error = ParseChunkError;

    fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
        let length: u32 = u32::from_be_bytes(
            value
                .get(0..4)
                .ok_or(ParseChunkError::LengthNotFound)?
                .try_into()?,
        );
        let chunk_type: [u8; 4] = value
            .get(4..8)
            .ok_or(ParseChunkError::ChunkTypeNotFound)?
            .try_into()?;
        let chunk_type: ChunkType =
            ChunkType::try_from(chunk_type)?;
        let msg: Vec<u8> = value
            .get(8..(length + 8) as usize)
            .ok_or(ParseChunkError::MessageNotFound)?
            .to_vec();
        let crc = u32::from_be_bytes(
            value
                .get((length + 8) as usize..)
                .ok_or(ParseChunkError::CrcNotFound)?
                .try_into()?
        );
        let checked_crc =
            crc::Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(&value[4..(length + 8) as usize]);
        if crc != checked_crc {
            return Err(ParseChunkError::InvalidCrc);
        }
        Ok(Self {
            length,
            chunk_type,
            data: msg,
            crc: crc,
        })
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let mut bytes: Vec<u8> = chunk_type.bytes().to_vec();
        bytes.extend(&data);
        let length = data.len() as u32;
        let crc = crc::Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(bytes.as_slice());
        Chunk {
            chunk_type,
            data,
            length,
            crc: crc,
        }
    }
    pub fn length(&self) -> u32 {
        self.data.len() as u32
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    pub fn crc(&self) -> u32 {
        let bytes: &[u8] = &[&self.chunk_type.bytes(), self.data.as_slice()].concat();
        crc::Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(bytes)
    }
    pub fn data_as_string(&self) -> Result<String, anyhow::Error> {
        Ok(self
            .data
            .iter()
            .map(|&byte| byte as char)
            .collect::<String>())
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_variables)]
    #![allow(dead_code)]
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
