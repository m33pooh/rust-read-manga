use std::fmt;
use std::path::PathBuf;

// Define a custom error enum for our application
#[derive(Debug)]
pub enum AppError {
    IoError(std::io::Error),
    NoImagesFound(PathBuf),
    OcrError(String),
    // Add more specific errors later:
    // ImageDecodeError(String),
    // VideoEncodeError(String),
}

// Implement standard Error trait
impl std::error::Error for AppError {}

// Implement Display trait for nice printing
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::IoError(e) => write!(f, "IO Error: {}", e),
            AppError::NoImagesFound(path) => {
                write!(f, "No supported images (png, jpg) found in directory: {:?}", path)
            },
            AppError::OcrError(e) => write!(f, "OCR Error: {}", e),
        }
    }
}

// Allow easy conversion from std::io::Error to our AppError
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err)
    }
}