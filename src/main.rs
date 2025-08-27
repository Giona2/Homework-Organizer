use std::io::{self, Write};

use termion;

mod data_file;
    use data_file::DataFile;
mod error;
    use error::Error;
mod subcommands;


/// Manager for the application's Text User Interface
struct TUI {}
impl TUI {
    /// Create new instance of this
    pub fn new() -> Self { return Self {}}

    /// Display the current state of the app to `Stdout`
    pub fn display(&self, data_file: &DataFile) {
        for (class_name, class_data) in data_file.content.iter() {
            println!("[{}] {}:", class_data.tag, class_name);
            for (assignment_index, assignment) in class_data.assignments.iter().enumerate() {
                println!("    {}) {}", assignment_index + 1, assignment);
            }
            println!();
        }
    }
}


fn main() {
    let tui = TUI::new();
    let mut data_file = DataFile::get();

    let mut is_running = true;
    while is_running {
        // Clear the terminal and return the cursor to home
        println!("{}", termion::clear::All);
        println!("{}", termion::cursor::Goto(1, 1));

        // Display the TUI
        tui.display(&data_file);

        // Get the given command
        print!("> ");
        io::stdout().flush().unwrap();
        let mut user_input: String = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        let input_parse: Vec<&str> = user_input.split(" ").map(|x| x.trim()).collect();

        // Match the command
        let subcommand = input_parse.clone()[0].trim();
        match subcommand {
            "e" | "exit" => is_running = false,
            "h" | "help" => subcommands::help(),

            _ if subcommand == subcommands::ADD_CLASS_COMMAND => {
                if input_parse.len() != 3 {
                    Error::MissingArguments.show();
                    continue;
                }

                subcommands::add_class(&mut data_file, &input_parse[1], &input_parse[2]);
            }

            _ if subcommand == subcommands::REMOVE_CLASS_COMMAND => {
                if input_parse.len() != 2 {
                    Error::MissingArguments.show();
                    continue;
                }

                subcommands::remove_class(&mut data_file, &input_parse[1]);
            }

            _ if subcommand == subcommands::CHANGE_CLASS_TAG_COMMAND => {
                if input_parse.len() != 3 {
                    Error::MissingArguments.show();
                    continue;
                }

                subcommands::change_class_tag(&mut data_file, &input_parse[1], &input_parse[2]);
            }

            _ if subcommand == subcommands::MOVE_CLASS_COMMAND => {
                if input_parse.len() != 3 {
                    Error::MissingArguments.show();
                    continue;
                }

                subcommands::move_class(&mut data_file, &input_parse[1], &input_parse[2])
            }

            _ if subcommand == subcommands::ADD_ASSIGNMENT_COMMAND => {
                if input_parse.len() < 3 {
                    Error::MissingArguments.show();
                    continue;
                }

                let assignment_name: String = input_parse.clone()[2..].join(" ");
                subcommands::add_assignment(&mut data_file, &input_parse[1], &assignment_name);
            }

            _ if subcommand == subcommands::REMOVE_ASSIGNMENT_COMMAND => {
                if input_parse.len() != 3 {
                    Error::MissingArguments.show();
                    continue;
                }

                let assignment_index = input_parse.clone()[2].parse::<usize>();
                if let Ok(assignment_index) = assignment_index {
                    subcommands::remove_assignment(&mut data_file, &input_parse[1], assignment_index);
                } else {
                    Error::InvalidArgumentType.show();
                    continue;
                }
            }

            _ if subcommand == subcommands::CLEAR_ASSIGNMENTS_COMMAND => {
                if input_parse.len() != 2 {
                    Error::MissingArguments.show();
                    continue;
                }

                subcommands::clear_assignments(&mut data_file, &input_parse[1]);
            }

            _ => {}
        }
    }
}
