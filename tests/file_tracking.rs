mod setup;
use setup::setup_testing_environment;

use multiedit_server::file_tracking::{Path, FileTracker, FileChange};
use std::path::PathBuf;
use std::io::Read;

use indoc::indoc;

#[cfg(test)]
mod test_file_tracking {
    use super::*;

    #[test]
    fn test_simple_change_on_single_file() {
        setup_testing_environment();

        let mut file = std::fs::File::open("tests/test-directory/python/example-1.py").unwrap();

        FileTracker::new(Path::new(PathBuf::from("tests/test-directory"), PathBuf::from("python/example-1.py")))
            .unwrap()
            .apply_change(FileChange::new(
                15, 26,
                String::from("some terrible")
            ))
            .unwrap();

        let mut file_content = String::new();
        file.read_to_string(&mut file_content).unwrap();

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
