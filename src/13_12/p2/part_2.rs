use AoC_2022::text_file_reader::TextFileReader;

use tree::Tree;
mod tree;

fn main() {
    println!("Puzzle du 13/12 Partie 2");
    
    let mut puzzle = get_puzzle();
    puzzle.retain(|signal| !signal.is_empty());
    let divider_packets = vec![String::from("[[2]]"), String::from("[[6]]")];
    puzzle.extend_from_slice(&divider_packets);

    let tree = Tree::new(puzzle);
    let sorted_signals = tree.get_list_of_sorted_node();
    // println!("{:?}", sorted_signals);
    // println!();

    let decoder_key = get_decoder_key(sorted_signals, divider_packets);
    println!("Decoder key : {decoder_key}");
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("13_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn get_decoder_key(sorted_signals: Vec<String>, divider_packets: Vec<String>) -> u32 {
    let mut decoder_key = 1;
    for divider_packet in divider_packets {
        let key = sorted_signals.iter().position(|signal| *signal == divider_packet).unwrap() + 1;
        decoder_key *= key;
    }
    decoder_key as u32
}