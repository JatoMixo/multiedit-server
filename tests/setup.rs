use std::{
    collections::HashMap,
    path::PathBuf,
    fs::{create_dir_all, File},
    io::Write,
};
use indoc::indoc;

pub struct TestingEnvironment {
    // The directory where the testing is gonna be done
    pub root: PathBuf,

    // Directory => Files it contains (Can be nested)
    pub files_per_directory: HashMap<PathBuf, Vec<PathBuf>>,
}

impl TestingEnvironment {
    pub fn take() -> TestingEnvironment {
        let root = PathBuf::from("tests/test-directory");

        create_dir_all(&root).expect("Failed when creating the testing directory wtf");

        let files_per_directory: HashMap<PathBuf, Vec<PathBuf>> = HashMap::from([
            (root.join("python/"), vec![root.join("python/example-1.py"),
                                        root.join("python/example-2.py"),]),
            (root.join("lua/"), vec![root.join("lua/example-1.lua"),
                                     root.join("lua/testing.lua"),]),
        ]);

        files_per_directory.iter().for_each(|(directory, file_groups)| {

            create_dir_all(directory).expect("This surely is impossible to error, right?");

            file_groups.iter().for_each(|file_path| {

                let mut file = File::create(file_path).expect("Couldn't create the file duh");

                let path_for_file_0 = root.join("python/example-1.py");
                let path_for_file_1 = root.join("python/example-2.py");
                let path_for_file_2 = root.join("lua/example-1.lua");
                let path_for_file_3 = root.join("lua/testing.lua");

                match file_path {
                    path_for_file_0 => {
                        file.write_all(content_for_file_0().as_bytes()).expect("Failed when creating the file wtf");
                    },
                    path_for_file_1 => {
                        file.write_all(content_for_file_1().as_bytes()).expect("Failed when creating the file wtf");
                    },
                    path_for_file_2 => {
                        file.write_all(content_for_file_2().as_bytes()).expect("Failed when creating the file wtf");
                    },
                    path_for_file_3 => {
                        file.write_all(content_for_file_3().as_bytes()).expect("Failed when creating the file wtf");
                    },
                    _ => unreachable!(),
                }
            });
        });

        TestingEnvironment {
            root,
            files_per_directory,
        }
    }
}

fn content_for_file_0() -> String {
    indoc! {r#"
        print("This is some proper python script")

        a = 5
        a += 2

        for number in range(0, a):
            print("Number: " + str(a))

        # idk what i'm doing at this point, i hate making tests...
        return -1
    "#}.to_string()
}

fn content_for_file_1() -> String {
    indoc! {r#"
        # Python? yeah, idk why I chose it
        print("fast to write")

        so_its_good_for_this_sort_of_tests = True

        potato = "god i still have to make 2 more file examples"

        print(potato)

        # writing brainfuck would be more interesting than this
    "#}.to_string()
}

fn content_for_file_2() -> String {
    indoc! {r#"
        print("yay! lua")

        local b = 69420

        print(b)

        -- i honestly don't know how to write lua

        print("the only lua thing that i've written is my nvim config (95% stolen from other people)")
    "#}.to_string()
}

fn content_for_file_3() -> String {
    indoc! {r#"
        print("god, this is finally getting over")

        -- just this file and i'm done

        a = 234635463456
        print(a / 2)

        print("and now (when i finished these examples) is when i realize i could've asked chatgpt for the content of these examples...")
        -- ^^ this hurted
    "#}.to_string()
}

