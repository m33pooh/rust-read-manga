use thiserror::Error;

#[derive(Debug, Error)]
pub enum MangaError {
    #[error("File not found")]
    NotFound,
}
