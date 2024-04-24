mod setup;
use multiedit_server::file_tracking::Path;
use std::path::PathBuf;

#[cfg(test)]
mod path_usage {
    use super::*;

    #[test]
    fn get_absolute_path() {
        let mut a = Path::new(
            PathBuf::from("tests/test-directory"),
            PathBuf::from("example-1.py"),
        );

        assert_eq!(PathBuf::from("tests/test-directory/example-1.py"), a.get_absolute_path());
        assert_eq!(a.local_path, PathBuf::from("example-1.py"));

        a = Path::new(
            PathBuf::from("tests"),
            PathBuf::from("test-directory/example-1.py"),
        );

        assert_eq!(PathBuf::from("tests/test-directory/example-1.py"), a.get_absolute_path());

        a = Path::new(
            PathBuf::from(""),
            PathBuf::from("tests/test-directory/example-1.py"),
        );

        assert_eq!(PathBuf::from("tests/test-directory/example-1.py"), a.get_absolute_path());
    }

    #[test]
    fn pushed_to_local_path() {
        assert_eq!(
            Path::new(PathBuf::from("a/b/c"), PathBuf::from("potato/asdasd")).get_absolute_path(),
            Path::new(PathBuf::from("a/b/c"), PathBuf::from("potato")).pushed_to_local_path(PathBuf::from("asdasd")).get_absolute_path()
        );

        assert_eq!(
            Path::new(PathBuf::from(""), PathBuf::from("potato/thisisweird/hello")).get_absolute_path(),
            Path::new(PathBuf::from(""), PathBuf::from("potato")).pushed_to_local_path(PathBuf::from("thisisweird/hello")).get_absolute_path()
        );

        assert_eq!(
            Path::new(PathBuf::from("making/tests/is/boring/me/rn"), PathBuf::from("send/help/pls/asdasd")).get_absolute_path(),
            Path::new(PathBuf::from("making/tests/is/boring/me/rn"), PathBuf::from("send/help/pls")).pushed_to_local_path(PathBuf::from("asdasd")).get_absolute_path()
        );
    }
}
