//! This module provides some common functions

use crate::errors::ReadNumberFromBytesError;
use crate::zip::constants;
use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom};

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

/// Compare the 4 next bytes of file to the given signature.
/// This function is a helper to check what the next part of the file is.
///
/// This function consumes the signature ONLY if its matches. If the signature
/// matches, the caller will probably want to read the next part of the file, and
/// the signature is not useful anymore once it's checked.
/// However, if the signature doesn't match, the cursor of the file is reset to
/// its previous position, as if the signature wasn't checked.
///
/// Note: in case of error, the file cursor is not reset. Usually not a problem
/// since the Err is usually returned by the caller in order to stop operations on
/// the file.
pub fn compare_signature(file: &mut File, signature: u32) -> Result<bool, String> {
    let current_offset = file.stream_position()
        .or(Err("Unable to read the current position in the archive".to_string()))?;
    let chunk = read_chunk(file, 4);
    let value = read_u32_le(&chunk)
        .or(Err("Unable to compare signature".to_string()))?;

    let signature_match = value == signature;
    if !signature_match {
        file.seek(SeekFrom::Start(current_offset))
                    .or(Err("Unable to move cursor in the archive".to_string()))?;
    }
    Ok(signature_match)
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