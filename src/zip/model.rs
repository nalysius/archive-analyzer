//! This module contains the structs and enums used to represent a ZIP file

/// Represents a local file header
pub struct LocalFileHeader {
    /// The minimum version to extract
    pub minimum_version: u16,
    /// A general purpose bit flag
    pub general_purpose_flag: u16,
    /// The compression method
    pub compression_method: u16,
    /// Last modification time of the file
    pub file_last_modification_time: u16,
    /// Last modification date of the file
    pub file_last_modification_date: u16,
    /// CRC32 of the file
    pub crc32: u32,
    /// Compressed size of the file
    pub compressed_size: u32,
    /// Uncompressed size of the file
    pub uncompressed_size: u32,
    /// The filename,
    pub filename: String,
    /// The extra field
    pub extra_field: Vec<u8>,
}