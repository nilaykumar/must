# must

A simple command line task list. Written primarily to learn Rust.

## (planned) usage

Show task lists and create a new task list

``` sh
must list
must list -a my_task_list
```

Switch to task list and show tasks

``` sh
must list my-task-list
must
```

Add a new task to the selected list

``` sh
must -a "Finish work presentation" -p inprogress
must -a "Give the goldfish a bath" -p todo
```

Complete a task and remove a task from the list

``` sh
must
must -c 1
must -d 2
```

