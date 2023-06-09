//! This module declares constants about the ZIP format, like signatures.

/// The signature of a local file header
pub const SIGNATURE_HEADER_LOCAL_FILE: u32 = 67324752; // 0x504b0304 (LE)

/// The signature of the archive extra data record
pub const SIGNATURE_ARCHIVE_EXTRA_DATA_RECORD: u32 = 134630224; // 0x08064b50 (LE)

/// The signature of a central directory header
pub const SIGNATURE_HEADER_CENTRAL_DIRECTORY: u32 = 33639248; // 0x02014b50 (LE)

/// The digital signature, optionally stored after all central directory headers
pub const SIGNATURE_CENTRAL_DIRECTORY_DIGITAL_SIGNATURE: u32 = 84233040; // 0x05054b50 (LE)

/// The signature of a end of central directory record
pub const SIGNATURE_END_OF_CENTRAL_DIRECTORY_RECORD: u32 = 101010256; // 0x06054b50 (LE)

/// The signature of a end of central directory record in a zip64
pub const SIGNATURE_ZIP64_END_OF_CENTRAL_DIRECTORY_RECORD: u32 = 101075792; // 0x06064b50 (LE)

