#![allow(unused_must_use)]

use regex::Regex;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;
use std::borrow::Cow;
use std::fs::File;

pub mod features;

const SPLIT_STR: &str = r#"//======="#;
const FEATURE_STR: &str = r#"#[cfg(feature = "#;
const FEATURE_MODE: &str = r#"#[cfg(feature = "_______")]"#;

const END: &str = "\"#;";
const FN_MAIN: &str = "adjoin";

pub fn hello() {
    println!("{}\n", "Start... Hello Borrowing!");
    println!("{}", "use source codes:");
    println!("\t\t{}", "cargo run --bin bw -- -h");
    println!("\t\tcargo run --example <RUST_FILE_NAME> --features ok");
    println!("\t\tcargo run --example <RUST_FILE_NAME> --features err");
    println!("\texample:");
    println!("\t\tcargo run --example kw_let --features ok");
    println!("\n{}", "use the crate `borrowing_exerci`:");
    println!("\t\t{}", "bw -h");
    println!("\t\t{}", "bw --help");
    println!("\t\t{}", "bw --file <RUST_FILE_NAME> --mode ok");
    println!("\t\t{}", "bw --file <RUST_FILE_NAME> --mode err");
    println!("\texample:");
    println!("\t\t{}", "bw --file kw_let --mode err | bat -l rs ");
    println!("\t\t{}", "tip: f");
    println!("\t\t{}", "tip: q");
}


fn get_begin<'de>(file_name: &'de str, feature_name: &'de str) -> Cow<'de, str>{
    const BEGIN: &str = r#"pub const ------- :&str = r#""#;
    let re_feature = Regex::new(r"-------").unwrap();
    let replace_str = format!(
        "{}_{}",
        file_name.to_uppercase(),
        feature_name.to_uppercase()
    );
    let begin_str = re_feature.replace(
        BEGIN, replace_str.as_str()
    );
    begin_str
}

pub fn convert_rs(folder_name: &str, file_name: &str) {
    //remove_rs(file_name);
    // Create a path to the desired file

    let source = read_rs(folder_name, file_name, false);
    let re_feature = Regex::new(r"_______").unwrap();

    let mut destination = String::new();
    let re_comment = Regex::new(r"^\s*//").unwrap();
    let re_main = Regex::new(r"adjoin").unwrap();


    // https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust
    let mut split: Vec<&str> = source.split(SPLIT_STR).collect();
    if split.is_empty() {
        unimplemented!();
    } else {
        if split[0].is_empty() {
            unimplemented!();
        } else {
            let pre = split[1];
            if split[1].is_empty() {
                unimplemented!();
            } else {
                if split[2].is_empty() {
                    unimplemented!();
                } else {
                    for item in split.iter_mut() {
                        let result = re_feature.replace(FEATURE_MODE, "ok");
                        if item.contains(result.as_ref()) {
                            print!("{}", item);
                            destination = format!("{}{}", pre, item);
                            break;
                        }
                    }
                }
            }
        }
    }

    // remove the comments
    //dbg!(&destination);
    let lines = destination.lines();
    let mut codes_str = String::new();
    let mut result;
    for line in lines {
        if line.contains(FN_MAIN) {
            result = re_main.replace(line, "main");
            codes_str = format!("{}{}\n", codes_str, result.as_ref());
        } else if line.contains(FEATURE_STR) {
            // 
        } else {
            if !line.is_empty() {
                result = re_comment.replace(line, "");
                if line == result {                
                    codes_str = format!("{}{}\n", codes_str, result.as_ref());
                }
            }
        }
        //dbg!(&result);
    }

    let begin_str = get_begin(file_name, "ok");
    codes_str = format!("{}{}{}", begin_str, codes_str, END);
    write_rs(file_name, codes_str);

    //show_rs(file_name);
}



// https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html
// https://doc.rust-lang.org/rust-by-example/std_misc/file/create.html
fn read_rs(folder_name: &str, file_name: &str, visible: bool) -> String {
    let file = format!("./examples/{}/{}.rs", folder_name, file_name);
    let path = Path::new(&file);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => {
            if visible {
                print!("\n\n\n{} contains:\n{}\n\n\n", display, s);
            } else {
                print!("successfully read to\n{}\n\n", display);
            }
        }
    }
    s
}

