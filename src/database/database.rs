use serde_json::to_string_pretty;
use std::fs::OpenOptions;


use std::io::{Read, Seek, SeekFrom, Write};
use std::{fs::File, io::Error, io::ErrorKind};
use crate::note::{Entry, Category};

pub enum DatabaseAction {
    Add,
    Remove(usize),
    Modify(usize)
}

enum Mode {
    Ascending,
    Descending
}

enum SortBy{
    Unsorted,
    DateCreated(Mode),
    DateModified(Mode),
    Title(Mode)
}

const DATABASE_FILE: &str = "database.json";
const DATABASE_FILE_FILTERED: &str = "database_filtered.json";

pub fn refresh_json_database(entry: Entry, action: DatabaseAction) -> Result<(), Error> {
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

    match action {
        DatabaseAction::Add => json_values.push(entry),

        DatabaseAction::Modify(key) => {
            if key >= json_values.len() {return Err(Error::from(ErrorKind::InvalidInput));}
            json_values[key].modify(entry);
        }

        DatabaseAction::Remove(key) =>{
            if key >= json_values.len() {return Err(Error::from(ErrorKind::InvalidInput));}
            json_values.remove(key);
        } 
    }

    file.set_len(0).unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();
    
    let json = to_string_pretty(&json_values).unwrap();
    println!("{}, {:?}", json, json_values);
    file.write_all(&json.as_bytes())
        .expect("Error writing file!");

    Ok(())
}

pub fn generate_filtered_json(category: Category, sort: SortBy) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .open(DATABASE_FILE_FILTERED)
        .unwrap();

    let mut file_content = String::new();

    file.read_to_string(&mut file_content).unwrap();

    let mut json_values: Vec<Entry> = vec![];

    if !file_content.is_empty() {
        json_values = serde_json::from_str(&file_content)
            .expect("The json file should be formatted correctly");

        println!("OK");
    }

    // ! to add logical code here

    file.set_len(0).unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();
    
    let json = to_string_pretty(&json_values).unwrap();
    println!("{}, {:?}", json, json_values);
    file.write_all(&json.as_bytes())
        .expect("Error writing file!");

    Ok(())
}


pub fn current_entry_number() -> usize {
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
            return json_values.len() + 1;
        }
    }
    return 1;
}

pub fn create_list() -> Result<(), Error> {
    File::create(DATABASE_FILE)?;
    Ok(())
}