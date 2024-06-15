mod time;
mod note;

use serde_json::to_string_pretty;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::{fs::File, io::Error, io::ErrorKind};
use note::Entry;

const DATABASE_FILE: &str = "database.json";

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

    //file.set_len(0).unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();
    
    let json = to_string_pretty(&json_values).unwrap();
    println!("{}, {:?}", json, json_values);
    file.write_all(&json.as_bytes())
        .expect("Error writing file!");
}

fn current_entry_number() -> usize {
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .open(DATABASE_FILE);
    
    if let Ok(mut inside_file) = file {
        let mut json_values: Vec<Entry> = vec![];
        let mut file_content = String::new();
        inside_file.read_to_string(&mut file_content).unwrap();
        if !file_content.is_empty() {
            json_values = serde_json::from_str(&file_content)
                .expect("The json file should be formatted correctly");
    
            println!("OK");
            return json_values.len() + 1;
        }
    }
    return 1;
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
