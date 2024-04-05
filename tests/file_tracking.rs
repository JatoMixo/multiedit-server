mod setup;
use multiedit_server::file_tracking::FileTracker;

#[cfg(test)]
mod test_file_tracking {
    use super::*;

    #[test]
    fn simple_file_tracking() {
        let testing_environment = setup::TestingEnvironment::create();

        let file_tracker_0 = FileTracker::new(testing_environment.example_files_at_root[0].try_clone().unwrap());
    }
}
