use AoC_2022::text_file_reader::TextFileReader;

mod way;

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
    let smallest_number_of_steps_to_reach_the_end = get_smallest_number_of_steps_to_reach_the_end(map);
    println!("Nombre de pas : {}", smallest_number_of_steps_to_reach_the_end);
    
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

fn get_smallest_number_of_steps_to_reach_the_end(map: Vec<Vec<u32>>) -> i32 {
    let initial_position = get_position(&map, 96); // (y, x)
    let final_position = get_position(&map, 123); // (y, x)
    println!("Position finale: {:?}", final_position);
    
    find_path(&map, initial_position, final_position, vec![initial_position], 0, 0) - 1
}

fn get_position(map: &Vec<Vec<u32>>, value_to_find: u32) -> (i32, i32) {
    for (y, piece_of_map) in map.iter().enumerate() {
        if piece_of_map.contains(&value_to_find) {
            return (y as i32, piece_of_map.iter().position(|v| v == &value_to_find).unwrap() as i32);
        }
    }

    panic!("Aucune position trouvée pour la valeur {}", value_to_find);
}

fn find_path(map: &Vec<Vec<u32>>, current_position: (i32, i32), final_position: (i32, i32), mut positions_visited: Vec<(i32, i32)>, current_step_count: i32, mut current_smallest_step_count: i32) -> i32 {
    if current_smallest_step_count > 0 {
        current_smallest_step_count -= 1;

        if current_smallest_step_count <= 0 {
            return 0;
        }

        if current_step_count >= current_smallest_step_count {
            return 0;
        }
    }

    let actual_value = map[current_position.0 as usize][current_position.1 as usize];
    if actual_value == 123 {
        return 1;
    }

    let mut smallest_count = 0;
    let mut find_a_way = false;
    let possible_ways = get_possible_ways(map, current_position, final_position, actual_value, &positions_visited);
    println!("possibles_ways : {:?}", possible_ways);

    positions_visited.extend(possible_ways.iter().map(|w| w.coordinate));
    for possible_way in possible_ways {
        positions_visited.push(possible_way.coordinate);
        let count = find_path(map, possible_way.coordinate, final_position, positions_visited.clone(), current_step_count + 1, current_smallest_step_count);
        positions_visited.pop();

        if count != 0 {
            println!("voie trouvée");
            find_a_way = true;
            if smallest_count == 0 || count < smallest_count {
                smallest_count = count;

                if smallest_count < current_smallest_step_count {
                    current_smallest_step_count = smallest_count;
                }
            }
        }
        
    }
    
    if find_a_way {
        smallest_count += 1;
    }
    smallest_count
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

        // let height = if actual_value > value_of_the_next_position {
        //     actual_value - value_of_the_next_position
        // } else {
        //     value_of_the_next_position - actual_value
        // };
        // if height > 1 {
        //     continue;
        // }

        if actual_value > value_of_the_next_position || value_of_the_next_position - actual_value > 1 {
            continue;
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