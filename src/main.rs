use std::error;
use std::fs::read_to_string;
use std::fs::File;
use std::fs::{self, read};
use std::io;
use std::io::Write;
use std::path::Path;

// fn write_to_new_file() -> io::Result<()> {
//     let mut file = File::create("new_file.txt")?; // Create a new file
//     let content = "This is a new file.\n";

//     file.write_all(content.as_bytes())?; // Write the content to the file
//     Ok(())
// }

fn main() {
    println!("Hello, world!");
    let mut path = get_input("Please enter the path to your file.");

    while !validate_file(&path) {
        path = get_input("Please enter the path to your file.");
    }

    let old_word = get_input("Please enter the word or phrase you want to replace.");
    let new_word = get_input("Please enter the new word or phrase to use in its place.");

    // Read file data into result type
    let result = read_file(&path);
    // Declare file_data variable
    let file_data: String;

    match result {
        Ok(content) => {
            file_data = content;
        }
        Err(e) => {
            println!("Error reading file: {}", e);
            return; // Exit early if there's an error
        }
    }

    let new_text = file_data.replace(&old_word, &new_word);

    let other_path = get_input("Please enter a file path output the results to.");

    match write_to_file(&other_path, &new_text) {
        Ok(_) => println!("Succesfully wrote to file!"),
        Err(e) => eprintln!("Error writing to file: {}", e),
    }
}

fn get_input(msg: &str) -> String {
    // Create a mutable string to store the input in
    let mut input = String::new();
    // Prompt the user
    println!("{}", msg);

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input.trim().to_string();
}

// Ensure file exists at given path
fn validate_file(path: &str) -> bool {
    if Path::new(path).exists() {
        println!("The file exists!");
        return true;
    } else {
        println!("The file doesn't exist!");
        return false;
    }
}

// Read a list of lines from the file
fn read_file(path: &str) -> Result<String, io::Error> {
    let data = read_to_string(&path)?;
    println!("{}", data.len());

    Ok(data)
}

// Write/overwrite to file
fn write_to_file(new_path: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(new_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
