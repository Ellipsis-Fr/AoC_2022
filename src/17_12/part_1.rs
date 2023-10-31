use std::{collections::HashMap, rc::Rc};

use AoC_2022::text_file_reader::TextFileReader;

use crate::shape::{TetrisFactory, Shape};

mod shape;

fn main() {
    println!("Puzzle du 17/12 Partie 1");
    
    let puzzle = get_puzzle();
    let jets = puzzle.chars().collect::<Vec<char>>();
    let tower_size = get_tower_size_for_specific_count_of_fallen_rock(jets, 2022);
    println!("{tower_size}");
}

fn get_puzzle() -> String {
    let mut text_file_reader = TextFileReader::new("17_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content().to_string()
}

fn get_tower_size_for_specific_count_of_fallen_rock(jets: Vec<char>, mut count_of_rock: i32) -> i32 {
    let mut tower_size = 0;

    let direction_top = "^";
    let direction_bottom = "v";
    
    let mut ptr_jets = 0;
    
    let tetriminos_index = [1, 2, 3, 4, 5];
    let mut ptr_tetriminos_index = 0;

    let mut tetriminos_by_position: HashMap<i32, Vec<Rc<Box<dyn Shape>>>> = HashMap::new(); // y -> Vec<Rc<tetrimino>>
    
    loop {

        let mut tetrimino = TetrisFactory::new(tetriminos_index[ptr_tetriminos_index], (2, tower_size + 4));
        
        loop {
            let next_jet = match jets.get(ptr_jets) {
                Some(v) => {
                    ptr_jets += 1;
                    v
                }
                None => {
                    ptr_jets = 1;
                    jets.get(0).unwrap()
                }
            };

            tetrimino.shift(&next_jet.to_string());
            if !check_movement_possible(&tetrimino, &tetriminos_by_position) {
                tetrimino.back(&next_jet.to_string());
            }

            tetrimino.shift(direction_bottom);
            if !check_movement_possible(&tetrimino, &tetriminos_by_position) {
                tetrimino.shift(direction_top);
                break;
            }
        }
       
        if ptr_tetriminos_index + 1 == tetriminos_index.len() {
            ptr_tetriminos_index = 0;
        } else {
            ptr_tetriminos_index += 1;
        }

        let tetrimino_top = tetrimino.get_top();
        if tower_size < tetrimino_top {
            tower_size = tetrimino_top;
        }
        
        let rc_tetrimino = Rc::new(tetrimino);
        let mut position_in_y_already_use = vec![];
        for coordinate in rc_tetrimino.get_coordinates() {
            if position_in_y_already_use.contains(&coordinate.1) {
                continue;
            }
            
            match tetriminos_by_position.get_mut(&coordinate.1) {
                Some(tetriminos_in_position) => {
                    position_in_y_already_use.push(coordinate.1);
                    tetriminos_in_position.push(Rc::clone(&rc_tetrimino));
                },
                None => {
                    position_in_y_already_use.push(coordinate.1);
                    tetriminos_by_position.insert(coordinate.1, vec![Rc::clone(&rc_tetrimino)]);
                }
            }
        }

        count_of_rock -= 1;
        if count_of_rock == 0 {
            break;
        }
    }

    tower_size
}

fn check_movement_possible(tetrimino_to_move: &Box<dyn Shape>, ref_tetriminos_by_position: &HashMap<i32, Vec<Rc<Box<dyn Shape>>>>) -> bool {
    let tetrimino_top = tetrimino_to_move.get_top();
    let tetrimino_bottom = tetrimino_to_move.get_bottom();

    if tetrimino_to_move.check_crossing_with_border() {
        for y in tetrimino_bottom..=tetrimino_top {
            if ref_tetriminos_by_position.contains_key(&y) {
                let tetriminos_at_same_height = ref_tetriminos_by_position.get(&y).unwrap();
                for tetrimino_at_same_height in tetriminos_at_same_height {
                    if !tetrimino_to_move.check_crossing_with_another_tetrimino(Rc::clone(tetrimino_at_same_height)) {
                        return false;
                    }
                }
            }
        }
        true
    } else {
        false
    }
}