use std::{
    fs::{
        File,
        create_dir,
    },
    io::Write,
};

pub struct TestingEnvironment {
    // TODO: Maybe this should be changed to a PathBuf
    pub directory_name: String,
    pub example_files_at_root: Vec<File>,
}

impl TestingEnvironment {
    pub fn create() -> TestingEnvironment {

        let directory_name = String::from("tests/test-directory");
        let _ = create_dir(&directory_name);

        let mut example_file_0 = File::create(format!("{}/example1.py", directory_name)).unwrap();
        example_file_0.write("print('Hello')
                             a = 5
                             print(a)
                             

                             # this test sucks".as_bytes()).unwrap();

        let mut example_file_1 = File::create(format!("{}/example2.lua", directory_name)).unwrap();
        example_file_1.write("print('Hello')
                             local a = 5
                             print(a)
                             

                             -- this test doesn't suck".as_bytes()).unwrap();

        let example_files_at_root = [
            example_file_0,
            example_file_1,
        ].try_into().unwrap();

        TestingEnvironment {
            directory_name,
            example_files_at_root,
        }
    }
}

