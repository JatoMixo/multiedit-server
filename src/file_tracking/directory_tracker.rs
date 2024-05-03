use std::collections::HashMap;
use crate::file_tracking::{
    Path,
    FileTracker,
    FileTrackingError,
    FileChange,
};

#[derive(Debug, serde::Serialize)]
pub struct DirectoryTracker {
    root: Path,
    files: HashMap<Path, FileTracker>,
}

impl DirectoryTracker {
    pub fn new(root: Path) -> Result<DirectoryTracker, FileTrackingError> {
        match std::fs::read_dir(&root.get_absolute_path()) {
            Ok(files_direntries) => {

                let mut files = HashMap::new();

                for file_direntry in files_direntries {
                    let file_direntry = file_direntry.unwrap();

                    if !file_direntry.metadata().unwrap().is_file() {
                        continue;
                    }

                    files.insert(
                        root.pushed_to_local_path(file_direntry.file_name().into()),
                        FileTracker::new(
                            root.pushed_to_local_path(file_direntry.file_name().into())
                        )?,
                    );
                }

                Ok(
                    DirectoryTracker {
                        root,
                        files,
                    }
                )
            },
            Err(_) => Err(FileTrackingError::CantOpenElement),
        }
    }

    pub fn apply_change_to_file(&mut self, file_path: Path, change: FileChange) -> Result<(), FileTrackingError> {

        if let Some(file_tracker) = self.files.get_mut(&file_path) {
            file_tracker.apply_change(change)?;
        }

        Ok(())
    }
}
