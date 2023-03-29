# To-Do List Application

This is a simple command-line application written in Rust for managing to-do lists.

## How to Use

1. Clone the repository:

`$ git clone https://github.com/username/to-do-list.git`

2. Change directory to the project root:

`$ cd week9`

3. Build the application:

`$ cargo build`

4. Run the application:

`$ cargo run`

5. Enter a command:

   - `1. Add task`: Adds a task to the to-do list.
   - `2. Complete task`: Marks a task as completed.
   - `3. Remove task`: Removes a task from the to-do list.
   - `4. Show tasks`: Displays all tasks in the to-do list.
   - `5. Exit`: Exits the application.

## How it Works

The application uses a `HashMap<String, bool>` to store tasks as keys and their completion status as values.

The `Todo` struct provides methods for adding, completing, removing, and showing tasks.

In the main loop of the program, the user enters commands via standard input and matches them to the appropriate Todo method.
