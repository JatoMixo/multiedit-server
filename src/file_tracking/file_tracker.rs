use std::{
    collections::VecDeque,
    fs::File,
    io::{Read, Write},
};
use socketioxide::socket::Sid;
use crate::file_tracking::Path;

#[derive(Debug)]
pub struct FileTracker {
    file_path: Path,
    changes_history: VecDeque<FileChange>,
}

impl FileTracker {
    pub fn new(file_path: Path) -> Result<FileTracker, FileTrackerError> {

        let file = File::open(&file_path.get_absolute_path());

        match file {
            Ok(_) => Ok(FileTracker {
                file_path,
                changes_history: VecDeque::new(),
            }),
            Err(_) => Err(FileTrackerError::CantOpenFile),
        }

    }

    fn open_file_for_reading(&self) -> Result<File, FileTrackerError> {
        match File::open(&self.file_path.get_absolute_path()) {
            Ok(file) => Ok(file),
            Err(_) => Err(FileTrackerError::CantOpenFile),
        }
    }

    /// WARN: This will create the file if it doesn't exist
    /// or it'll truncate it if it does, this is because it's using
    /// File::create to open it
    fn open_file_for_writing(&self) -> Result<File, FileTrackerError> {
        match File::create(&self.file_path.get_absolute_path()) {
            Ok(file) => Ok(file),
            Err(_) => Err(FileTrackerError::CantOpenFile),
        }
    }

    fn get_current_file_content(&self) -> Result<String, FileTrackerError> {
        let mut file = self.open_file_for_reading()?;

        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Ok(_) => Ok(content),
            Err(_) => Err(FileTrackerError::CantReadContentOfFile),
        }
    }

    fn write_content_to_file(&mut self, content: String) -> Result<(), FileTrackerError> {
        let mut file = self.open_file_for_writing()?;

        match file.write_all(content.as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => Err(FileTrackerError::CantWriteContentToFile),
        }
    }

    pub fn apply_change(&mut self, change: FileChange) -> Result<(), FileTrackerError> {
        
        // TODO: This can surely be optimized with some sort of cache stuff to prevent getting the
        // file content over and over whenever a change is applied.
        let mut file_content = self.get_current_file_content()?;

        change.apply_to(&mut file_content);

        self.write_content_to_file(file_content)?;
        self.changes_history.push_back(change);

        Ok(())
    }
}

#[derive(Debug)]
pub enum FileTrackerError {
    CantOpenFile,
    CantReadContentOfFile,
    CantWriteContentToFile,
}

// TODO: This way of storing the changes is a bit weird to use with the Myers diff algorithm, it
// works, but a single change that has a certain part that hasn't changed will count as 2, for
// example: AAABBB -> CCABCC Will require 2 FileChange's to get stored, when it was just a single
// changed with a spot in the middle that stays the same. Maybe create a different struct like
// LineChange that takes care of this part and store a vector of LineChange's in the FileChange

/// A Change applied to a file. When applying the change, it deletes the content in
/// between the start index and the end index of it, and then inserts the new content
/// there as a sort of replacement. It also contains the ID of the author.
#[derive(Debug)]
pub struct FileChange {
    author_id: Sid,

    start_index: usize,
    end_index: usize,
    // TODO: Change this to some sort of [u8]
    content: String,
}

impl FileChange {
    /// Create a new FileChange
    pub fn new(
        author_id: Sid,
        start_index: usize,
        end_index: usize,
        content: String,
    ) -> FileChange {
        FileChange {
            author_id,
            start_index,
            end_index,
            content,
        }
        
    }

    /// Create a new file change that deletes a range of characters (Replaces that range in the
    /// content by an empty one)
    pub fn create_deletion(
        author_id: Sid,
        start_index: usize,
        end_index: usize
    ) -> FileChange {
        FileChange {
            author_id,
            start_index,
            end_index,
            content: String::new(),
        }
    }

    pub fn apply_to(&self, str: &mut String) {
        str.drain(self.start_index..self.end_index);

        str.insert_str(self.start_index, &self.content);
    }

    pub fn get_author_id(&self) -> Sid {
        self.author_id
    }
}

