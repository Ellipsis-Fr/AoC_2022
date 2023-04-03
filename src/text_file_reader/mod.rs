use std::env;

use std::error::Error;
use std::fs;

const PATH_TO_PUZZLE: &str = "puzzles/";

pub struct TextFileReader {
    file_name: String,
    content: Option<String>,
}

impl TextFileReader {
    pub fn new(file_name: &str) -> TextFileReader {
        TextFileReader {
            file_name: file_name.to_string(),
            content: None
        }
    }

    pub fn read_file_text(&mut self) -> Result<(), Box<dyn Error>> {
        let file_location = PATH_TO_PUZZLE.to_owned() + &self.file_name;
        println!("localisation du fichier : {file_location}");
        self.content = Some(fs::read_to_string(file_location)?);
        Ok(())
    }

    pub fn get_content(&self) -> &str {
        &self.content.as_ref().unwrap()
    }

    pub fn get_content_as_list_split_by_newline(&self) -> Vec<String> {
        self.content.as_ref().unwrap().to_owned().lines().map(str::to_string).collect()
    }
}