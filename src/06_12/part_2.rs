use AoC_2022::text_file_reader::TextFileReader;
use std::{collections::{HashMap, VecDeque, HashSet}, cell::RefCell};

fn main() {
    println!("Puzzle du 06/12 Partie 2");
    
    let puzzle = get_puzzle();
    process_datastreams(puzzle);
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("06_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn process_datastreams(datastreams: Vec<String>) {
    for datastream in datastreams {
        let mut marker_chars = VecDeque::new();

        for (index, char) in datastream.chars().enumerate() {
            marker_chars.push_back(char);

            match index {
                0..=12 => continue,
                13 => (),
                _ => {
                    marker_chars.pop_front();
                }
            }

            if start_of_message_detected(&marker_chars) {
                println!("flux : {datastream},\nmarqueur : {:?},\nindex : {}", marker_chars, index + 1);
                break;
            } 
        }
    }
}

fn start_of_message_detected(marker_chars: &VecDeque<char>) -> bool {
    marker_chars.into_iter().collect::<HashSet<&char>>().len() == 14
}