use core::borrow;
use std::{collections::HashMap, rc::Rc, time::Instant, cell::RefCell};

use AoC_2022::text_file_reader::TextFileReader;
use shape::{LinkedList, CycleGuesser, Node};

use crate::shape::{TetrisFactory, Shape, Coordinate};

mod shape;

fn main() {
    let start_time = Instant::now();
    println!("Puzzle du 17/12 Partie 2");
    
    let puzzle = get_puzzle();
    let jets = puzzle.chars().collect::<Vec<char>>();
    // let tower_size = get_tower_size_for_specific_count_of_fallen_rock(jets, 1_000_000_000_000);
    // println!("{tower_size}");

    let count_of_rock = 1_000_000_000_000;
    // let count_of_rock = 2022;

    create_linked_list(jets, count_of_rock);

    let end_time = Instant::now();
    println!("Time elapsed {:?}", end_time.duration_since(start_time));
}

fn get_puzzle() -> String {
    let mut text_file_reader = TextFileReader::new("17_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content().to_string()
}

fn check_movement_possible(tetrimino_to_move: &Box<dyn Shape>, ref_tetriminos_by_position: &HashMap<i64, Vec<Rc<Box<dyn Shape>>>>) -> bool {
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

fn create_linked_list(jets: Vec<char>, mut count_of_rock: i64) {
    let mut linked_list: LinkedList<CycleGuesser> = LinkedList::new();
    let cycle_lenght = jets.len();


    let mut tower_size = 0;

    let direction_top = "^";
    let direction_bottom = "v";
    
    let mut ptr_jets = 0;
    let mut total_jets = 0;

    let mut cycle_found = false;
    
    let tetriminos_index = [1, 2, 3, 4, 5];
    let mut ptr_tetriminos_index = 0;

    let mut tetriminos_by_position: HashMap<i64, Vec<Rc<Box<dyn Shape>>>> = HashMap::new(); // y -> Vec<Rc<tetrimino>>
    
    loop {

        let mut tetrimino = TetrisFactory::new(tetriminos_index[ptr_tetriminos_index], (2, tower_size + 4));
        let init_jet = ptr_jets;
        
        loop {
            let next_jet = match jets.get(ptr_jets) {
                Some(v) => {
                    total_jets += 1;
                    ptr_jets += 1;
                    v
                }
                None => {
                    total_jets += 1;
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

        if !cycle_found {
            let cycle_guesser = CycleGuesser::new(Rc::clone(&rc_tetrimino), ptr_jets - 1, total_jets, tetriminos_index[ptr_tetriminos_index]);
            linked_list.add(cycle_guesser);

            if total_jets > 2 * cycle_lenght as u32 {
                let a = check_cycle(&linked_list, cycle_lenght);
                match a {
                    None => (),
                    Some((tetriminos_per_cycle, height_per_cycle)) => {
                        cycle_found = true;
                        println!("hauteur de cycle trouvée {height_per_cycle}");
                        let current_index = linked_list.current_index;

                        let cycle_to_do = count_of_rock / tetriminos_per_cycle;
                        count_of_rock %= tetriminos_per_cycle;
                        let extra_height = cycle_to_do * height_per_cycle;
                        // let a: i32 = extra_height.try_into().unwrap(); 
                        tower_size += extra_height;
                        dbg!(count_of_rock, extra_height, tower_size);

                        for i in (tetriminos_per_cycle..=current_index).rev() {
                            let c = linked_list.get(i).unwrap();
                            let (order, mut coordinates) = {
                                let d = c.borrow().data.clone();
                                (d.ptr_tetriminos_index , (d.tetrimino.clone().get_coordinates()).iter().cloned().collect::<Vec<Coordinate>>())
                            };

                            for c in coordinates.iter_mut() {
                                c.1 += extra_height;
                            }

                            let new_tetrimino = TetrisFactory::new_with_coordinates(order, coordinates);
                            let rc_tetrimino = Rc::new(new_tetrimino);
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
                        }


                        // manque juste à recréer une partie de la séquence dans "tetriminos_by_position" pour que cela bloque les prochain tetriminos
                        // il faudrait reprendre la séquence du cycle et la placer à hauteur trouvée
                    }
                }
            }
        }
    }

    println!("{tower_size}");


    // print_test(&linked_list);
}

fn check_cycle(linked_list: &LinkedList<CycleGuesser>, cycle_lenght: usize) -> Option<(i64, i64)> {
    let current_index = linked_list.current_index;
    // println!("current index : {current_index}");
    let mut index_to_look = current_index;
    let actual_cycle_guesser = linked_list.get(current_index).unwrap();
    
    // {
    //     let borrow_node = actual_cycle_guesser.borrow();
    //     let data = &borrow_node.data;
    //     let data_name = &data.tetrimino_name;
    //     let data_x_position = &data.x_position;
    //     let data_index_jet = data.index_jet;
    //     let data_total_jet = data.total_jet;
    //     let data_height = data.height;
        
    //     println!("actual_cycle_guesser :");
    //     println!("name : {data_name}, x_position : {data_x_position}, index_jet : {data_index_jet}, total_jet : {data_total_jet}, height : {data_height}");
    // }
    
    let (actual_x_position, actual_total_jet, actual_height, actual_index_jet) = {
        let borrow_data = &actual_cycle_guesser.borrow().data;
        (borrow_data.x_position, borrow_data.total_jet, borrow_data.height, borrow_data.index_jet)
    };


    loop {
        index_to_look -= 5;
        let cycle_guesser = match linked_list.get(index_to_look) {
            Err(_) => break,
            Ok(v) => v
        };

        // {
        //     let borrow_node = cycle_guesser.borrow();
        //     let data = &borrow_node.data;
        //     let data_name = &data.tetrimino_name;
        //     let data_x_position = &data.x_position;
        //     let data_index_jet = data.index_jet;
        //     let data_total_jet = data.total_jet;
        //     let data_height = data.height;
            
        //     println!("cycle_guesser :");
        //     println!("name : {data_name}, x_position : {data_x_position}, index_jet : {data_index_jet}, total_jet : {data_total_jet}, height : {data_height}");
        // }
        

        let (x_position, total_jet, height) = {
            let borrow_data = &cycle_guesser.borrow().data;
            (borrow_data.x_position, borrow_data.total_jet, borrow_data.height)
        };

        if actual_x_position == x_position && (actual_total_jet - total_jet) >= cycle_lenght as u32 {
            let index_to_look_to_find_cycle = index_to_look - (current_index - index_to_look);
            // println!("index_to_look_to_find_cycle : {index_to_look_to_find_cycle}, current_index : {current_index}, index_to_look : {index_to_look}");

            if index_to_look_to_find_cycle < 0 {
                continue;
            }

            let may_cycle_guesser = match linked_list.get(index_to_look_to_find_cycle) {
                Err(_) => break,
                Ok(v) => v
            };

            let (may_x_position, may_total_jet, may_height, may_index_jet) = {
                let borrow_data = &may_cycle_guesser.borrow().data;
                (borrow_data.x_position, borrow_data.total_jet, borrow_data.height, borrow_data.index_jet)
            };

            if actual_x_position == may_x_position && (actual_total_jet - total_jet) == (total_jet - may_total_jet) && (actual_height - height) == (height - may_height) {
                // checker pièce par pièce
                if is_same_tetriminos(may_cycle_guesser, cycle_guesser) {
                    dbg!(actual_index_jet, may_index_jet);
                    dbg!(current_index, index_to_look);
                    dbg!(actual_height, height, may_height);
                    return Some((current_index - index_to_look, actual_height - height));
                }
            }
        }
    }
    
    None
}

fn is_same_tetriminos(mut first_cycle: Rc<RefCell<Node<CycleGuesser>>>, mut second_cycle: Rc<RefCell<Node<CycleGuesser>>>) -> bool {
    let break_point = {
        second_cycle.borrow().data.clone()
    };

    loop {

        first_cycle = {
            let borrow = first_cycle.borrow();
            borrow.next.clone().unwrap()
        };
        
        second_cycle = {
            let borrow = second_cycle.borrow();
            borrow.next.clone().unwrap()
        };

        {
            if first_cycle.borrow().data == break_point {
                break;
            }
        }

        let (first_cycle_name, first_cycle_x_position) = {
            let borrow_data = &first_cycle.borrow().data;
            (borrow_data.tetrimino_name.clone(), borrow_data.x_position)
        };

        let (second_cycle_name, second_cycle_x_position) = {
            let borrow_data = &second_cycle.borrow().data;
            (borrow_data.tetrimino_name.clone(), borrow_data.x_position)
        };

        if first_cycle_name != second_cycle_name || first_cycle_x_position != second_cycle_x_position {
            return  false;
        }
        
    }

    true
}

fn print_test(linked_list: &LinkedList<CycleGuesser>) {
    let index = 888; // vertical bar
    match linked_list.get(index) {
        Ok(v) => {
            println!("Element index {index}");
            let borrow_node = v.borrow();
            let data = &borrow_node.data;
            let data_name = &data.tetrimino_name;
            let data_index_jet = data.index_jet;
            let data_total_jet = data.total_jet;
            let data_height = data.height;
            
            println!("data :");
            println!("name : {data_name}, index_jet : {data_index_jet}, total_jet : {data_total_jet}, height : {data_height}");

            let previous = borrow_node.previous.as_ref().unwrap().upgrade().unwrap();
            let previous_data = &previous.borrow().data;
            let previous_data_name = &previous_data.tetrimino_name;
            let previous_data_index_jet = previous_data.index_jet;
            let previous_data_total_jet = previous_data.total_jet;
            let previous_data_height = previous_data.height;

            println!("previous data :");
            println!("name : {previous_data_name}, index_jet : {previous_data_index_jet}, total_jet : {previous_data_total_jet}, height : {previous_data_height}");

            
            let next = borrow_node.next.as_ref().unwrap();
            let next_data = &next.borrow().data;
            let next_data_index_jet = next_data.index_jet;
            let next_data_total_jet = next_data.total_jet;
            let next_data_height = next_data.height;
            let next_data_name = &next_data.tetrimino_name;

            println!("next data :");
            println!("name : {next_data_name}, index_jet : {next_data_index_jet}, total_jet : {next_data_total_jet}, height : {next_data_height}");

        }
        Err(e) => println!("{e:?}")
    }


    println!("");
    println!("=============================================");
    println!("=============================================");
    println!("=============================================");
    println!("");


    let index = 509; // square
    match linked_list.get(index) {
        Ok(v) => {
            println!("Element index {index}");
            let borrow_node = v.borrow();
            let data = &borrow_node.data;
            let data_name = &data.tetrimino_name;
            let data_index_jet = data.index_jet;
            let data_total_jet = data.total_jet;
            let data_height = data.height;
            
            println!("data :");
            println!("name : {data_name}, index_jet : {data_index_jet}, total_jet : {data_total_jet}, height : {data_height}");

            let previous = borrow_node.previous.as_ref().unwrap().upgrade().unwrap();
            let previous_data = &previous.borrow().data;
            let previous_data_name = &previous_data.tetrimino_name;
            let previous_data_index_jet = previous_data.index_jet;
            let previous_data_total_jet = previous_data.total_jet;
            let previous_data_height = previous_data.height;

            println!("previous data :");
            println!("name : {previous_data_name}, index_jet : {previous_data_index_jet}, total_jet : {previous_data_total_jet}, height : {previous_data_height}");

            
            let next = borrow_node.next.as_ref().unwrap();
            let next_data = &next.borrow().data;
            let next_data_index_jet = next_data.index_jet;
            let next_data_total_jet = next_data.total_jet;
            let next_data_height = next_data.height;
            let next_data_name = &next_data.tetrimino_name;

            println!("next data :");
            println!("name : {next_data_name}, index_jet : {next_data_index_jet}, total_jet : {next_data_total_jet}, height : {next_data_height}");
        },
        Err(e) => println!("{e:?}")
    }


    println!("");
    println!("=============================================");
    println!("=============================================");
    println!("=============================================");
    println!("");

    let index = 1200; // horizontal bar
    match linked_list.get(index) {
        Ok(v) => {
            println!("Element index {index}");
            let borrow_node = v.borrow();
            let data = &borrow_node.data;
            let data_name = &data.tetrimino_name;
            let data_index_jet = data.index_jet;
            let data_total_jet = data.total_jet;
            let data_height = data.height;
            
            println!("data :");
            println!("name : {data_name}, index_jet : {data_index_jet}, total_jet : {data_total_jet}, height : {data_height}");

            let previous = borrow_node.previous.as_ref().unwrap().upgrade().unwrap();
            let previous_data = &previous.borrow().data;
            let previous_data_name = &previous_data.tetrimino_name;
            let previous_data_index_jet = previous_data.index_jet;
            let previous_data_total_jet = previous_data.total_jet;
            let previous_data_height = previous_data.height;

            println!("previous data :");
            println!("name : {previous_data_name}, index_jet : {previous_data_index_jet}, total_jet : {previous_data_total_jet}, height : {previous_data_height}");

            
            let next = borrow_node.next.as_ref().unwrap();
            let next_data = &next.borrow().data;
            let next_data_index_jet = next_data.index_jet;
            let next_data_total_jet = next_data.total_jet;
            let next_data_height = next_data.height;
            let next_data_name = &next_data.tetrimino_name;

            println!("next data :");
            println!("name : {next_data_name}, index_jet : {next_data_index_jet}, total_jet : {next_data_total_jet}, height : {next_data_height}");
        },
        Err(e) => println!("{e:?}")
    }

    println!("Size : {}", linked_list.size());
}