//! This module contains the structs and enums used to represent a ZIP file
//! Official ZIP specification: https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT
//! Other docs about structure at https://users.cs.jmu.edu/buchhofp/forensics/formats/pkzip-printable.html
//! and https://docs.fileformat.com/compression/zip/

/// Represents a local file header
pub struct LocalFileHeader {
    /// The minimum version to extract
    pub minimum_version: u16,
    /// A general purpose bit flag
    /// Bit 00: encrypted file
    /// Bit 01: compression option
    /// Bit 02: compression option
    /// Bit 03: data descriptor
    /// Bit 04: enhanced deflation
    /// Bit 05: compressed patched data
    /// Bit 06: strong encryption
    /// Bit 07-10: unused
    /// Bit 11: language encoding
    /// Bit 12: reserved
    /// Bit 13: mask header values
    /// Bit 14-15: reserved
    pub general_purpose_flag: u16,
    /// The compression method
    /// 00: no compression
    /// 01: shrunk
    /// 02: reduced with compression factor 1
    /// 03: reduced with compression factor 2
    /// 04: reduced with compression factor 3
    /// 05: reduced with compression factor 4
    /// 06: imploded
    /// 07: reserved
    /// 08: deflated
    /// 09: enhanced deflated
    /// 10: PKWare DCL imploded
    /// 11: reserved
    /// 12: compressed using BZIP2
    /// 13: reserved
    /// 14: LZMA
    /// 15-17: reserved
    /// 18: compressed using IBM TERSE
    /// 19: IBM LZ77 z
    /// 98: PPMd version I, Rev 1
    pub compression_method: u16,
    /// Last modification time of the file
    /// Bits 00-04: seconds divided by 2
    /// Bits 05-10: minute
    /// Bits 11-15: hour
    pub file_last_modification_time: u16,
    /// Last modification date of the file
    /// Bits 00-04: day
    /// Bits 05-08: month
    /// Bits 09-15: years from 1980
    pub file_last_modification_date: u16,
    /// CRC32 of the file
    /// value computed over file data by CRC-32 algorithm with
    /// 'magic number' 0xdebb20e3 (little endian)
    pub crc32: u32,
    /// Compressed size of the file
    /// if archive is in ZIP64 format, this filed is 0xffffffff and the length
    /// is stored in the extra field
    pub compressed_size: u32,
    /// Uncompressed size of the file
    /// if archive is in ZIP64 format, this filed is 0xffffffff and the length is
    /// stored in the extra field
    pub uncompressed_size: u32,
    /// The filename
    pub filename: String,
    /// The extra field
    /// Used to store additional information. The field consistes of a sequence of
    /// header and data pairs, where the header has a 2 byte identifier and a 2
    /// bytes data size field.
    pub extra_field: Vec<u8>,
}

/// Represents a Data Descriptor for a file stored in a ZIP.
/// Used only when third bit of the flag in Local File Header is set.
pub struct DataDescriptor {
    /// The crc32 of the file
    pub crc32: u32,
    /// The size of the compressed size
    pub compressed_size: u32,
    /// The size of the uncompressed size
    pub uncompressed_size: u32,
}

/// Represents a file stored in a ZIP
pub struct StoredFile {
    /// The local file header
    pub local_file_header: LocalFileHeader,
    /// The file data (uncompressed)
    pub file_data: Vec<u8>,
    /// The optional data descriptor
    pub data_descriptor: Option<DataDescriptor>,
    /// The position of the file in the archive (0-based)
    /// The position is about the order of the files in
    /// the archive, not the order in the central directory
    pub position: usize,
    /// Whether this file was present in the central directory.
    /// If false, that means the file was improperly removed from the archive
    /// or hidden
    pub found_in_central_directory: bool,
    /// The offset in bytes from the beginning of the archive file.
    /// This value can be used to compare with the value stored in the central
    /// directory.
    pub offset_in_archive: usize,
    /// The offset in bytes as read from the central directory.
    /// This value can be used to compare with the real position of the file
    /// in the archive.
    /// Note: if the file is not announced in the central directory but is
    /// present in the archive, this value is None.
    pub offset_from_central_directory: Option<usize>,
}

