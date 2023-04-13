use AoC_2022::text_file_reader::TextFileReader;
use std::{collections::HashMap, cell::RefCell};

#[derive(Debug)]
struct Procedure {
    quantity: u32,
    origin: u32, // name stack one
    destination: u32 // name stack two
}

impl Procedure {
    fn new(quantity: u32, origin: u32, destination: u32) -> Self {
        Self { quantity, origin, destination }
    }
}

fn main() {
    println!("Puzzle du 05/12 Partie 2");
    
    let puzzle = get_puzzle();
    let (stacks_string, procedures_string) = split_skacks_and_procedures(puzzle); 
    
    let stacks = get_stacks(stacks_string);
    let procedures = get_procedures(procedures_string);

    execution_of_procedures(&stacks, procedures);
    print_crates_name_on_top(&stacks);
}

fn get_puzzle() -> String {
    let mut text_file_reader = TextFileReader::new("05_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content().to_string()
}

fn split_skacks_and_procedures(puzzle: String) -> (String, String) {
    let mut start_procedures: bool = false;
    let mut stacks_vec: Vec<String> = Vec::new();
    let mut procedures_vec: Vec<String> = Vec::new();
    puzzle.lines().collect::<Vec<&str>>().iter().for_each(|x| {
        if x.is_empty() {
            start_procedures = true;
            stacks_vec.remove(stacks_vec.len() - 1);
            return
        }
        if start_procedures {
            procedures_vec.push(x.to_string())
        } else {
            stacks_vec.push(x.to_string())
        }
    });

    (stacks_vec.join("\n"), procedures_vec.join("\n"))
}

fn get_stacks(stacks_string: String) -> HashMap<u32, RefCell<Vec<String>>> {
    let mut stacks: HashMap<u32, RefCell<Vec<String>>> = HashMap::new();
    let space_between_stacks = 4;
    stacks_string.lines().rev().for_each(|x| {
        let chars = x.chars();
        
        for (stack_number, crate_name) in chars.skip(1).enumerate().step_by(4) {
            if crate_name.is_whitespace() {
                continue;
            }
            match stacks.get_mut(&((stack_number / space_between_stacks) as u32 + 1)) {
                Some(stack) => stack.get_mut().push(crate_name.to_string()),
                None => {
                    let stack = vec![crate_name.to_string()];
                    stacks.insert((stack_number / space_between_stacks) as u32 + 1, RefCell::new(stack));
                }
            }         
        }
    });

    stacks
}

fn get_procedures(procedures_string: String) -> Vec<Procedure> {
    let mut procedures: Vec<Procedure> = Vec::new();
    
    procedures_string.lines().for_each(|procedure_string| {
        let procedure = procedure_string.split_whitespace().filter_map(|c| c.parse().ok()).collect::<Vec<u32>>();
        procedures.push(Procedure::new(procedure[0], procedure[1], procedure[2]));        
    });

    procedures
}

fn execution_of_procedures(stacks: &HashMap<u32, RefCell<Vec<String>>>, procedures: Vec<Procedure>) {
    for procedure in procedures {
        let mut origin_stack = stacks.get(&procedure.origin).unwrap().borrow_mut();
        let mut destination_stack = stacks.get(&procedure.destination).unwrap().borrow_mut();
        let mut crates_to_move: Vec<String> = Vec::new();

        for _ in 0..procedure.quantity {
            let possible_crate = origin_stack.pop();
            
            if let Some(krate) = possible_crate {
                crates_to_move.push(krate);
            }
        }

        if !crates_to_move.is_empty() {
            crates_to_move.reverse();
            destination_stack.extend(crates_to_move);
        }
    }
}

fn print_crates_name_on_top(stacks: &HashMap<u32, RefCell<Vec<String>>>) {
    let mut keys = stacks.clone().into_keys().collect::<Vec<u32>>();
    keys.sort();
    
    for key in keys {
        print!("{}", stacks.get(&key).unwrap().borrow().last().unwrap());
    }
}