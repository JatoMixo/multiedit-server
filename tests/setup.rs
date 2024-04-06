use std::{
    fs::{
        File,
        create_dir,
    },
    path::PathBuf,
    io::Write,
};

pub struct TestingEnvironment {
    // TODO: Maybe this should be changed to a PathBuf
    pub directory_path: PathBuf,
    pub example_files_at_root: Vec<String>,
}

impl TestingEnvironment {
    pub fn create() -> TestingEnvironment {

        const DIRECTORY_NAME: &str = "tests/test-directory";
        let _ = create_dir(&DIRECTORY_NAME);

        // TODO: This way of creating the files adds weird tabs in the beginning of the 
        // new lines; find a way to prevent that from happening.
        let example_file_0_path = format!("{}/example1.py", DIRECTORY_NAME);
        let mut example_file_0 = File::create(&example_file_0_path).unwrap();
        example_file_0.write("print('Hello')
                             a = 5
                             print(a)
                             

                             # this test sucks".as_bytes()).unwrap();

        let example_file_1_path = format!("{}/example2.lua", DIRECTORY_NAME);
        let mut example_file_1 = File::create(&example_file_1_path).unwrap();
        example_file_1.write("print('Hello')
                             local a = 5
                             print(a)
                             

                             -- this test doesn't suck".as_bytes()).unwrap();

        let example_files_at_root = [
            example_file_0_path,
            example_file_1_path,
        ].try_into().unwrap();

        TestingEnvironment {
            directory_path: PathBuf::from(DIRECTORY_NAME),
            example_files_at_root,
        }
    }
}