impl StoredFile {
    /// Update fields related to central directory
    pub fn update_from_central_directory(&mut self, central_directory: &CentralDirectory) {
        for central_directory_file_header in &central_directory.file_headers {
            if central_directory_file_header.filename == self.local_file_header.filename {
                self.found_in_central_directory = true;
                self.offset_from_central_directory = Some(central_directory.offset_from_start_of_archive - self.offset_in_archive);
            }
        }
    }
}

/// Represents an Archive Decryption Header
pub struct ArchiveDecryptionHeader {
    /// The optional archive extra data record
    pub archive_extra_data_record: Option<ArchiveExtraDataRecord>,
    /// The central directory
    /// Has to be encrypted / decrypted when writing / reading in a file
    pub central_directory: CentralDirectory,
}

/// Represents an Archive Extra Data Record
pub struct ArchiveExtraDataRecord {
    /// The extra field
    pub extra_field: Vec<u8>,
}

/// Represents a File Header in the Central Directory
pub struct CentralDirectoryFileHeader {
    /// The version of zip spec used to make the file
    pub version_made_by: u16,
    /// The version of zip spec needed to to extract
    pub minimum_version: u16,
    /// A general purpose bits flag
    pub general_purpose_flag: u16,
    /// The compression method
    pub compression_method: u16,
    /// Time of last modification of the file
    pub file_last_modification_time: u16,
    /// Date of last modification of the file
    pub file_last_modification_date: u16,
    /// CRC32 of the file
    pub crc32: u32,
    /// File's compressed size
    pub compressed_size: u32,
    /// File's uncompressed size
    pub uncompressed_size: u32,
    /// Disk number where file starts
    pub disk_start: u16,
    /// Internal file attributes
    pub internal_file_attributes: u16,
    /// External file attributes
    pub external_file_attributes: u32,
    /// The number of bytes between the start of the first disk on which the
    /// file occurs, and the start of the local file header
    pub local_file_header_offset: u32,
    /// The filename
    pub filename: String,
    /// The extra field
    pub extra_field: Vec<u8>,
    /// The file comment
    pub file_comment: String,
    /// The position of the file in the central directory
    pub position: Option<usize>,
}

/// Represents a digital signature in Central Directory
pub struct DigitalSignature {
    /// The signature data
    pub signature_data: Vec<u8>,
}

/// Represents the end of the Central Directory
pub struct EndOfCentralDirectoryRecord {
    /// Current disk number
    pub disk_number: u16,
    /// Disk where the central directory starts
    pub disk_start_central_directory: u16,
    /// The number of central directory records on this disk
    pub central_directory_records_number_on_disk: u16,
    /// The total number of central directory records
    pub central_directory_records_total_number: u16,
    /// The size of the central directory in bytes
    pub central_directory_size: u32,
    /// Ofset to start of central directory, relative to start of archive
    pub offset_start_central_directory: u32,
    /// The comment
    pub comment: String,

}

/// Represents the Central Directory
pub struct CentralDirectory {
    /// The file headers
    pub file_headers: Vec<CentralDirectoryFileHeader>,
    pub digital_signature: Option<DigitalSignature>,
    /// The record for end of central directory
    pub end_of_central_directory_record: EndOfCentralDirectoryRecord,

    /// The offset of the central directory, from the start of the archive.
    /// Not in the specification, but it helps to compute the offset of
    /// local file headers relative to the central directory.
    pub offset_from_start_of_archive: usize,
}

/// Represents a whole ZIP file
pub struct ZipFile {
    /// A list of stored file
    pub stored_files: Vec<StoredFile>,
    /// The archive extra data record
    pub archive_extra_data_record: Option<ArchiveExtraDataRecord>,
    /// The central directory
    /// In the specification it's not optional, but in practice
    /// it could let us reading a ZIP file even if the central directory
    /// has been removed / damaged
    pub central_directory: Option<CentralDirectory>,
}