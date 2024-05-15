/*mod setup;
use setup::setup_testing_environment;

use multiedit_server::file_tracking::{DirectoryTracker, FileChange, Path};

use std::path::PathBuf;
use std::io::Read;

use indoc::indoc;

#[cfg(test)]
mod directory_tracking_tests {
    use super::*;

    #[test]
    fn basic_directory_tracking_testing() {
        setup_testing_environment();

        let mut directory_tracker = DirectoryTracker::new(
            Path::new(PathBuf::from("tests/test-directory"), PathBuf::from("python/"))
        ).unwrap();

        // ====== FIRST FILE ======
        directory_tracker.apply_change_to_file(
            Path::new(PathBuf::from("tests/test-directory"), PathBuf::from("python/example-1.py")),
            FileChange::new(
                socketioxide::socket::Sid::new(),
                15, 26,
                String::from("some terrible")
            )
        ).unwrap();

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

        // ====== SECOND FILE ======
        directory_tracker.apply_change_to_file(
            Path::new(PathBuf::from("tests/test-directory"), PathBuf::from("python/example-2.py")),
            FileChange::new(
                socketioxide::socket::Sid::new(),
                59, 158,
                String::from("#fuck this")
            )
        ).unwrap();

        let mut file_content = String::new();
        std::fs::File::open("tests/test-directory/python/example-2.py").unwrap().read_to_string(&mut file_content).unwrap();

        assert_eq!(indoc! {r#"
            # Python? yeah, idk why I chose it
            print("fast to write")

            #fuck this

            print(potato)

            # writing brainfuck would be more interesting than this
        "#}, file_content);
    }
}*/
