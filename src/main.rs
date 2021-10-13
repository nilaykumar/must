/// must: a simple CLI todo application
use std::fs::{self, File, OpenOptions};
use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

extern crate home;

// the data folder will be contained in the user's home folder
const DATA_FOLDER: &str = "must/";
// the text file containing our todo list
const DATA_FILE: &str = "must";
// the header of the todo list data file
const DATA_HEADER: &str = "# DO NOT MODIFY THIS FILE MANUALLY\n";

fn main() {
    // create the data folder if it does not exist
    let mut path = PathBuf::new();
    match home::home_dir() {
        Some(p) => path.push(p),
        None => panic!("Could not find home directory!"),
    };
    path.push(DATA_FOLDER);
    fs::create_dir_all(&path).unwrap_or_else(|error| {
        panic!("Could not find or create data folder: {:?}", error);
    });
    path.push(DATA_FILE);
    let file_path = path.as_path();

    // if a todo list file doesn't exist, create it
    let mut file_options = OpenOptions::new();
    file_options.read(true).append(true);
    let mut file: File;
    match file_options.open(file_path) {
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                println!("Could not find file {:?}, creating it now.", file_path);
                file = file_options
                    .create(true)
                    .open(file_path)
                    .unwrap_or_else(|error| {
                        panic!("Could not create file: {:?}", error);
                    });
                file.write(DATA_HEADER.as_bytes()).unwrap();
            } else {
                panic!("Could not open file: {:?}", e);
            }
        }
        Ok(f) => file = f,
    }

    let mut contents = String::new();
    file.seek(SeekFrom::Start(0)).unwrap();
    match file.read_to_string(&mut contents) {
        Err(e) => panic!("Could not read file: {:?}", e),
        Ok(_) => println!("Read: \n{:?}", contents),
    }
}
