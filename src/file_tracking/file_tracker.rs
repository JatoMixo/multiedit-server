use std::{
    collections::VecDeque,
    fs::File,
    io::{Read, Write},
};
use crate::file_tracking::Path;
use super::error::FileTrackingError;

#[derive(Debug, serde::Serialize)]
pub struct FileTracker {
    file_path: Path,
    changes_history: VecDeque<FileChange>,
}

impl FileTracker {
    pub fn new(file_path: Path) -> Result<FileTracker, FileTrackingError> {

        let file = File::open(&file_path.get_absolute_path());

        match file {
            Ok(_) => Ok(FileTracker {
                file_path,
                changes_history: VecDeque::new(),
            }),
            Err(_) => Err(FileTrackingError::CantOpenElement),
        }

    }

    fn open_file_for_reading(&self) -> Result<File, FileTrackingError> {
        match File::open(&self.file_path.get_absolute_path()) {
            Ok(file) => Ok(file),
            Err(_) => Err(FileTrackingError::CantOpenElement),
        }
    }

    /// WARN: This will create the file if it doesn't exist
    /// or it'll truncate it if it does, this is because it's using
    /// File::create to open it
    fn open_file_for_writing(&self) -> Result<File, FileTrackingError> {
        match File::create(&self.file_path.get_absolute_path()) {
            Ok(file) => Ok(file),
            Err(_) => Err(FileTrackingError::CantOpenElement),
        }
    }

    pub fn get_current_file_content(&self) -> Result<String, FileTrackingError> {
        let mut file = self.open_file_for_reading()?;

        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Ok(_) => Ok(content),
            Err(_) => Err(FileTrackingError::CantReadContentOfFile),
        }
    }

    fn write_content_to_file(&mut self, content: String) -> Result<(), FileTrackingError> {
        let mut file = self.open_file_for_writing()?;

        match file.write_all(content.as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => Err(FileTrackingError::CantWriteContentToFile),
        }
    }

    // TODO: Make this use references instead of borrowing the object of the FileChange, for
    // optimization reasons
    pub fn apply_change(&mut self, change: FileChange) -> Result<(), FileTrackingError> {
        
        // TODO: This can surely be optimized with some sort of cache stuff to prevent getting the
        // file content over and over whenever a change is applied.
        let mut file_content = self.get_current_file_content()?;

        change.apply_to(&mut file_content);

        self.write_content_to_file(file_content)?;
        self.changes_history.push_back(change);

        Ok(())
    }

    pub fn get_path(&self) -> &Path {
        &self.file_path
    }
}

/// A Change applied to a file. When applying the change, it deletes the content in
/// between the start index and the end index of it, and then inserts the new content
/// there as a sort of replacement. It also contains the ID of the author.
#[derive(Debug, serde::Serialize, Clone)]
pub struct FileChange {
    start_index: usize,
    end_index: usize,
    // TODO: Change this to some sort of [u8]
    content: String,
}

impl FileChange {
    pub fn new(
        start_index: usize,
        end_index: usize,
        content: String,
    ) -> FileChange {
        FileChange {
            start_index,
            end_index,
            content,
        }
    }

    pub fn apply_to(&self, str: &mut String) {
        str.drain(self.start_index..self.end_index);

        str.insert_str(self.start_index, &self.content);
    }
}

