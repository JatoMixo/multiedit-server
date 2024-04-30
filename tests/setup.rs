use std::{
    path::PathBuf,
    fs::{create_dir_all, File},
    io::Write,
};
use indoc::indoc;

pub fn setup_testing_environment() {
    let root = PathBuf::from("tests/test-directory");

    create_dir_all(&root).expect("Failed when creating the testing directory wtf");

    let directories: Vec<PathBuf> = vec![
        root.join("python/"),
        root.join("lua/"),
    ];

    directories.iter().for_each(|directory| {
        create_dir_all(directory).expect("This surely is impossible to error, right?");
    });

    File::create(root.join("python/example-1.py"))
        .expect("This is starting to get a bit annoying...")
        .write_all(content_for_python_example_0().as_bytes())
        .expect("Couldn't write content to file");

    File::create(root.join("python/example-2.py"))
        .expect("This is starting to get a bit annoying...")
        .write_all(content_for_python_example_1().as_bytes())
        .expect("Couldn't write content to file");

    File::create(root.join("lua/example-1.lua"))
        .expect("This is starting to get a bit annoying...")
        .write_all(content_for_lua_example_0().as_bytes())
        .expect("Couldn't write content to file");

    File::create(root.join("lua/testing.lua"))
        .expect("This is starting to get a bit annoying...")
        .write_all(content_for_lua_example_testing().as_bytes())
        .expect("Couldn't write content to file");

}

fn content_for_python_example_0() -> String {
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

fn content_for_python_example_1() -> String {
    indoc! {r#"
        # Python? yeah, idk why I chose it
        print("fast to write")

        so_its_good_for_this_sort_of_tests = True

        potato = "god i still have to make 2 more file examples"

        print(potato)

        # writing brainfuck would be more interesting than this
    "#}.to_string()
}

fn content_for_lua_example_0() -> String {
    indoc! {r#"
        print("yay! lua")

        local b = 69420

        print(b)

        -- i honestly don't know how to write lua

        print("the only lua thing that i've written is my nvim config (95% stolen from other people)")
    "#}.to_string()
}

fn content_for_lua_example_testing() -> String {
    indoc! {r#"
        print("god, this is finally getting over")

        -- just this file and i'm done

        a = 234635463456
        print(a / 2)

        print("and now (when i finished these examples) is when i realize i could've asked chatgpt for the content of these examples...")
        -- ^^ this hurted
    "#}.to_string()
}

