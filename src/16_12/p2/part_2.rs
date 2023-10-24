use core::time;
use std::time::Instant;

use AoC_2022::text_file_reader::TextFileReader;

use crate::hydraulic_network::HydraulicNetwork;

mod hydraulic_network;

fn main() {
    let start_time = Instant::now();
    println!("Puzzle du 16/12 Partie 2");

    const TIME: u32 = 26;
    
    let puzzle = get_puzzle();
    let hydraulic_network = HydraulicNetwork::new(puzzle);
    // println!("{:?}", hydraulic_network);
    let max_pressure = hydraulic_network.get_max_pressure_can_be_released_for_given_time("AA", TIME);
    println!("max_pressure {max_pressure}");
    let end_time = Instant::now();
    println!("Time elapsed {:?}", end_time.duration_since(start_time));
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("16_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn time_to_explain(number_of_elephant: u32) -> u32 {
    if number_of_elephant == 0 {
        return 0;
    }

    let mut time_to_explain = 4;

    for i in 1..number_of_elephant {
        time_to_explain += (time_to_explain) / (i + 1)
    }

    time_to_explain
}