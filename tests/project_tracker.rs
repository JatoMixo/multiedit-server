mod setup;
use setup::setup_testing_environment;
use multiedit_server::file_tracking::{
    ProjectTracker,
    Path,
};
use std::path::PathBuf;
use std::io::Read;
use indoc::indoc;

#[cfg(test)]
mod project_tracker_testing {
    use multiedit_server::file_tracking::FileChange;

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
                Path::new(
                    PathBuf::from("tests/test-directory"),
                    PathBuf::from("python/example-2.py")
                ),
                Path::new(
                    PathBuf::from("tests/test-directory"),
                    PathBuf::from("python/example-1.py")
                ),
                Path::new(
                    PathBuf::from("tests/test-directory"),
                    PathBuf::from("lua/testing.lua")
                ),
                Path::new(
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

        let file_changed = PathBuf::from("python/example-1.py");

        let change = FileChange::new(
            15, 26,
            String::from("some terrible")
        );

        project_tracker.apply_change_to_file(&file_changed, &change).unwrap();

        let mut file_content = String::new();
        std::fs::File::open("tests/test-directory/python/example-1.py").unwrap().read_to_string(&mut file_content).unwrap();

        assert_eq!(indoc! {r#"
            print("This is some terrible python script")

            a = 5
            a += 2

            for number in range(0, a):
                print("Number: " + str(a))

            # idk what i'm doing at this point, i hate making tests...
            return -1
        "#}, file_content);
    }
}

