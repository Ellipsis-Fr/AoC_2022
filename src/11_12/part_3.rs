use AoC_2022::text_file_reader::TextFileReader;

use monkey_p3::Monkey;

mod monkey_p3;
mod operation_str;

fn main() {
    println!("Puzzle du 11/12 Partie 2");
    
    let puzzle = get_puzzle();
    let monkeys = get_monkeys(puzzle);
    // println!("{:?}", monkeys);
    let level_monkey_business = get_level_monkey_business(monkeys);
    println!("{level_monkey_business}");
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("11_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn get_monkeys(puzzle: Vec<String>) -> Vec<Monkey> {
    let mut monkeys = vec![];
    const NUMBER_LINES_BY_MONKEY: usize = 7;
    const NUMBER_LINES_DEFINING_MONKEY: usize = 5;

    for (index, _) in puzzle.iter().step_by(NUMBER_LINES_BY_MONKEY).enumerate() {
        let start_index_defining_monkey = index * NUMBER_LINES_BY_MONKEY + 1;
        let monkey_definition = puzzle[start_index_defining_monkey..start_index_defining_monkey + NUMBER_LINES_DEFINING_MONKEY].to_vec();
        let monkey = Monkey::new(monkey_definition);
        monkeys.push(monkey)
    }

    monkeys
}

fn get_level_monkey_business(mut monkeys: Vec<Monkey>) -> u32 {
    let number_monkey = monkeys.len();
    
    for _ in 0..20 {
        for index in 0..number_monkey {
            let count_items = monkeys.get(index).unwrap().items.len();
            for _ in 0..count_items {
                let monkey = monkeys.get_mut(index).unwrap();
                monkey.operates();
                let index_next_monkey = monkey.next_monkey();
                let item = monkey.items.pop_front().unwrap();

                let monkey = monkeys.get_mut(index_next_monkey as usize).unwrap();
                monkey.items.push_back(item);
            }
        }
    }

    let (largest_number_inspection_1, largest_number_inspection_2) = get_two_largest_number_inspection(monkeys);

    largest_number_inspection_1 * largest_number_inspection_2
}

fn get_two_largest_number_inspection(monkeys: Vec<Monkey>) -> (u32, u32) {
    let mut inspections_counter = monkeys.iter().map(|m| m.inspection_counter).collect::<Vec<u32>>();
    inspections_counter.sort_by(|a, b| b.cmp(a));
    (inspections_counter[0], inspections_counter[1])
}