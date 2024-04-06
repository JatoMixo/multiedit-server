mod setup;
use multiedit_server::file_tracking::{
    FileTracker,
    FileChange,
    Insertion,
    Deletion,
};

#[cfg(test)]
mod test_file_tracking {
    use std::io::Read;

    use super::*;

    #[test]
    fn simple_file_tracking() {
        let testing_environment = setup::TestingEnvironment::create();

        let mut file_tracker_0 = FileTracker::new(testing_environment.example_files_at_root[0].try_clone().unwrap());
        file_tracker_0.apply_change(FileChange::new(
            socketioxide::socket::Sid::new(),

            vec![Deletion{ start_index: 0, end_index: 5 }],
            vec![Insertion{ start_index: 5, content_inserted: "potato".to_string()}],
        ));

        let mut new_file_content = String::new();
        let _ = testing_environment.example_files_at_root[0].try_clone().unwrap().read_to_string(&mut new_file_content);

        /*assert_eq!(new_file_content,
                   "");*/
    }
}
