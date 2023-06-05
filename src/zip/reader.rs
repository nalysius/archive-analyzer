//! This module contains readers whose goal is to read and parse a ZIP file

use crate::util::{compare_signature, file_has_remaining_space,read_chunk, read_string_bytes, read_u16_le, read_u32_le, compare_signature_raw, rewind_file_cursor};
use std::fs::File;
use std::io::Seek;
use super::constants;
use super::model::{DataDescriptor, LocalFileHeader, StoredFile, ZipFile, ArchiveExtraDataRecord, CentralDirectory, CentralDirectoryFileHeader, DigitalSignature, EndOfCentralDirectoryRecord};

/// A reader for LocalFileHeader
pub struct LocalFileHeaderReader {
}

impl LocalFileHeaderReader {
    /// Read a file and try to create a LocalFileHeader
    pub fn read(file: &mut File) -> Result<LocalFileHeader, String> {
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
    pub fn read(file: &mut File, position: usize) -> Result<StoredFile, String> {
        // Read the offset, or stop the function and return the error
        let mut offset_in_archive = file.stream_position()
                                .or(Err("Unable to read current position in archive".to_string()))?;

        // The StoredFile begins with a 4-bytes signature. This signature has already been consumed.
        // So substract 4 to the current offset, to match the reality
        offset_in_archive -= 4;
        let local_file_header = LocalFileHeaderReader::read(file)?;
        let file_data = read_chunk(file, local_file_header.compressed_size as usize);
        let mut data_descriptor: Option<DataDescriptor> = None;
        // If bit 3 of general purpose flag is set, read data descriptor
        if local_file_header.general_purpose_flag & 4 == 4 {
            data_descriptor = Some(DataDescriptorReader::read(file)?);
        }

        Ok(StoredFile {
            local_file_header: local_file_header,
            file_data: file_data,
            data_descriptor: data_descriptor,
            // Position is computed in ZipFile
            position: position,
            // Set to true when reading the central directory
            found_in_central_directory: false,
            offset_in_archive: offset_in_archive as usize,
            // TODO: compute this value when reading the central directory
            offset_from_central_directory: None,
        })
    }
}

/// Represents a reader for ArchiveExtraDataRecord
pub struct ArchiveExtraDataRecordReader {

}

impl ArchiveExtraDataRecordReader {
    /// Read a file and try to create an ArchiveExtraDataRecord
    pub fn read(file: &mut File) -> Result<ArchiveExtraDataRecord, String> {
        let extra_field_length_chunk = read_chunk(file, 4);
        let extra_field_length = read_u32_le(&extra_field_length_chunk)
            .or(Err("Unable to read the archive extra data record: unreadable extra field length".to_string()))?;

        let extra_field = read_chunk(file, extra_field_length as usize);
        Ok(ArchiveExtraDataRecord {
            extra_field: extra_field,
        })
    }
}

/// Represents a reader for CentralDirectoryFileHeader
pub struct CentralDirectoryFileHeaderReader {

}

impl CentralDirectoryFileHeaderReader {
    /// Read a file and try to create a CentralDirectory
    pub fn read(file: &mut File) -> Result<CentralDirectoryFileHeader, String> {
        let version_made_by_chunk = read_chunk(file, 2);
        let minimum_version_chunk = read_chunk(file, 2);
        let general_purpose_flag_chunk = read_chunk(file, 2);
        let compression_method_chunk = read_chunk(file, 2);
        let file_last_modification_time_chunk = read_chunk(file, 2);
        let file_last_modification_date_chunk = read_chunk(file, 2);
        let crc32_chunk = read_chunk(file, 4);
        let compressed_size_chunk = read_chunk(file, 4);
        let uncompressed_size_chunk = read_chunk(file, 4);
        let filename_length_chunk = read_chunk(file, 2);
        let extra_field_length_chunk = read_chunk(file, 2);
        let file_comment_length_chunk = read_chunk(file, 2);
        let disk_number_where_file_starts_chunk = read_chunk(file, 2);
        let internal_file_attributes_chunk = read_chunk(file, 2);
        let external_file_attributes_chunk = read_chunk(file, 4);
        let relative_offset_of_local_header_chunk = read_chunk(file, 4);

        // Handle errors and read numbers from values above
        let version_made_by = read_u16_le(&version_made_by_chunk)
            .or(Err("Unable to read the central directory file header: unreadable version made by".to_string()))?;
        let minimum_version = read_u16_le(&minimum_version_chunk)
            .or(Err("Unable to read the central directory file header: unreadable minimum version".to_string()))?;
        let general_purpose_flag = read_u16_le(&general_purpose_flag_chunk)
            .or(Err("Unable to read the central directory file header: unreadable general purpose flag".to_string()))?;
        let compression_method = read_u16_le(&compression_method_chunk)
            .or(Err("Unable to read the central directory file header: unreadable compression method".to_string()))?;
        let file_last_modification_time = read_u16_le(&file_last_modification_time_chunk)
            .or(Err("Unable to read the central directory file header: unreadable file last modification time".to_string()))?;
        let file_last_modification_date = read_u16_le(&file_last_modification_date_chunk)
            .or(Err("Unable to read the central directory file header: unreadable file last modification date".to_string()))?;
        let crc32 = read_u32_le(&crc32_chunk)
            .or(Err("Unable to read the central directory file header: unreadable crc32".to_string()))?;
        let compressed_size = read_u32_le(&compressed_size_chunk)
            .or(Err("Unable to read the central directory file header: unreadable compressed size".to_string()))?;
        let uncompressed_size = read_u32_le(&uncompressed_size_chunk)
            .or(Err("Unable to read the central directory file header: unreadable uncompressed size".to_string()))?;
        let filename_length = read_u16_le(&filename_length_chunk)
            .or(Err("Unable to read the central directory file header: unreadable filename length".to_string()))?;
        let extra_field_length = read_u16_le(&extra_field_length_chunk)
            .or(Err("Unable to read the central directory file header: unreadable extra field length".to_string()))?;
        let file_comment_length = read_u16_le(&file_comment_length_chunk)
            .or(Err("Unable to read the central directory file header: unreadable file comment length".to_string()))?;
        let disk_number_where_file_starts = read_u16_le(&disk_number_where_file_starts_chunk)
            .or(Err("Unable to read the central directory file header: unreadable disk number where file starts".to_string()))?;
        let internal_file_attributes = read_u16_le(&internal_file_attributes_chunk)
            .or(Err("Unable to read the central directory file header: unreadable internal file attributes".to_string()))?;
        let external_file_attributes = read_u32_le(&external_file_attributes_chunk)
            .or(Err("Unable to read the central directory file header: unreadable external file attributes".to_string()))?;
        let relative_offset_of_local_header = read_u32_le(&relative_offset_of_local_header_chunk)
            .or(Err("Unable to read the central directory file header: unreadable internal file attributes".to_string()))?;

        let filename_chunk = read_chunk(file, filename_length as usize);
        let extra_field_chunk = read_chunk(file, extra_field_length as usize);
        let file_comment_chunk = read_chunk(file, file_comment_length as usize);

        let filename = read_string_bytes(&filename_chunk);
        let file_comment = read_string_bytes(&file_comment_chunk);

        Ok(CentralDirectoryFileHeader {
            version_made_by: version_made_by,
            minimum_version: minimum_version,
            general_purpose_flag: general_purpose_flag,
            compression_method,
            file_last_modification_time: file_last_modification_time,
            file_last_modification_date: file_last_modification_date,
            crc32: crc32,
            compressed_size: compressed_size,
            uncompressed_size: uncompressed_size,
            disk_start: disk_number_where_file_starts,
            internal_file_attributes: internal_file_attributes,
            external_file_attributes: external_file_attributes,
            local_file_header_offset: relative_offset_of_local_header,
            filename: filename,
            extra_field: extra_field_chunk,
            file_comment: file_comment,
            position: None,
        })
    }
}

/// Represents a reader for DigitalSignature
pub struct DigitalSignatureReader {

}

impl DigitalSignatureReader {
    /// Read a file and try to create a DigitalSignature
    pub fn read(file: &mut File) -> Result<DigitalSignature, String> {
        let size_of_data_chunk = read_chunk(file, 2);
        let size_of_data = read_u16_le(&size_of_data_chunk)
            .or(Err("Unable to read digital signature: unreadable size of data".to_string()))?;

        let signature_data_chunk = read_chunk(file, size_of_data as usize);

        Ok(DigitalSignature {
            signature_data: signature_data_chunk,
        })
    }
}

/// Represents a reader for EndOfCentralDirectoryRecord
pub struct EndOfCentralDirectoryRecordReader {

}

impl EndOfCentralDirectoryRecordReader {
    /// Read a file and try to create a EndOfCentralDirectoryRecord
    pub fn read(file: &mut File) -> Result<EndOfCentralDirectoryRecord, String> {
        let number_of_this_disk_chunk = read_chunk(file, 2);
        let disk_where_central_directory_starts_chunk = read_chunk(file, 2);
        let number_of_central_directory_records_on_this_disk_chunk = read_chunk(file, 2);
        let total_number_of_central_directory_records_chunk = read_chunk(file, 2);
        let size_of_central_directory_chunk = read_chunk(file, 4);
        let offset_start_of_central_directory_from_archive_chunk = read_chunk(file, 4);
        let comment_length_chunk = read_chunk(file, 2);
        let comment_length = read_u16_le(&comment_length_chunk)
            .or(Err("Unable to read end of central directory: unreadable comment length".to_string()))?;
        let comment_chunk = read_chunk(file, comment_length as usize);
        let comment = read_string_bytes(&comment_chunk);

        let number_of_this_disk = read_u16_le(&number_of_this_disk_chunk)
            .or(Err("Unable to read end of central directory: unreadable disk number".to_string()))?;
        let disk_where_central_directory_starts = read_u16_le(&disk_where_central_directory_starts_chunk)
            .or(Err("Unable to read end of central directory: unreadable disk where central directory starts".to_string()))?;
        let number_of_central_directory_records_on_this_disk = read_u16_le(&number_of_central_directory_records_on_this_disk_chunk)
            .or(Err("Unable to read end of central directory: unreadable number of central directory records on this disk".to_string()))?;
        let total_number_of_central_directory_records = read_u16_le(&total_number_of_central_directory_records_chunk)
            .or(Err("Unable to read end of central directory: unreadable total number or central directory records".to_string()))?;
        let size_of_central_directory = read_u32_le(&size_of_central_directory_chunk)
            .or(Err("Unable to read end of central directory: unreadable size of central directory".to_string()))?;
        let offset_start_of_central_directory_from_archive = read_u32_le(&offset_start_of_central_directory_from_archive_chunk)
            .or(Err("Unable to read end of central directory: unreadable offset of start of central directory from archive".to_string()))?;

        Ok(EndOfCentralDirectoryRecord {
            disk_number: number_of_this_disk,
            disk_start_central_directory: disk_where_central_directory_starts,
            central_directory_records_number_on_disk: number_of_central_directory_records_on_this_disk,
            central_directory_records_total_number: total_number_of_central_directory_records,
            central_directory_size: size_of_central_directory,
            offset_start_central_directory: offset_start_of_central_directory_from_archive,
            comment: comment,
        })
    }
}

/// Represents a reader for CentralDirectory
pub struct CentralDirectoryReader {

}

impl CentralDirectoryReader {
    pub fn read(file: &mut File) -> Result<CentralDirectory, String> {
        let mut central_directory_file_headers: Vec<CentralDirectoryFileHeader> = Vec::new();
        let offset_from_start_of_archive = file.stream_position()
            .or(Err("Unable to read the current position in the archive".to_string()))?;

        while compare_signature(file, constants::SIGNATURE_HEADER_CENTRAL_DIRECTORY)? {
            // TODO: Better handle errors.
            // Actually if one header is unreadable, the whole CentralDirectory will be
            // ignored. It would be better to log the error and ignore only this header
            let mut header = CentralDirectoryFileHeaderReader::read(file)?;
            header.position = Some(central_directory_file_headers.len());
            central_directory_file_headers.push(header);
        }

        // TODO: as in ZipFileReader, loop over the file byte by byte to
        // find the next signature.
        // by doing so, it would be possible to ignore a damaged central directory header
        // while still being able to read the next one / next part of the file.

        // Check if digital signature is present
        let mut digital_signature = None;
        if compare_signature(file, constants::SIGNATURE_CENTRAL_DIRECTORY_DIGITAL_SIGNATURE)
                .or::<String>(Ok(false))
                .unwrap()
        {
            digital_signature = Some(DigitalSignatureReader::read(file)?);
        }

        // Read end of central directory record
        let mut end_of_central_directory_record = None;
        if compare_signature(file, constants::SIGNATURE_END_OF_CENTRAL_DIRECTORY_RECORD)
                .or::<String>(Ok(false))
                .unwrap() {
            end_of_central_directory_record = Some(EndOfCentralDirectoryRecordReader::read(file)?);
        }

        if end_of_central_directory_record.is_none() {
            return Err("Unable to read central directory: end of central directory not found".to_string());
        }

        Ok(CentralDirectory {
            file_headers: central_directory_file_headers,
            digital_signature,
            end_of_central_directory_record: end_of_central_directory_record.unwrap(),
            offset_from_start_of_archive: offset_from_start_of_archive as usize,
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
        // Read the stored files
        while compare_signature(file, constants::SIGNATURE_HEADER_LOCAL_FILE)
                .or::<String>(Ok(false)).unwrap()
        {
            let current_offset = file.stream_position()
                .or(Err("Unable to read current position in archive"))?;
            let stored_file = StoredFileReader::read(file, stored_files.len());

            if stored_file.is_ok() {
                stored_files.push(stored_file.unwrap());
            } else {
                // If the stored file cannot be read, reset the file cursor
                // and continue reading manually
                let new_current_offset = file.stream_position()
                    .or(Err("Unable to read current position in archive"))?;
                rewind_file_cursor(file, new_current_offset - current_offset)?
            }
        }

        let mut archive_extra_data_record = None;
        let mut central_directory = None;

        /*
         * We can reach this point in several cases:
         * 1. At least one of the Local File Headers was unreadable, because the
         *    file was damaged.
         * 2. All the Local File Headers have been read, and next there is another
         *    section (Archive Decryption Header, Archive Extra Data Header, or
         *    Central Directory).
         * 3. The end of the file is truncated, and we reached the end.
         *
         * To have more chance to read any content from the file, I decided to
         * ignore any unreadable part and to try to find another readable
         * section as soon as possible.
         * That means that once we finished to read the series of Local File
         * Headers, signatures are checked for Local File Header, Archive Extra
         * Data Record, and Central Directory.
         * TODO: add check and reading for Archive Decryption Header
         * By doing this, as soon as a known section is found somewhere in the
         * file, reading can continue.
         *
         * Note: there is a little chance of false positive. While low, it's
         * possible to have 4 bytes somewhere whose value matches a signature.
         * It would break reading of the rest of the file.
         */
        loop {
            let chunk = read_chunk(file, 4);
            // Did we found another local file header?
            if compare_signature_raw(file, &chunk, constants::SIGNATURE_HEADER_LOCAL_FILE, false)? {
                let stored_file = StoredFileReader::read(file, stored_files.len());
                // TODO: handle the case if stored_file is an Err. Log it, at least
                if stored_file.is_ok() {
                    stored_files.push(stored_file.unwrap());
                }
            } else if compare_signature_raw(file, &chunk, constants::SIGNATURE_ARCHIVE_EXTRA_DATA_RECORD, false)? {
                // Did we found the archive extra data record?
                archive_extra_data_record = Some(ArchiveExtraDataRecordReader::read(file)?);
            } else if compare_signature_raw(file, &chunk, constants::SIGNATURE_HEADER_CENTRAL_DIRECTORY, false)? {
                // Did we found the central directory?
                // This struct is repeated for each file, so the CentralDirectoryReader
                // will loop on each file. For this, it needs to read the signature. Since
                // we already consumed it because of the usage of compare_signature_raw(),
                // rewind the file cursor.
                rewind_file_cursor(file, 4)?;
                let cd_result = CentralDirectoryReader::read(file);
                if cd_result.is_ok() {
                    let cd = cd_result.unwrap();
                    central_directory = Some(cd);

                    // Set StoredFile values with the ones found in CentralDirectory
                    for stored_file in &mut stored_files {
                        stored_file.update_from_central_directory(central_directory.as_ref().unwrap());
                    }
                } else {
                    // TODO: use a logger instead of printing to STDOUT
                    if let Err(e) = cd_result {
                        println!("ERROR when reading central directory header: {}", e);
                    }
                }
                // Central directory is the last part of a ZIP, if we found it
                // we can exit the loop
                break;
            } else if !file_has_remaining_space(file, 4)? {
                // It seems we reached the end of the file, stop here
                break;
            } else {
                // We didn't find anything. Shift of 1 byte, and try again
                rewind_file_cursor(file, 3)?;
            }
        }

        Ok(ZipFile {
            stored_files: stored_files,
            archive_extra_data_record: archive_extra_data_record,
            central_directory: central_directory,
        })
    }
}