use crate::{current_entry_number, locale::now_date};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum Category {
    All,
    Draft,
    InProgress,
    Cancelled,
    Done,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    category: Category,
    key: usize,
    title: String,
    message: String,
    pub date_created: u64,
    pub date_modified: u64,
    tags: Vec<String>,
}

#[wasm_bindgen]
impl Entry {
    // #[wasm_bindgen(constructor)]
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
            tags: vec![],
        }
    }

    #[wasm_bindgen(setter)]
    pub fn set_entry(&mut self, other: Entry) {
        self.title = other.title;
        self.message = other.message;
        self.category = other.category;
        self.date_modified = now_date();
        self.tags = other.tags.to_owned();
    }

    #[wasm_bindgen(constructor)]
    pub fn from(category: Category, title: String, message: String, tags: Vec<String>) -> Entry {
        let entry_number = current_entry_number();
        Entry {
            // todo: query the last element of the entries in the json file
            // todo: add a way to customize the input of a valid entry
            category,
            key: entry_number,
            title,
            message,
            date_created: now_date(),
            date_modified: now_date(),
            tags,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn get_category(&self) -> Category {
        self.category
    }

    #[wasm_bindgen(getter)]
    pub fn title(&self) -> String {
        self.title.clone()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn name() {}
}
