use crate::file_tracking::{
    Path,
    FileTracker,
    FileTrackingError
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

impl DirectoryTracker {
    /// Create a new directory tracker given the path of the directory to track
    pub fn new(root: Path) -> Result<DirectoryTracker, FileTrackingError> {
        match std::fs::read_dir(&root.get_absolute_path()) {
            Ok(files_direntries) => {

                let mut files = Vec::new();
                let mut directories = Vec::new();

                for file_direntry in files_direntries {
                    let file_direntry = file_direntry.unwrap();

                    if file_direntry.metadata().unwrap().is_file() {
                        files.push(FileTracker::new(
                            root.pushed_to_local_path(file_direntry.path())
                        )?);
                    }

                    if file_direntry.metadata().unwrap().is_dir() {
                        directories.push(DirectoryTracker::new(
                            root.pushed_to_local_path(file_direntry.path())
                        )?);
                    }
                }

                Ok(
                    DirectoryTracker {
                        root,
                        files,
                        directories,
                    }
                )
            },
            Err(_) => Err(FileTrackingError::CantOpenElement),
        }
    }
}
