use std::{
    env,
    fs,
};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json;


const DATA_DIR:      &str = "/.local/share/homework_organizer";
const DATA_FILE_DIR: &str = "/data.json";


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClassData {
    pub tag: String,
    pub assignments: Vec<String>,
} impl ClassData {
    pub fn new<S: ToString, T: IntoIterator<Item = S>>(tag: S, assignments: T) -> Self { return Self {
        tag: tag.to_string(),
        assignments: assignments.into_iter().map(|x| x.to_string()).collect(),
    }}
}

#[derive(Clone)]
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

        // create the data directory
        fs::create_dir(home_dir.clone() + DATA_DIR)
            .unwrap_or(());

        // create the data file
        fs::write(home_dir.clone() + DATA_DIR + DATA_FILE_DIR, "{}")
            .unwrap_or(());

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

    /// Returns the name of and the mutable reference to the first class with the given tag
    ///
    /// Returns None if a class with the given tag is not found
    pub fn find_class_from_tag<'a>(&'a mut self, class_tag: &str) -> Option<(String, &'a mut ClassData)> {
        let mut result: Option<(String, &'a mut ClassData)> = None;

        for (class_name, class_data) in self.content.iter_mut() {
            if class_data.tag == class_tag {
                result = Some((class_name.to_owned(), class_data));
                break;
            }
        }

        return result
    }

    /// Returns the index of the first class with the given tag
    pub fn find_class_index_from_tag(&self, class_tag: &str) -> Option<usize> {
        let mut result: Option<usize> = None;

        for (class_index, (_, class_data)) in self.content.iter().enumerate() {
            if class_data.tag == class_tag {
                result = Some(class_index);
                break;
            }
        }

        return result
    }

    /// Check if a class with the given tag already exists
    pub fn tag_already_exists(&self, class_tag: &str) -> bool {
        let mut result: bool = false;

        for (_, class_data) in self.content.iter() {
            if class_data.tag == class_tag {
                result = true;
                break;
            }
        }

        return result
    }
}
