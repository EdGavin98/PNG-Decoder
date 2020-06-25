use crate::chunk_type::ChunkType;
use std::fmt::{Display, Formatter};
use std::convert::{TryFrom, TryInto};
use crc::crc32;
use anyhow::anyhow;
use core::fmt;

use crate::{Error, Result};
use std::str::FromStr;

pub struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>
}

impl Chunk {

    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        Chunk {
            chunk_type,
            data
        }
    }

    pub fn length(&self) -> u32 {
        self.data.len() as u32
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data.as_slice()
    }

    pub fn crc(&self) -> u32 {
        let mut check: Vec<u8> = Vec::new();
        check.extend(&self.chunk_type.bytes());
        check.extend(&self.data);
        crc32::checksum_ieee(check.as_slice())
    }

    pub fn data_as_string(&self) -> Result<String> {
        let as_string =  std::str::from_utf8(self.data.as_slice())?;
        Ok(as_string.to_string())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut as_bytes = Vec::new();
        as_bytes.extend(&self.length().to_be_bytes());
        as_bytes.extend(&self.chunk_type.bytes());
        as_bytes.extend(&self.data);
        as_bytes.extend(&self.crc().to_be_bytes());
        as_bytes
    }

}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        //Check length, crc, length and type are required, so should be 12 at minimum.
        if value.len() < 12 {
            return Err(anyhow!("Not enough chunk data!"))
        }
        let length = u32::from_be_bytes(value[0..4].try_into()?);
        let length = usize::try_from(length)?;
        let data_end_point= 8 + length;

        let chunk_type = std::str::from_utf8(&value[4..8])?;
        let data = Vec::from(&value[8..data_end_point]);
        let crc = &value[data_end_point..data_end_point + 4];

        let chunk_type = ChunkType::from_str(&chunk_type)?;
        let data = Vec::from(data);

        let chunk = Chunk::new(chunk_type, data);

        let crc = u32::from_be_bytes([crc[0], crc[1], crc[2], crc[3]]);
        if chunk.crc() != crc {
             return Err(anyhow!("Invalid CRC"));
        }

        Ok(chunk)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.data_as_string().unwrap(), self.length(), self.chunk_type)
    }
}



/////////////    ////////////    /////////////   //////////////
     //          //              //                    //
     //          //              //                    //
     //          //////////      ////////////          //
     //          //                        //          //
     //          //                        //          //
     //          ////////////    ////////////          //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data: Vec<u8> = "This is where your secret message will be!"
            .bytes()
            .collect();
        Chunk::new(chunk_type, data)
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
}