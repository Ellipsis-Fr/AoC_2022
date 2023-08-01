use AoC_2022::text_file_reader::TextFileReader;
use device::Sensor;

mod device;

fn main() {
    println!("Puzzle du 15/12 Partie 1");
    
    let puzzle = get_puzzle();
    let sensors = get_sensors(puzzle);
    // println!("Sensors : {:?}", sensors);
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("15_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn get_sensors(puzzle: Vec<String>) -> Vec<Sensor> {
    let mut sensors = vec![];
    
    for p in puzzle {
        sensors.push(Sensor::new(&p));
    }

    sensors
}