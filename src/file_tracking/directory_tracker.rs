use std::path::PathBuf;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct DirectoryTracker {
    // WARN: This is using PathBuf, which might leak data about the files in the host computer (A
    // local file inclusion)
    // TODO: Change this to a custom LocalPath struct or something like that to prevent this from
    // happening
    root: PathBuf,
    // String for now, just to test and send something back to the client
    files: Vec<String>,
}

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
