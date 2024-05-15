mod setup;
use setup::setup_testing_environment;

use multiedit_server::file_tracking::{
    ProjectTracker,
    Path,
};
use std::path::PathBuf;

#[cfg(test)]
mod project_tracker_testing {
    use super::*;

    #[test]
    fn test_spawn_project_tracker() {

        setup_testing_environment();

        let project_tracker = ProjectTracker::new(Path::new(
            std::path::PathBuf::from("tests/test-directory"),
            std::path::PathBuf::new(),
        ))
        .unwrap();

        assert_eq!(
            vec![
                &Path::new(
                    PathBuf::from("tests/test-directory"),
                    PathBuf::from("python/example-1.py")
                ),
                &Path::new(
                    PathBuf::from("tests/test-directory"),
                    PathBuf::from("python/example-2.py")
                ),
                &Path::new(
                    PathBuf::from("tests/test-directory"),
                    PathBuf::from("lua/testing.lua")
                ),
                &Path::new(
                    PathBuf::from("tests/test-directory"),
                    PathBuf::from("lua/example-1.lua")
                ),
            ],
            project_tracker.get_file_paths()
        );
    }

    #[test]
    fn test_change_applied_to_project() {

        setup_testing_environment();

        let project_tracker = ProjectTracker::new(Path::new(
            std::path::PathBuf::from("tests/test-directory"),
            std::path::PathBuf::new(),
        ))
        .unwrap();


    }
}

