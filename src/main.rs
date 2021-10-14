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

enum Completion {
    Done,
    InProgress,
    Todo,
}

struct Task {
    id: usize,
    task: String,
    completion: Completion,
}

struct TaskList {
    name: String,
    tasks: Vec<Task>,
}

fn main() {
    // get data file handle
    let mut file: File = get_data_file();

    // read data file
    let contents: String = get_string_from_file(&mut file);

    // load task lists from string
    let task_lists: Vec<TaskList> = string_as_task_lists(contents);

    for task_list in task_lists {
        println!("= {}", task_list.name);
        for task in task_list.tasks {
            println!(
                "\t {} {} {}",
                task.id,
                completion_as_string(task.completion),
                task.task
            );
        }
    }
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

fn string_as_task_lists(contents: String) -> Vec<TaskList> {
    let mut task_lists: Vec<TaskList> = Vec::new();
    // task lists are written "= MyTaskList"
    let v = contents.split("=").collect::<Vec<&str>>();
    let mut it = v.iter();
    // ignore everything before the first task list
    it.next();

    for list_string in it {
        if list_string.len() == 0 {
            continue;
        };
        let list_data = list_string.split("\n").collect::<Vec<&str>>();
        let mut list = TaskList {
            name: list_data[0].trim().to_string(),
            tasks: Vec::new(),
        };
        for task_string in list_data.get(1..).unwrap() {
            if task_string.len() == 0 {
                continue;
            }
            let task_data = task_string.split(" ").collect::<Vec<&str>>();
            let task = Task {
                id: task_data[0]
                    .parse()
                    .expect(&format!("Could not parse {:?} as an integer", task_data[0])),
                completion: string_as_completion(task_data[1]).expect(&format!(
                    "Could not parse {:?} as a Completion",
                    task_data[1]
                )),
                task: task_data
                    .get(2..)
                    .expect(&format!("Could not parse task from {:?}", task_data))
                    .iter()
                    .fold(String::from(""), |acc, x| acc + x),
            };
            list.tasks.push(task)
        }
        task_lists.push(list);
    }
    task_lists
}

fn string_as_completion(s: &str) -> Result<Completion, ()> {
    match s.to_lowercase().as_str() {
        "todo" => Ok(Completion::Todo),
        "inprogress" => Ok(Completion::InProgress),
        "done" => Ok(Completion::Done),
        _ => Err(()),
    }
}

fn completion_as_string(completion: Completion) -> &'static str {
    match completion {
        Completion::Done => "done",
        Completion::InProgress => "inprogress",
        Completion::Todo => "todo",
    }
}
