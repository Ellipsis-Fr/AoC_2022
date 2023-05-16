use AoC_2022::text_file_reader::TextFileReader;

use instruction::Instructions;

mod instruction;

fn main() {
    println!("Puzzle du 10/12 Partie 1");
    
    let puzzle = get_puzzle();
    let signal_strength = get_signal_strength(puzzle);
    println!("signal_strenght total : {signal_strength}");
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("10_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn get_signal_strength(puzzle: Vec<String>) -> i32 {
    let mut x = 1;
    let mut cycle_counter = 0;

    let mut signal_strength = 0;

    for (i, command) in puzzle.into_iter().enumerate() {
        let instruction = Instructions::new(command);

        match instruction {
            Instructions::Noop => {
                cycle_counter += 1;
                signal_strength += add_signal_strength(x, cycle_counter, i);
            },
            Instructions::AddX(v) => {
                for _ in 0..2 {
                    cycle_counter += 1;
                    signal_strength += add_signal_strength(x, cycle_counter, i);
                }
                x += v;
            }
        }

    }

    signal_strength
}

fn add_signal_strength(x: i32, cycle_counter: u8, i: usize) -> i32 {
    if cycle_counter % 20 != 0 || (cycle_counter / 20) % 2 == 0 {
        0
    } else {
        cycle_counter as i32 * x
    }
}