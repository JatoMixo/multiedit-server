use std::{
    collections::VecDeque,
    fs::File,
    io::{Read, Write},
    path::PathBuf
};
use socketioxide::socket::Sid;

#[derive(Debug)]
pub struct FileTracker {
    // TODO: PathBuf problems again
    file_path: PathBuf,
    changes_history: VecDeque<FileChange>,
}

impl FileTracker {
    pub fn new(file_path: PathBuf) -> Result<FileTracker, FileTrackerError> {

        let file = File::open(&file_path);

        match file {
            Ok(_) => Ok(FileTracker {
                file_path,
                changes_history: VecDeque::new(),
            }),
            Err(_) => Err(FileTrackerError::CantOpenFile),
        }

    }

    fn open_file(&self) -> Result<File, FileTrackerError> {
        match File::open(&self.file_path) {
            Ok(file) => Ok(file),
            Err(_) => Err(FileTrackerError::CantOpenFile),
        }
    }

    fn get_current_file_content(&self) -> Result<String, FileTrackerError> {
        let mut file = self.open_file()?;

        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Ok(_) => Ok(content),
            Err(_) => Err(FileTrackerError::CantReadContentOfFile),
        }
    }

    fn write_content_to_file(&mut self, content: String) -> Result<(), FileTrackerError> {
        let mut file = self.open_file()?;

        match file.write_all(content.as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => Err(FileTrackerError::CantWriteContentToFile),
        }
    }

    pub fn apply_change(&mut self, change: FileChange) -> Result<(), FileTrackerError> {

        // TODO: This can surely be optimized with some sort of cache stuff  to prevent getting the
        // file content over and over whenever a change is applied.
        let mut file_content = self.get_current_file_content()?;

        change.deletions.iter().for_each(|deletion| {
            file_content.drain(deletion.start_index..deletion.end_index); 
        });

        change.insertions.iter().for_each(|insertion| {
            file_content.insert_str(insertion.start_index, &insertion.content_inserted);
        });

        self.write_content_to_file(file_content)?;
        self.changes_history.push_back(change);

        Ok(())
    }
}

pub enum FileTrackerError {
    CantOpenFile,
    CantReadContentOfFile,
    CantWriteContentToFile,
}

#[derive(Debug)]
pub struct FileChange {
    author_id: Sid,

    pub deletions: Vec<Deletion>,
    pub insertions: Vec<Insertion>,
}

impl FileChange {
    pub fn new(author_id: Sid, deletions: Vec<Deletion>, insertions: Vec<Insertion>) -> FileChange {
        FileChange {
            author_id,
            insertions,
            deletions,
        }
    }

    pub fn get_author_id(&self) -> Sid {
        self.author_id
    }
}

#[derive(Debug)]
struct Insertion {
    pub start_index: usize,
    // TODO: Maybe change from string to [u8]??? Works better for files I think... 
    pub content_inserted: String,
}

#[derive(Debug)]
struct Deletion {
    pub start_index: usize,
    pub end_index: usize,
}