fn write_rs(file_name: &str, stream: String) {
    // Create a path to the desired file
    let file = format!("./src/features/rs_files/{}.rs", file_name);
    let new_path = Path::new(&file);
    let new_display = new_path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut new_file = match File::create(&new_path) {
        Err(why) => panic!("couldn't create {}: {}", new_display, why.description()),
        Ok(new_file) => new_file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match new_file.write_all(stream.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", new_display, why.description()),
        Ok(_) => println!("successfully wrote to\n{}\n\n", new_display),
    }
}

/*
fn show_rs(file_name: &str) {
    let file = format!("./src/features/rs_files/{}.rs", file_name);
    let new_path = Path::new(&file);
    let new_display = new_path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut new_file = match File::open(&new_path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", new_display,
                                                   why.description()),
        Ok(new_file) => new_file,
    };

    let mut s = String::new();
    match new_file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", new_display,
                                                   why.description()),
        Ok(_) => print!("{} contains:\n{}\n\n\n", new_display, s),
    }
}
*/

pub fn allx_cmds(file_name: &str) {
    let ret = format!(
        r#"
# Show this command
cargo run --bin bw -- --file {0} | bat -l cmd
cargo install borrowing_exerci
bw --file {0}
bw --file {0} | bat -l cmd

# Show the Help for Rust File
cargo run --bin bw -- --file <FILENAME>
cargo install borrowing_exerci
bw --file <FILENAME> | bat -l cmd

# Run OK:
cargo run --bin bw -- --file <FILENAME> --mode ok
cargo run --example <FILENAME> --features ok
cargo install borrowing_exerci
bw --file <FILENAME> --mode ok

# Compile-Time Error:
cargo run --bin bw -- -f <FILENAME> -m err | bat -l rs
cargo run --example <FILENAME> --features err
cargo install borrowing_exerci
bw --file <FILENAME> -m err | bat -l rs
"#,
        file_name
    );
    println!("{}", ret);
}

pub fn devx_cmds(file_name: &str) {
    let ret = format!(
        r#"
# Show this Help for Developer
cargo run --bin bw -- --file {0} | bat -l cmd
../target/debug/bw --file {0} | bat -l cmd
cargo install borrowing_exerci
bw --file {0}
bw --file {0} | bat -l cmd

# Use the tool cargo-expand
cargo expand --example {0} --features ok | bat -l cmd
cargo expand --example {0} --features err | bat -l cmd
RUSTFLAGS="--emit mir" cargo build --release --example {0} --features ok
RUSTFLAGS="--emit mir" cargo build --release --example {0} --features err
ls -al ../target/release/examples/{0}-*.mir
open -t ../target/release/examples/{0}-*.mir

# Show the Help for User
cargo run --bin bw -- --file {0}
../target/debug/bw --file {0}
cargo install borrowing_exerci
bw --file {0} | bat -l cmd

# Run OK:
cargo run --example {0} --features ok | bat -l cmd
cargo run --bin bw -- --file {0} --mode ok | bat -l cmd
../target/debug/bw --file {0} --mode ok | bat -l cmd
cargo install borrowing_exerci
bw --file {0} --mode ok

# Compile-Time Error:
cargo run --example {0} --features err | bat -l cmd
cargo run --bin bw -- -f {0} -m err | bat -l rs
../target/debug/bw --file {0} --mode err | bat -l cmd
cargo install borrowing_exerci
bw --file {0} -m err | bat -l rs
"#,
        file_name
    );
    println!("{}", ret);
}

pub fn user_cmds(file_name: &str) {
    let ret = format!(
        r#"
// Show this command
cargo install borrowing_exerci
bw --file {0} | bat -l cmd

// Run OK:
cargo install borrowing_exerci
bw --file <RS_FILE_NAME> --mode ok | bat -l rs

// Compile-Time Error:
cargo install borrowing_exerci
bw --file <RS_FILE_NAME> -m err | bat -l rs
"#,
        file_name
    );
    println!("{}", ret);
}

//## 题外话
//- https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=50eaf399f641e0965e86be08a6d2d777

// https://stackoverflow.com/questions/26946646/rust-package-with-both-a-library-and-a-binary

// Cow, as_ref()
// https://stackoverflow.com/questions/47147844/how-do-i-get-a-str-or-string-from-stdborrowcowstr
// https://jwilm.io/blog/from-str-to-cow/

// fn contains
// https://stackoverflow.com/questions/48794974/how-to-check-if-a-string-contains-a-substring-in-rust

// fn is_empty
// https://doc.rust-lang.org/nightly/std/vec/struct.Vec.html#method.is_empty

// regex
// https://stackoverflow.com/questions/19274493/regular-expression-match-lines-starting-with-a-certain-character-or-whitespace-a
// https://stackoverflow.com/questions/1240504/regular-expression-to-match-string-starting-with-stop

// fn split
// https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust

// fn to_uppercase
// https://doc.rust-lang.org/std/char/struct.ToUppercase.html

// file read write
// https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html
// https://doc.rust-lang.org/rust-by-example/std_misc/file/create.html
