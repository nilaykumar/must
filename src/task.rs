pub struct Task {
    pub id: usize,
    pub task: String,
}

pub fn add_to_task_list(task_list: &mut Vec<Task>, task_string: &str) {
    // get next id
    let next_id = task_list.iter().map(|t| t.id).max().unwrap_or(0);
    let task = Task {
        id: next_id,
        task: String::from(task_string),
    };
    task_list.push(task);
}
