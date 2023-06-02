//! This module contains readers whose goal is to read and parse a ZIP file

use crate::util::{read_chunk, read_string_bytes, read_u16_le, read_u32_le};
use std::fs::File;
use std::io::{Seek, SeekFrom};
use super::model::{DataDescriptor, LocalFileHeader, StoredFile, ZipFile};

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

/// Represents a reader for DataDescriptor
pub struct DataDescriptorReader {

}

impl DataDescriptorReader {
    /// Read a file and try to create a DataDescriptor
    pub fn read(file: &mut File) -> Result<DataDescriptor, String> {
        let crc32_chunk = read_chunk(file, 4);
        let compressed_size_chunk = read_chunk(file, 4);
        let uncompressed_size_chunk = read_chunk(file, 4);

        let crc32 = read_u32_le(&crc32_chunk);
        let compressed_size = read_u32_le(&compressed_size_chunk);
        let uncompressed_size = read_u32_le(&uncompressed_size_chunk);

        if crc32.is_err() {
            return Err("Unable to read DataDescriptor: unreadable crc32".to_string());
        } else if compressed_size.is_err() {
            return Err("Unable to read DataDescriptor: unreadable compressed size".to_string());
        } else if uncompressed_size.is_err() {
            return Err("Unable to read DataDescriptor: unreadable uncompressed size".to_string());
        }

        Ok(DataDescriptor {
            crc32: crc32.unwrap(),
            compressed_size: compressed_size.unwrap(),
            uncompressed_size: uncompressed_size.unwrap(),
        })
    }
}

/// Represents a reader for StoredFile
pub struct StoredFileReader {

}

impl StoredFileReader {
    /// Read a file and try to create a StoredFile
    pub fn read(file: &mut File) -> Result<StoredFile, String> {
        // Read the offset, or stop the function and return the error
        let offset_in_archive = file.stream_position()
                                .or(Err("Unable to read current position in archive".to_string()))?;
        let local_file_header = LocalFileHeaderReader::read(file)?;
        let file_data = read_chunk(file, local_file_header.compressed_size as usize);
        let mut data_descriptor: Option<DataDescriptor> = None;
        // If bit 3 of general purpose flag is set, read data descriptor
        // TODO: ensure the bit test is done properly
        if local_file_header.general_purpose_flag & 4 == 4 {
            data_descriptor = Some(DataDescriptorReader::read(file)?);
        }

        Ok(StoredFile {
            local_file_header: local_file_header,
            file_data: file_data,
            data_descriptor: data_descriptor,
            // Position is computed in ZipFile
            position_in_archive: None,
            // Position is computed in ZipFile
            position_in_central_directory: None,
            // Set to true when reading the central directory
            found_in_central_directory: false,
            // Set to false only if StoredFile is created by reading the CentralDirectory
            found_in_archive: true,
            offset_in_archive: Some(offset_in_archive as usize),
            // TODO: compute this value when reading the central directory
            offset_from_central_directory: None,
        })
    }
}

/// Represents a reader for ZipFile
pub struct ZipFileReader {

}

impl ZipFileReader {
    /// Read a file and try to create a ZipFile
    pub fn read(file: &mut File) -> Result<ZipFile, String> {
        let mut stored_files: Vec<StoredFile> = Vec::new();
        let mut current_offset;
        // Read the stored files
        loop {
            current_offset = file.stream_position()
                                .or(Err("Unable to read the current position in archive".to_string()))?;
            let stored_file = StoredFileReader::read(file);
            // TODO: improve the detection of the end of stored files
            // Actually, if an invalid stored file is encountered, the program
            // considers it's the beginning of the next section (Archive Decryption Header,
            // Archive Extra Data Record or Central Directory).
            //
            // In theory, it could be a stored file that's corrupted and contain
            // an invalid value.
            // To improve this, look for signatures when possible
            if stored_file.is_err() {
                // Reset the cursor to its previous position, before starting to
                // read this stored file
                file.seek(SeekFrom::Start(current_offset))
                    .or(Err("Unable to move cursor in the archive".to_string()))?;
                break;
            }
            let mut stored_file = stored_file.unwrap();
            stored_file.offset_in_archive = Some(current_offset as usize);
            stored_file.position_in_archive = Some(stored_files.len());
            stored_files.push(stored_file);
        }

        // TODO: read the Archive Decryption Header, Archive Extra Data Record,
        // and Central Directory


        Ok(ZipFile {
            stored_files: stored_files,
            central_directory: None
        })
    }
}