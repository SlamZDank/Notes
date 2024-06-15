use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::{fs::File, io::Error, io::ErrorKind};

mod time;
use time::now_date;

const DATABASE_FILE: &str = "database.json";

#[derive(Serialize, Deserialize, Debug)]
enum Category {
    Draft,
    InProgress,
    Cancelled,
    Done,
}

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    key: u64,
    category: Category,
    title: String,
    message: String,
    pub date_created: u64,
    pub date_modified: u64,
    tags: Vec<String>
}


impl Entry {
    fn new() -> Entry {
        Entry {
            // todo: query the last element of the entries in the json file
            category: Category::Draft,
            key: 3,
            title: String::from(""),
            message: String::from(""),
            date_created: now_date(),
            date_modified: now_date(),
            tags: vec![]
        }
    }
}

fn refresh_json_database(entry: Entry) {
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .open(DATABASE_FILE)
        .unwrap();

    let mut file_content = String::new();

    file.read_to_string(&mut file_content).unwrap();

    let mut json_values: Vec<Entry> = vec![];

    if !file_content.is_empty() {
        json_values = serde_json::from_str(&file_content)
            .expect("The json file should be formatted correctly");

        println!("OK");
    }

    json_values.push(entry);

    file.set_len(0);
    file.seek(SeekFrom::Start(0));
    
    let json = to_string_pretty(&json_values).unwrap();
    println!("{}, {:?}", json, json_values);
    file.write_all(&json.as_bytes())
        .expect("Error writing file!");
}

fn create_list() -> Result<(), Error> {
    File::create(DATABASE_FILE)?;
    Ok(())
}

fn enter_audit() {
    let note = Entry::new();
    println!("Would you like to edit {}? (y/N)", note.title);
}

fn add_note(item: Entry) {
    {
        let file = File::open(DATABASE_FILE);
        match file {
            Err(error) => {
                match error.kind() {
                    // TODO: proper error handling
                    ErrorKind::NotFound => create_list().unwrap(),
                    ErrorKind::PermissionDenied => println!("Permission denied!"),
                    _ => {
                        println!("Unexpected error, Panicking!");
                        panic!();
                    }
                }
            }
            Ok(_) => (),
        }
    }
    // todo read the items in the "database", add json current item then store it, easier to implement in database but my ass wouldnt think of that
    refresh_json_database(item);
}

pub fn note_audit() {
    let new_entry = Entry::new();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn note_addtion_checking() {
        let note = Entry::new();
        add_note(note);
    }
}
