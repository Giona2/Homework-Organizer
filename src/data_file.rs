use std::{
    env,
    fs,
};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json;


const DATA_DIR:      &str = "/.local/share/homework_organizer";
const DATA_FILE_DIR: &str = "/data.json";


#[derive(Serialize, Deserialize, Debug)]
pub struct ClassData {
    tag: String,
    assignments: Vec<String>,
} impl ClassData {
    pub fn new<S: ToString, T: IntoIterator<Item = S>>(tag: S, assignments: T) -> Self { return Self {
        tag: tag.to_string(),
        assignments: assignments.into_iter().map(|x| x.to_string()).collect(),
    }}
}

pub struct DataFile {
    pub content: IndexMap<String, ClassData>,

    path: String,
} impl DataFile {
    /// Read the data file to this
    ///
    /// If the data file or the data folder doesn't exist, this function will create it
    pub fn get() -> Self {
        let home_dir = env::home_dir().unwrap()
            .to_str().unwrap()
            .to_owned();

        // If the .local file doesn't exist, create it
        if !fs::exists(home_dir.clone() + DATA_DIR).unwrap() {
            // create the data directory
            fs::create_dir(home_dir.clone() + DATA_DIR).unwrap();

            // create the data file
            fs::write(home_dir.clone() + DATA_DIR + DATA_FILE_DIR, "{}").unwrap();
        }

        // Deserialize the data file
        let data_content = fs::read_to_string(home_dir.clone() + DATA_DIR + DATA_FILE_DIR).unwrap();
        let data_map: IndexMap<String, ClassData> = serde_json::from_str(&data_content).unwrap();

        // Construct and return
        return Self {
            content: data_map,
            path: home_dir + DATA_DIR + DATA_FILE_DIR,
        }
    }

    /// Serialize the current content stored here and write to the data file
    pub fn write(&self) {
        // Serialize content
        let data_content = serde_json::to_string(&self.content).unwrap();

        // Write to the file
        fs::write(&self.path, data_content).unwrap();
    }
}
