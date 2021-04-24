use std::{error::Error, fmt::Display, io};

use image::ImageError;

#[derive(Debug)]
pub enum AtlasError {
    ImageError(ImageError),
    IoError(io::Error),
    /// Describes when images are larger than the maximum atlas size or require more atlantes
    PackingError(&'static str),
}

impl Error for AtlasError {}

impl Display for AtlasError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AtlasError::ImageError(err) => {err.fmt(f)}
            AtlasError::IoError(err) => {err.fmt(f)}
            AtlasError::PackingError(err) => {err.fmt(f)}
        }
    }
}

impl From<ImageError> for AtlasError {
    fn from(err: ImageError) -> AtlasError {
        AtlasError::ImageError(err)
    }
}

impl From<io::Error> for AtlasError {
    fn from(err: io::Error) -> AtlasError {
        AtlasError::IoError(err)
    }
}