use serde::{Deserialize, Serialize};
use crate::{current_entry_number, time::now_date};

#[derive(Serialize, Deserialize, Debug)]
pub enum Category {
    All,
    
    Draft,
    InProgress,
    Cancelled,
    Done,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    category: Category,
    key: usize,
    pub title: String,
    message: String,
    pub date_created: u64,
    pub date_modified: u64,
    tags: Vec<String>
}

impl Entry {
    pub fn new() -> Entry {
        let entry_number = current_entry_number();
        Entry {
            // todo: query the last element of the entries in the json file
            // todo: add a way to customize the input of a valid entry
            category: Category::Draft,
            key: entry_number,
            title: format!("New Note {}", entry_number),
            message: String::from(""),
            date_created: now_date(),
            date_modified: now_date(),
            tags: vec![]
        }
    }

    pub fn modify(&mut self, other: Entry) {
        self.title = other.title;
        self.message = other.message;
        self.category = other.category;
        self.date_modified = now_date();
        self.tags = other.tags.to_owned();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn name() {
    }
}
