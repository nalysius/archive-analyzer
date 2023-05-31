//! This module contains readers whose goal is to read and parse a ZIP file

use std::fs::File;

use super::model::LocalFileHeader;
use crate::util::{read_chunk, read_string_bytes, read_u16_le, read_u32_le};

/// A reader for LocalFileHeader
pub struct LocalFileHeaderReader {
}

impl LocalFileHeaderReader {
    /// Read a file and try to create a LocalFileHeader
    pub fn read(file: &mut File) -> Result<LocalFileHeader, String> {
        let _chunk = read_chunk(file, 4); // local file header
        let minimum_version_chunk = read_chunk(file, 2); // Minimum version to extract
        let general_purpose_flag_chunk = read_chunk(file, 2); // General purpose bit flag
        let compression_method_chunk = read_chunk(file, 2); // Compression method
        let file_last_modification_time_chunk = read_chunk(file, 2); // File last modification time
        let file_last_modification_date_chunk = read_chunk(file, 2); // File last modification date
        let crc32_chunk = read_chunk(file, 4); // CRC32
        let compressed_size_chunk = read_chunk(file, 4); // Compressed size
        let uncompressed_size_chunk = read_chunk(file, 4); // Uncompressed size
        let filename_length_chunk = read_chunk(file, 2); // File name length
        let filename_length = u16::from_le_bytes(filename_length_chunk.try_into().unwrap());
        let extra_fields_length_chunk = read_chunk(file, 2); // Extra field length
        let extra_fields_length = u16::from_le_bytes(extra_fields_length_chunk.try_into().unwrap());
        let filename_chunk = read_chunk(file, filename_length as usize); // File name
        let extra_field_chunk = read_chunk(file, extra_fields_length as usize); // Extra field

        let minimum_version = read_u16_le(&minimum_version_chunk);
        let general_purpose_flag = read_u16_le(&general_purpose_flag_chunk);
        let compression_method = read_u16_le(&compression_method_chunk);
        let file_last_modification_time = read_u16_le(&file_last_modification_time_chunk);
        let file_last_modification_date = read_u16_le(&file_last_modification_date_chunk);
        let crc32 = read_u32_le(&crc32_chunk);
        let compressed_size = read_u32_le(&compressed_size_chunk);
        let uncompressed_size = read_u32_le(&uncompressed_size_chunk);

        if minimum_version.is_err() {
            return Err("Unable to read Local File Header: unreadable minimum version.".to_string());
        } else if general_purpose_flag.is_err() {
            return Err("Unable to read Local File Header: unreadable general purpose flag.".to_string());
        } else if compression_method.is_err() {
            return Err("Unable to read Local File Header: unreadable compression method.".to_string());
        } else if file_last_modification_time.is_err() {
            return Err("Unable to read Local File Header: unreadable file last modification time.".to_string());
        } else if file_last_modification_date.is_err() {
            return Err("Unable to read Local File Header: unreadable file last modification date.".to_string());
        } else if crc32.is_err() {
            return Err("Unable to read Local File Header: unreadable crc32.".to_string());
        } else if compressed_size.is_err() {
            return Err("Unable to read Local File Header: unreadable compressed size.".to_string());
        } else if uncompressed_size.is_err() {
            return Err("Unable to read Local File Header: unreadable uncompressed size.".to_string());
        }

        Ok(LocalFileHeader {
            minimum_version: minimum_version.unwrap(),
            general_purpose_flag: general_purpose_flag.unwrap(),
            compression_method: compression_method.unwrap(),
            file_last_modification_time: file_last_modification_time.unwrap(),
            file_last_modification_date: file_last_modification_date.unwrap(),
            crc32: crc32.unwrap(),
            compressed_size: compressed_size.unwrap(),
            uncompressed_size: uncompressed_size.unwrap(),
            filename: read_string_bytes(&filename_chunk),
            extra_field: extra_field_chunk,
        })
    }
}