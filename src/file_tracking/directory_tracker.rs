use std::path::PathBuf;

#[derive(Debug, serde::Deserialize)]
pub struct DirectoryTracker {
    root: PathBuf,
    // String for now, just to test and send something back to the client
    files: Vec<String>,
}

// TODO: Move this to a different file
#[derive(Debug)]
pub enum DirectoryTrackerError {
    RequestedDirectoryDoesntExist,
}

impl DirectoryTracker {
    pub fn new(root: PathBuf) -> Result<DirectoryTracker, DirectoryTrackerError> {
        match std::fs::read_dir(&root) {
            Ok(files_direntries) => {
                
                let file_names = files_direntries.map(|file_direntry| {
                    file_direntry.unwrap().file_name().to_str().unwrap().to_string()
                }).collect::<Vec<String>>();

                Ok(
                    DirectoryTracker {
                        root,
                        files: file_names,
                    }
                )
            },
            Err(_) => Err(DirectoryTrackerError::RequestedDirectoryDoesntExist),
        }
    }
}
