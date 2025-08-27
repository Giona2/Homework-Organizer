use std::io::{self, Write};


pub enum Error {
    ClassOfThatTagNotFound,
    TagAleadyExists,
    InvalidMovementDirection,
    InvalidAssignmentIndex,
    MissingArguments,
    InvalidArgumentType,
} impl Error {
    /// Displays a message based on the given error branch, then waits for user to enter
    pub fn show(self) {
        match self {
            Self::ClassOfThatTagNotFound => {
                print!("A class with that tag could not be found");
            }
            Self::TagAleadyExists => {
                print!("A class with that tag already exists. Please choose a different tag or remove that class first");
            }
            Self::InvalidMovementDirection => {
                print!("The given movement command is invalid. Please either use \'u\' or \'d\' for up or down");
            }
            Self::InvalidAssignmentIndex => {
                print!("An assignment of that index in this class does not exist")
            }
            Self::MissingArguments => {
                print!("An incorrect number of arguments was given. Please run `h` or `help` to view the commands")
            }
            Self::InvalidArgumentType => {
                print!("An argument was given with an invalid type. Please run `h` or `help` to view the commands")
            }
        }
        io::stdout().flush().unwrap();

        let mut input_buf = String::new();
        io::stdin().read_line(&mut input_buf).unwrap();
    }
}
