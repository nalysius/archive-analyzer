//! This module provides some common functions

use crate::errors::ReadNumberFromBytesError;
use crate::zip::constants;
use std::fs;
use std::io::Read;

/**
 * Read a chunk of the file.
 */
pub fn read_chunk(file: &mut fs::File, chunk_size: usize) -> Vec<u8> {
    let mut chunk = Vec::with_capacity(chunk_size);
    let _n = file.by_ref()
                .take(chunk_size as u64)
                .read_to_end(&mut chunk)
                .unwrap();
    return chunk;
}

/**
 * Check the signature of the file
 */
pub fn check_signature(chunk: Vec<u8>) -> String {
    let signature = "unknown";
    let number = u32::from_le_bytes(chunk.try_into().unwrap());
    if number == constants::SIGNATURE_HEADER_LOCAL_FILE {
        return "zip".to_string();
    }
    return signature.to_string();
}

/// Reads a u32 from little indian bytes
pub fn read_u32_le(chunk: &[u8]) -> Result<u32, ReadNumberFromBytesError> {
    if chunk.len() > 4 {
        return Err(ReadNumberFromBytesError::TooManyBytes);
    } else if chunk.len() < 4 {
        return Err(ReadNumberFromBytesError::NotEnoughBytes);
    }
    Ok(u32::from_le_bytes(chunk.try_into().unwrap()))
}

/// Reads a u16 from little indian bytes
pub fn read_u16_le(chunk: &[u8]) -> Result<u16, ReadNumberFromBytesError> {
    if chunk.len() > 2 {
        return Err(ReadNumberFromBytesError::TooManyBytes);
    } else if chunk.len() < 2 {
        return Err(ReadNumberFromBytesError::NotEnoughBytes);
    }
    Ok(u16::from_le_bytes(chunk.try_into().unwrap()))
}

/// Reads a string from bytes
/// The bytes must be ASCII codes
pub fn read_string_bytes(chunk: &[u8]) -> String {
    let mut s = "".to_string();
    for item in chunk {
        s.push(char::from_u32(*item as u32).unwrap());
    }
    return s;
}