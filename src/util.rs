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
/// The 4 bytes of signature are consumed ONLY if the signature matches.
/// Otherwise, the cursor is reset to its previous position.
///
/// Note: see compare_signature_raw() which does most of the job.
pub fn compare_signature(file: &mut File, signature: u32) -> Result<bool, String> {
    let chunk = read_chunk(file, 4);
    compare_signature_raw(file, &chunk, signature, true)
}

/// Compare the given 4 bytes to a signature.
/// This helps to detect the next part of the file.
///
/// This function consumes the signature ONLY if its matches. If the signature
/// matches, the caller will probably want to read the next part of the file, and
/// the signature is not useful anymore once it's checked.
/// However, if the signature doesn't match, the cursor of the file is reset to
/// its previous position, as if the signature wasn't checked, except if
/// rewind_on_mismatch is set to false. This parameter is mainly useful for
/// manual signature comparison.
///
/// Note: in case of error, the file cursor is not reset. Usually not a problem
/// since the Err is usually returned by the caller in order to stop operations on
/// the file.
pub fn compare_signature_raw(file: &mut File, signature_1: &[u8], signature_2: u32, rewind_on_mismatch: bool) -> Result<bool, String> {
    let value = read_u32_le(&signature_1)
        .or(Err("Unable to compare signature"))?;

    let signature_match = value == signature_2;
    // The bytes of the signature have already been read by the caller
    // so if the signature doesn't match, rewind the cursor of 4 bytes.
    if !signature_match && rewind_on_mismatch {
        rewind_file_cursor(file, 4)?;
    }
    Ok(signature_match)
}

/// Check if a file has enough bytes remaining to read
/// It's a helper function to detect if we're at the end of the file
pub fn file_has_remaining_space(file: &mut File, number_of_bytes: u32) -> Result<bool, String> {
    let current_offset = file.stream_position()
        .or(Err("Unable to read current position in archive"))?;

    let end_of_file = file.seek(SeekFrom::End(0)).or(Err("Unable to move cursor to end of archive"))?;

    // Reset the cursor as its original position
    file.seek(SeekFrom::Start(current_offset))
        .or(Err("Unable to move cursor in archive"))?;

    Ok(end_of_file - current_offset > number_of_bytes as u64)
}

/// Rewind the cursor of file of number_of_bytes bytes.
/// Returns true if it worked, false if an error occured
pub fn rewind_file_cursor(file: &mut File, number_of_bytes: u64) -> Result<(), String> {
    let current_offset = file.stream_position()
        .or(Err("Unable to read current position in archive"))?;

    file.seek(SeekFrom::Start(current_offset-number_of_bytes))
        .or(Err("Unable to move the cursor in the archive"))?;

    return Ok(());
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