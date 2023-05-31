//! This module contains error types used in the application

/// An error used when reading a number from bytes
#[derive(Debug)]
pub enum ReadNumberFromBytesError {
    /// There is too few bytes to read the number
    NotEnoughBytes,
    /// There are too many bytes to read the number
    TooManyBytes,
}