use std::{sync::{Mutex, Arc}, thread};

use AoC_2022::text_file_reader::TextFileReader;

mod way;
use crate::way::tree::Tree;
use way::tree::Node;

#[derive(Debug)]
struct PossibleWay {
    coordinate: (i32, i32),
    value: u32
}

impl PossibleWay {
    fn get_distance_from(&self, position: (i32, i32)) -> f64 {
        let d: f64 = (((self.coordinate.0 - position.0).pow(2) + (self.coordinate.1 - position.1).pow(2))) as f64;
        d.sqrt()
    }
}

fn main() {
    println!("Puzzle du 12/12 Partie 1");
    
    let puzzle = get_puzzle();
    let map = init_map(puzzle);
    let tree = get_way(map);
    // println!("tree : {:?}", tree.lock().unwrap().root_node);
    // tree.lock().unwrap().print();
    let tree_lock = tree.lock().unwrap();
    tree_lock.count_step();
    tree_lock.print_step();

    // println!("Nombre de pas : {}", smallest_number_of_steps_to_reach_the_end);
    
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("12_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn init_map(puzzle: Vec<String>) -> Vec<Vec<u32>> {
    let mut map = vec![];

    for line in puzzle {
        let mut piece_of_map = vec![];
        for c in line.chars() {
            match  c {
                'S' => piece_of_map.push(96), // == 'a' - 1 == 97 - 1
                'E' => piece_of_map.push(123), // == 'z' + 1 == 122 + 1
                _ => piece_of_map.push(c as u32) // [97-122]
            }
        }
        map.push(piece_of_map);
    }

    map
}

fn get_way(map: Vec<Vec<u32>>) -> Arc<Mutex<Tree>> {
    let initial_position = get_position(&map, 96); // (y, x)
    let final_position = get_position(&map, 123); // (y, x)
    println!("Position finale: {:?}", final_position);

    let tree = Arc::new(Mutex::new(Tree::new(initial_position, 96)));
    let lock_tree = tree.lock().unwrap();
    let actual_node = lock_tree.get_existing_node_by_position(initial_position).unwrap();
    drop(lock_tree);

    find_path(&map, actual_node, final_position, Arc::clone(&tree),  vec![initial_position]);
    tree
}

fn get_position(map: &Vec<Vec<u32>>, value_to_find: u32) -> (i32, i32) {
    for (y, piece_of_map) in map.iter().enumerate() {
        if piece_of_map.contains(&value_to_find) {
            return (y as i32, piece_of_map.iter().position(|v| v == &value_to_find).unwrap() as i32);
        }
    }

    panic!("Aucune position trouvée pour la valeur {}", value_to_find);
}

fn find_path(map: &Vec<Vec<u32>>, actual_node: Arc<Mutex<Node>>, final_position: (i32, i32), tree: Arc<Mutex<Tree>>, positions_visited: Vec<(i32, i32)>) {
    let mut lock_tree = tree.lock().unwrap();
    if lock_tree.end_node.is_some() {
        return;
    }
    
    let lock_actual_node = actual_node.lock().unwrap();
    let (current_position, actual_value) = ((lock_actual_node.point.y, lock_actual_node.point.x), lock_actual_node.point.value);
    let possible_ways = get_possible_ways(map, current_position, final_position, actual_value, &positions_visited);

    let mut nodes_to_continue = vec![];
    drop(lock_actual_node);
    
    for possible_way in &possible_ways {

        match lock_tree.get_existing_node_by_position(possible_way.coordinate) {
            Some(node) => {
                let mut lock_actual_node = actual_node.lock().unwrap();
                lock_actual_node.add_node(Arc::clone(&node));
            },
            None => {

                let node = Arc::new(Mutex::new(Node::new(possible_way.coordinate, possible_way.value)));
                lock_tree.add_node(Arc::clone(&node));
                let mut lock_actual_node = actual_node.lock().unwrap();
                lock_actual_node.add_node(Arc::clone(&node));
                nodes_to_continue.push(node);
            }
        }
    }

    drop(lock_tree);
    
    
    let mut handlers = vec![];
    for node in nodes_to_continue {
        let point = node.lock().unwrap().point.clone();
        let map = map.clone();
        let actual_node = Arc::clone(&node);
        let tree = Arc::clone(&tree);
        let mut positions_visited = positions_visited.clone();
        positions_visited.push((point.y, point.x));
        drop(node);
        // println!("là");
        let handler = thread::spawn(move || {
            find_path(&map, actual_node, final_position, tree, positions_visited)
        });
        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap();
    }
}

fn get_possible_ways(map: &Vec<Vec<u32>>, current_position: (i32, i32), final_position: (i32, i32), actual_value: u32, positions_visited: &Vec<(i32, i32)>) -> Vec<PossibleWay> {
    let mut possible_ways = vec![];
    
    for i in 0..4 {
        let next_move = match i {
            0 => (-1, 0), // UP
            1 => (1, 0), // DOWN
            2 => (0, -1), // LEFT
            3 => (0, 1), // RIGH
            _ => (0, 0)
        };

        let (y, x) = (current_position.0 as i32 + next_move.0, current_position.1 as i32 + next_move.1);
        if y < 0 || x < 0 {
            continue;
        }

        let next_position = (y as i32, x as i32);
        if positions_visited.contains(&next_position) {
            continue;
        }


        let value_of_the_next_position = match map.get(next_position.0 as usize) {
            None => continue,
            Some(line) => {
                match line.get(next_position.1 as usize) {
                    None => continue,
                    Some(v) => *v
                }
            }
        };

        if actual_value < value_of_the_next_position {
            if value_of_the_next_position - actual_value > 1 {
                continue;
            }
        }

        possible_ways.push(
            PossibleWay {
                coordinate: next_position,
                value: value_of_the_next_position
            }
        );
    }

    possible_ways.sort_unstable_by(|a, b| a.get_distance_from(final_position).total_cmp(&b.get_distance_from(final_position)));
    possible_ways
}