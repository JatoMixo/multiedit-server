#[derive(Debug)]
pub enum FileTrackingError {
    CantOpenElement,
    CantReadContentOfFile,
    CantWriteContentToFile,
}
