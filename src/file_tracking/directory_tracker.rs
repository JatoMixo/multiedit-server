use std::fs::DirEntry;
use crate::file_tracking::{
    Path,
    FileTracker,
};

/// Tracks the changes applied to all the files in a directory, and also tracks the changes inside
/// of other directories within it recursively.
#[derive(Debug, serde::Serialize)]
pub struct DirectoryTracker {
    /// The location of the tracked directory
    root: Path,
    /// The file trackers for each file inside the tracked directory
    files: Vec<FileTracker>,
    /// The directory trackers for each nested directory
    directories: Vec<DirectoryTracker>,
}

#[derive(Debug)]
pub enum DirectoryTrackerError {
    RequestedDirectoryDoesntExist,
}

impl DirectoryTracker {
    /// Create a new directory tracker given the path of the directory to track
    pub fn new(root: Path) -> Result<DirectoryTracker, DirectoryTrackerError> {
        match std::fs::read_dir(&root.get_absolute_path()) {
            Ok(files_direntries) => {

                let files = files_direntries
                    .map(|direntry_result| {
                        direntry_result.unwrap()
                    })
                    .filter(|direntry| {
                        direntry.metadata().unwrap().is_file()
                    })
                    .map(|direntry| {
                        FileTracker::new(
                            root.pushed_to_local_path(direntry.path())
                        ).unwrap()
                    })
                    .collect::<Vec<FileTracker>>();

                Ok(
                    DirectoryTracker {
                        root,
                        files,
                        directories: Vec::new(),
                    }
                )
            },
            Err(_) => Err(DirectoryTrackerError::RequestedDirectoryDoesntExist),
        }
    }
}
