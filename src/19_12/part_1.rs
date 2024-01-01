use AoC_2022::text_file_reader::TextFileReader;


fn main() {
    println!("Puzzle du 19/12 Partie 1");
    
    let puzzle = get_puzzle();

}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("19_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}