#![allow(unused_variables)]

use std::{array::TryFromSliceError, fmt::Display, str::FromStr};

#[derive(thiserror::Error, Debug)]
pub enum ParseChunkTypeError {
    #[error("Invalid length (expected 4), got {:?}", found)]
    InvalidLength { found: usize },
    #[error("Invalid length, got {0}")]
    InvalidByte(char),
    #[error("Slice conversion failed: {0}")]
    ParseSliceError(#[from] TryFromSliceError),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChunkType([u8; 4]);

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ParseChunkTypeError;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0.iter().map(|byte| *byte as char).collect::<String>()
        )
    }
}

impl FromStr for ChunkType {
    type Err = ParseChunkTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chunk_type_bytes: [u8; 4] = s
            .chars()
            .map(|c| match c {
                c if c.is_alphabetic() => Ok(c as u8),
                _ => Err(ParseChunkTypeError::InvalidByte(c)),
            })
            .collect::<Result<Vec<u8>, ParseChunkTypeError>>()?
            .as_slice()
            .try_into()?;
        Ok(Self(chunk_type_bytes))
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.0
            .clone()
            .try_into()
            .expect("Chunk type must have exactly 4 elements")
    }

    pub fn is_critical(&self) -> bool {
        self.0[0].is_ascii_uppercase()
    }

    pub fn is_public(&self) -> bool {
        self.0[1].is_ascii_uppercase()
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.0[2].is_ascii_uppercase()
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.0[3].is_ascii_lowercase()
    }

    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }

    #[test]
    pub fn test_from_invalid_string() {
        let result: Result<_, ParseChunkTypeError> = ChunkType::from_str("RuStAbc");
        assert!(matches!(
            result,
            Err(ParseChunkTypeError::ParseSliceError(_))
        ));
    }
}
