#[derive(Debug, serde::Serialize)]
pub enum FileTrackingError {
    CantOpenElement,
    CantReadContentOfFile,
    CantWriteContentToFile,
}
