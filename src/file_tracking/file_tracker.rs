use std::{
    collections::VecDeque, fs::File, io::{Read, Write},
};
use crate::user_management::User;

#[derive(Debug)]
pub struct FileTracker {
    file: File,
    changes_history: VecDeque<FileChange>,
}

impl FileTracker {
    pub fn new(file: File) -> FileTracker {
        FileTracker {
            file,
            changes_history: VecDeque::new(),
        }
    }

    pub fn apply_change(&mut self, change: FileChange) {

        // TODO: This can surely be optimized with some sort of cache stuff  to prevent getting the
        // file content over and over whenever a change is applied.
        let mut file_content = String::new();
        self.file.read_to_string(&mut file_content).unwrap();

        change.deletions.iter().for_each(|deletion| {
            file_content.drain(deletion.start_index..deletion.end_index); 
        });

        change.insertions.iter().for_each(|insertion| {
            file_content.insert_str(insertion.start_index, &insertion.content_inserted);
        });

        let _ = self.file.write_all(file_content.as_bytes());

        self.changes_history.push_back(change);
    }
}

#[derive(Debug)]
pub struct FileChange {
    author_id: socketioxide::socket::Sid,

    pub deletions: Vec<Deletion>,
    pub insertions: Vec<Insertion>,
}

#[derive(Debug)]
struct Insertion {
    pub start_index: usize,
    // TODO: Maybe change from string to [u8]??? Works better for files I think... 
    pub content_inserted: String,
}

#[derive(Debug)]
struct Deletion {
    start_index: usize,
    end_index: usize,
}

