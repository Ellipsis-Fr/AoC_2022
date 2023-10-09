use AoC_2022::text_file_reader::TextFileReader;

use crate::hydraulic_network::HydraulicNetwork;

mod hydraulic_network;

fn main() {
    println!("Puzzle du 16/12 Partie 1");
    
    let puzzle = get_puzzle();
    let hydraulic_network = HydraulicNetwork::new(puzzle);
    println!("{:?}", hydraulic_network);
    // let max_pressure = hydraulic_network.get_max_pressure_can_be_released_for_given_time("AA", 30);
    // println!("max_pressure {max_pressure}");
    
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("16_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

