use std::path::PathBuf;

/// A Path for a file hosted in the server, this is used to prevent using PathBuf's across the
/// application, which could create accidentally a Local File Inclusion. With this only the local
/// path to the file is available.
/// It takes the root to the directory where the server was launched and the local path, relative
/// to the server.
#[derive(Debug, serde::Serialize)]
pub struct Path {
    root_path: PathBuf,
    pub local_path: PathBuf,
}

impl Path {
    pub fn new(root_path: PathBuf, local_path: PathBuf) -> Path {
        Path {
            local_path,
            root_path,
        }
    }

    /// Get the absolute path to the file/directory in the disk, better for accessing it using the
    /// filesystem, this way only the local path of the server will be sent to
    /// the client, and nobody will be in risk of getting hacked :)
    pub fn get_absolute_path(&self) -> PathBuf {
        let mut absolute_path = self.root_path.clone();
        absolute_path.push(&self.local_path);

        absolute_path
    }
    
    pub fn pushed_to_local_path(&self, new_part: PathBuf) -> Path {
        let mut new_local_path = self.local_path.clone();
        new_local_path.push(new_part);

        Path::new(self.root_path.clone(), new_local_path)
    }
}
