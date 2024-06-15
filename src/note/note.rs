use serde::{Deserialize, Serialize};
use crate::{current_entry_number, time::now_date};

#[derive(Serialize, Deserialize, Debug)]
enum Category {
    Draft,
    InProgress,
    Cancelled,
    Done,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    category: Category,
    pub title: String,
    message: String,
    pub date_created: u64,
    pub date_modified: u64,
    tags: Vec<String>
}

impl Entry {
    pub fn new() -> Entry {
        Entry {
            // todo: query the last element of the entries in the json file
            category: Category::Draft,
            title: format!("New Note {}", current_entry_number()),
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
