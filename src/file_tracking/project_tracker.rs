use crate::file_tracking::{Path, FileTracker, FileTrackingError, FileChange};
use std::{collections::HashMap, fs::read_dir, path::PathBuf, sync::RwLock};

/// Tracks all the files inside a directory, including files nested in other directories
#[derive(Debug)]
pub struct ProjectTracker {
    root: Path,
    // TODO: Optimize with hashmap?
    files: RwLock<Vec<FileTracker>>,
}

impl ProjectTracker {
    pub fn new(root: Path) -> Result<ProjectTracker, FileTrackingError> {

        let mut files = Vec::new();

        for file_path in get_file_tree(&root)?.iter() {
            files.push(
                FileTracker::new(file_path.clone())?
            );
        }

        Ok(ProjectTracker {
            root,
            files: RwLock::new(files),
        })
    }

    pub fn apply_change_to_file(&self, target_file: &PathBuf, change: &FileChange) -> Result<(), FileTrackingError> {
        for file_tracker in self.files.write().unwrap().iter_mut() {
            // WARN: Maybe this way of checking paths will cause errors for stupid paths that
            // are the same, but aren't represented the same way with something like a string
            if &file_tracker.get_path().local_path == target_file {
                file_tracker.apply_change(change.clone())?;
            }
        }

        Ok(())
    }

    pub fn get_file_content_tree(&self) -> Result<HashMap<String, String>, FileTrackingError> {

        let mut file_content_tree: HashMap<String, String> = HashMap::new();

        let insertion = self
            .files
            .read()
            .unwrap()
            .iter()
            .try_for_each(|file_tracker| -> Result<(), FileTrackingError> {
                file_content_tree.insert(
                    file_tracker.get_path().clone().local_path.to_str().unwrap().to_string(),
                    file_tracker.get_current_file_content()?
                );

                Ok(())
            });

        match insertion {
            Ok(()) => Ok(file_content_tree),
            Err(err) => Err(err),
        }
    }
}

fn get_file_tree(root: &Path) -> Result<Vec<Path>, FileTrackingError> {
    match read_dir(root.get_absolute_path()) {
        Ok(file_elements) => {
            
            let mut file_tree: Vec<Path> = Vec::new();

            let file_elements = file_elements.map(|element| element.unwrap());

            for element in file_elements {
                if element.metadata().unwrap().is_file() {
                    file_tree.push(
                        root.pushed_to_local_path(element.file_name().into())
                    );

                    continue;
                }

                let mut nested_elements = get_file_tree(&root.pushed_to_local_path(element.file_name().into()))?;
                file_tree.append(&mut nested_elements);
            }

            Ok(file_tree)
        },
        Err(_) => Err(FileTrackingError::CantOpenElement),
    }
}

