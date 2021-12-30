/// must: a simple CLI todo application
use std::fs::{self, File, OpenOptions};
use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

extern crate clap;
extern crate home;

use clap::{App, Arg};

mod task;
use task::Task;

use crate::task::add_to_task_list;

// application version
const VERSION: &str = "0.1";

// the data folder will be contained in the user's home folder
const DATA_FOLDER: &str = "must/";
// the text file containing our todo list
const DATA_FILE: &str = "todo.txt";
// the header of the todo list data file
const DATA_HEADER: &str = "# DO NOT MODIFY THIS FILE MANUALLY\n";

fn main() {
    // handle command line arguments
    let matches = App::new("must")
        .version(VERSION)
        .author("Nilay Kumar <nilaykumar@tutanota.com>")
        .about("A simple CLI todo application")
        .arg(
            Arg::with_name("add")
                .short("a")
                .long("add")
                .help("Adds a task to the current task list")
                .takes_value(true)
                .empty_values(false),
        )
        .get_matches();

    // get data file handle
    let mut file: File = get_data_file();

    // read data file
    let contents: String = get_string_from_file(&mut file);

    // load task list from string
    let mut task_list: Vec<Task> = string_as_task_list(contents);

    println!("found {} tasks.", task_list.len());
    for task_string in &task_list {
        println!("{}", task_string.task);
    }

    // adding a new task
    if let Some(task_string) = matches.value_of("add") {
        println!("Adding task: {}", task_string);

        add_to_task_list(&mut task_list, task_string);
    }

    println!(
        "Writing modified task list:\n{}",
        &task_list_as_string(&task_list)
    );

    // FIXME figure out why the generated list is wrong

    // write modified task list to file
    write_string_to_file(&mut file, &task_list_as_string(&task_list));
}

fn get_data_file() -> File {
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
    file_options.read(true).write(true);
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
                // write header to the new data file
                file.write(DATA_HEADER.as_bytes()).unwrap();
            } else {
                panic!("Could not open file: {:?}", e);
            }
        }
        Ok(f) => file = f,
    }
    file
}

fn get_string_from_file(file: &mut File) -> String {
    let mut contents = String::new();
    file.seek(SeekFrom::Start(0)).unwrap();
    if let Err(e) = file.read_to_string(&mut contents) {
        panic!("Could not read file: {:?}", e);
    }
    contents
}

fn write_string_to_file(file: &mut File, s: &String) {
    file.seek(SeekFrom::Start(0)).unwrap();
    file.write_all(s.as_bytes()).unwrap_or_else(|error| {
        panic!("Could not write to file: {:?}", error);
    });
}

fn string_as_task_list(contents: String) -> Vec<Task> {
    let mut task_list: Vec<Task> = Vec::new();
    let list_data = contents.split("\n").collect::<Vec<&str>>();
    for task_string in list_data {
        if task_string.is_empty() {
            continue;
        }
        // if the line starts with a #, ignore it
        if task_string.trim().starts_with("#") {
            continue;
        }
        // otherwise, create a task using the id and task string
        let task_data = task_string.split(" ").collect::<Vec<&str>>();
        let task = Task {
            id: task_data[0]
                .parse()
                .expect(&format!("Could not parse {:?} as an integer", task_data[0])),
            task: task_data
                .get(1..)
                .expect(&format!("Could not parse task from {:?}", task_data))
                .iter()
                .fold(String::from(""), |acc, x| acc + x),
        };
        task_list.push(task);
    }
    task_list
}

fn task_list_as_string(task_list: &Vec<Task>) -> String {
    let mut list_string = String::from(DATA_HEADER);
    for task in task_list {
        list_string += &format!("{} {}\n", task.id, task.task);
    }
    list_string
}
