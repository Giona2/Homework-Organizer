use crate::{
    data_file::{
        DataFile,
        ClassData
    },
    error::Error,
};

use std::io::{
    self,
    Write,
};


/// Prints a help message
pub fn help() {
    println!(
r#"
{} <class_name> <class_tag>
    Add a class

{} <class_tag>
    Remove a class

{} <class_tag> <new_class_tag>
    Change a classes tag

{} <class_tag> <class_position>
    Move a class up [u] or down [d]

{} <class_tag> <assignment>
    Add an assignment to a class

{} <class_tag> <assignment_index>
    Remove an assignment from a class

{} <class_tag>
    Clear a classes assignments
"#, ADD_CLASS_COMMAND, REMOVE_CLASS_COMMAND, CHANGE_CLASS_TAG_COMMAND, MOVE_CLASS_COMMAND, ADD_ASSIGNMENT_COMMAND, REMOVE_ASSIGNMENT_COMMAND, CLEAR_ASSIGNMENTS_COMMAND,
    );

    io::stdout().flush().unwrap();

    let mut input_buf = String::new();
    io::stdin().read_line(&mut input_buf).unwrap();
}

/// Command to add a class
pub const ADD_CLASS_COMMAND: &str = "ac";
pub fn add_class(data_file: &mut DataFile, class_name: &str, class_tag: &str) {
    // Check if the tag already exists
    if data_file.clone().tag_already_exists(class_tag) {
        Error::TagAleadyExists.show();
        return;
    }

    // Insert the new class
    let class_metadata = ClassData::new(class_tag, []);
    data_file.content.insert(class_name.into(), class_metadata);

    // Write to the data file
    data_file.write();
}

/// Command to remove a class
pub const REMOVE_CLASS_COMMAND: &str = "rc";
pub fn remove_class(data_file: &mut DataFile, class_tag: &str) {
    // Remove given class
    if let Some(class_index) = data_file.clone().find_class_index_from_tag(class_tag) {
        data_file.content.swap_remove_index(class_index);
    } else {
        Error::ClassOfThatTagNotFound.show();
        return;
    }

    // Write to the data file
    data_file.write();
}

/// Command to change a classes tag
pub const CHANGE_CLASS_TAG_COMMAND: &str = "cc";
pub fn change_class_tag(data_file: &mut DataFile, from_class_tag: &str, to_class_tag: &str) {
    // Check if the new tag already exists
    if data_file.clone().tag_already_exists(to_class_tag) {
        Error::TagAleadyExists.show();
        return;
    }

    // Find the old tag and replace it
    if let Some((_, class_tag)) = data_file.find_class_from_tag(from_class_tag) {
        class_tag.tag = to_class_tag.to_owned();
    } else {
        Error::ClassOfThatTagNotFound.show();
        return;
    }

    // Write to the data file
    data_file.write();
}

/// Command to move a class up or down
pub const MOVE_CLASS_COMMAND: &str = "mc";
pub fn move_class(data_file: &mut DataFile, class_tag: &str, position: &str) {
    if let Some(class_index) = data_file.clone().find_class_index_from_tag(class_tag) {
        if position == "u" && class_index > 0 {
            data_file.content.move_index(class_index, class_index - 1);
        } else if position == "d" && class_index < data_file.clone().content.len() - 1 {
            data_file.content.move_index(class_index, class_index + 1);
        } else {
            Error::InvalidMovementDirection.show();
            return;
        }
    } else {
        Error::ClassOfThatTagNotFound.show();
        return;
    }

    // Write to the data file
    data_file.write();
}

/// Command to add an assignment to a specified class
pub const ADD_ASSIGNMENT_COMMAND: &str = "aa";
pub fn add_assignment(data_file: &mut DataFile, class_tag: &str, assignment_name: &str) {
    // Add the assignment
    if let Some((_, class_data)) = data_file.find_class_from_tag(class_tag) {
        class_data.assignments.push(assignment_name.to_owned());
    } else {
        Error::ClassOfThatTagNotFound.show();
        return;
    }

    // Write to the data file
    data_file.write();
}

/// Command to remove an assignment from a specified class by its index
pub const REMOVE_ASSIGNMENT_COMMAND: &str = "ra";
pub fn remove_assignment(data_file: &mut DataFile, class_tag: &str, assignment_index: usize) {
    println!("{}", assignment_index);

    // Remove given assignment by index
    if let Some((_, class_data)) = data_file.find_class_from_tag(class_tag) {
        // get the number of assignments
        let assignment_len = class_data.clone().assignments.len();

        // remove the assignment if the index is valid
        let actual_assignment_index: isize = assignment_index as isize - 1;
        if actual_assignment_index >= 0 && actual_assignment_index < assignment_len as isize {
            _ = class_data.assignments.remove(actual_assignment_index as usize);
        } else {
            Error::InvalidAssignmentIndex.show();
            return;
        }
    } else {
        Error::ClassOfThatTagNotFound.show();
        return;
    }

    // Write to the data file
    data_file.write();
}

/// Command to clear assignments in a specified class
pub const CLEAR_ASSIGNMENTS_COMMAND: &str = "ca";
pub fn clear_assignments(data_file: &mut DataFile, class_tag: &str) {
    // Clear assignments
    if let Some((_, class_data)) = data_file.find_class_from_tag(class_tag) {
        class_data.assignments.clear();
    } else {
        Error::ClassOfThatTagNotFound.show();
        return;
    }

    // Write to the data file
    data_file.write();
}
