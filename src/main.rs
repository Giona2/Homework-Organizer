use std::io::{self, Write};

use termion;

mod data_file;
    use data_file::{DataFile, ClassData};
mod error;
    use error::Error;


/// Manager for the application's Text User Interface
struct TUI {
    data_file: DataFile,
} impl TUI {
    /// Create new instance of this
    pub fn new() -> Self { return Self {
        data_file: DataFile::get(),
    }}

    /// Display the current state of the app to `Stdout`
    pub fn display(&self) {
        for (class_name, class_data) in self.data_file.content.iter() {
            println!("[{}] {}:", class_data.tag, class_name);
            for (assignment_index, assignment) in class_data.assignments.iter().enumerate() {
                println!("    {}) {}", assignment_index, assignment);
            }
        }
    }
}

/// Prints a help message
fn help() {
    println!(
r#"
ac <class_name> <class_tag>
    Add a class

rc <class_tag>
    Remove a class

cc <class_tag> <new_class_tag>
    Change a classes tag

mc <class_tag> <class_position>
    Move a class up [u] or down [d]

aa <class_tag> <assignment>
    Add an assignment to a class

ra <class_tag> <assignment_index>
    Remove an assignment from a class

ca <class_tag>
    Clear a classes assignments
"#
    )
}

/// Command to add a class
fn add_class(data_file: &mut DataFile, class_name: &str, class_tag: &str) {
    // Insert the new class
    let class_metadata = ClassData::new(class_tag, []);
    data_file.content.insert(class_name.into(), class_metadata);

    // Write to the data file
    data_file.write();
}

/// Command to remove a class
fn remove_class(data_file: &mut DataFile, class_tag: &str) {
    // Remove given class
    if let Some(class_index) = data_file.clone().find_class_index_from_tag(class_tag) {
        data_file.content.swap_remove_index(class_index);
    } else {
        Error::ClassOfThatTagNotFound.show();
    }

    // Write to the data file
    data_file.write();
}

/// Command to change a classes tag
fn change_class_tag(data_file: &mut DataFile, from_class_tag: &str, to_class_tag: &str) {
    // Check if the new tag already exists
    if data_file.clone().tag_already_exists(to_class_tag) { Error::TagAleadyExists.show() }

    // Find the old tag and replace it
    if let Some((_, class_tag)) = data_file.find_class_from_tag(from_class_tag) {
        class_tag.tag = to_class_tag.to_owned();
    } else {
        Error::ClassOfThatTagNotFound.show();
    }

    // Write to the data file
    data_file.write();
}

/// Command to move a class up or down
fn move_class(data_file: &mut DataFile, class_tag: &str, position: &str) {
    if let Some(class_index) = data_file.clone().find_class_index_from_tag(class_tag) {
        if position == "u" && class_index < data_file.clone().content.len() - 1 {
            data_file.content.move_index(class_index, class_index + 1);
        } else if position == "d" && class_index > 0 {
            data_file.content.move_index(class_index, class_index - 1);
        } else {
            Error::InvalidMovementDirection.show();
        }
    } else {
        Error::ClassOfThatTagNotFound.show();
    }

    // Write to the data file
    data_file.write();
}

/// Command to add an assignment to a specified class
fn add_assignment(data_file: &mut DataFile, class_tag: &str, assignment_name: &str) {
    // Add the assignment
    if let Some((_, class_data)) = data_file.find_class_from_tag(class_tag) {
        class_data.assignments.push(assignment_name.to_owned());
    } else {
        Error::ClassOfThatTagNotFound.show();
    }

    // Write to the data file
    data_file.write();
}

/// Command to remove an assignment from a specified class by its index
fn remove_assignment(data_file: &mut DataFile, class_tag: &str, assignment_index: usize) {
    // Remove given assignment by index
    if let Some((_, class_data)) = data_file.find_class_from_tag(class_tag) {
        // get the number of assignments
        let assignment_len = class_data.clone().assignments.len();

        // remove the assignment if the index is valid
        let actual_assignment_index = assignment_index - 1;
        if actual_assignment_index > 0 && actual_assignment_index < assignment_len {
            _ = class_data.assignments.remove(assignment_index);
        } else {
            Error::InvalidAssignmentIndex.show();
        }
    } else {
        Error::ClassOfThatTagNotFound.show();
    }

    // Write to the data file
    data_file.write();
}

/// Command to clear assignments in a specified class
fn clear_assignments(data_file: &mut DataFile, class_tag: &str) {
    // Clear assignments
    if let Some((_, class_data)) = data_file.find_class_from_tag(class_tag) {
        class_data.assignments.clear();
    } else {
        Error::ClassOfThatTagNotFound.show();
    }

    // Write to the data file
    data_file.write();
}


fn main() {
    let tui = TUI::new();

    let mut is_running = true;
    while is_running {
        // Clear the terminal and return the cursor to home
        println!("{}", termion::clear::All);
        println!("{}", termion::cursor::Goto(1, 1));

        // Display the TUI
        tui.display();

        // Get the given command
        print!("> ");
        io::stdout().flush().unwrap();
        let mut user_input: String = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        let input_parse: Vec<&str> = user_input.split(" ").map(|x| x.trim()).collect();

        // Match the command
        match input_parse.clone()[0].trim() {
            "e" | "exit" => is_running = false,
                       _ => {}
        }
    }
}
