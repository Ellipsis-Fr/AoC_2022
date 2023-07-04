use std::collections::{HashSet, HashMap};

use AoC_2022::text_file_reader::TextFileReader;
mod line;
use line::{Line, Direction};

#[derive(Debug, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    
}

fn main() {
    println!("Puzzle du 14/12 Partie 1");
    
    let puzzle = get_puzzle();
    let rock_structure_path = get_rock_structure_path(puzzle);
    const origin: (u32, u32) = (500, 0); // (x, y)
    // println!("{}", rock_structure_path.len());
    // println!("{:?}", rock_structure_path);

    let sand_unit_count_before_fall = get_sand_unit_count_before_fall(rock_structure_path, origin);
    println!("sand unit count before fall : {sand_unit_count_before_fall}");
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("14_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn get_rock_structure_path(puzzle: Vec<String>) -> HashSet<Line> {
    let mut rock_structure_path = HashSet::new();

    for input in puzzle {
        let coordinates = input.split("->").map(|coordinate| coordinate.trim()).collect::<Vec<&str>>();
        for (index, origin) in coordinates.iter().enumerate() {
            let end = match coordinates.get(index + 1) {
                Some(c) => c,
                None => continue
            };

            rock_structure_path.insert(Line::new(origin, end));
        }
    }

    rock_structure_path
}

struct MovingSandGrain {
    actual_position: (u32, u32),
    previous_position: Option<(u32, u32)>,
    is_falling: bool
}

fn get_sand_unit_count_before_fall(rock_structure_path: HashSet<Line>, origin: (u32, u32)) -> u32 {
    let mut actual_sand_grain = origin; // current sand grain on which new sand falls, used as reference
    let mut sand_grains = vec![]; // list of all fallen sand grains
    let mut references_sand_grain = vec![origin]; // list of all sand grains that have been a reference

    let mut drop_coordinates = HashMap::new(); // list of fall coordinates, each linked to the current reference sand grain prior to the fall. key: (u32,u32) => value: (u32,u32) : drop_coordinate => current_sand_grain

    loop {
        // check that the current sand grain does not correspond to an old drop coordinate
        if drop_coordinates.contains_key(&actual_sand_grain) {
            actual_sand_grain = drop_coordinates.remove(&actual_sand_grain).unwrap();
            loop {
                let ex_reference_sand_grain = references_sand_grain.pop().unwrap();
                if actual_sand_grain == ex_reference_sand_grain {
                    references_sand_grain.push(actual_sand_grain);
                    break;
                }
            }
        }

        // initialize drop point at a coordinate offset by 1 in y (except for origin)
        let mut falling_sand_grain = actual_sand_grain; // drop point
        if actual_sand_grain != origin {
            falling_sand_grain = (actual_sand_grain.0, actual_sand_grain.1 - 1);
        }

        // check that the drop point is not in a wall or another sand grain // USELESS

        let moving_sand_grain = MovingSandGrain { actual_position: falling_sand_grain, previous_position: None, is_falling: false };
        if !fall_continuing(
            &rock_structure_path,
            origin,
            &mut actual_sand_grain,
            &mut sand_grains,
            &mut references_sand_grain,
            &mut drop_coordinates,
            falling_sand_grain,
            moving_sand_grain
        ) {
            break;
        }

    }

    sand_grains.len() as u32
}

fn fall_continuing(
    rock_structure_path: &HashSet<Line>,
    origin: (u32, u32),
    mut actual_sand_grain: &mut (u32, u32),
    mut sand_grains: &mut Vec<(u32, u32)>,
    mut references_sand_grain: &mut Vec<(u32, u32)>,
    mut drop_coordinates: &mut HashMap<(u32, u32), (u32, u32)>,
    mut falling_sand_grain: (u32, u32),
    mut moving_sand_grain: MovingSandGrain
) -> bool {
    let fall_from_origin = falling_sand_grain == origin;
    
    if fall_from_origin {
        falling_sand_grain = match get_next_drop_point(rock_structure_path, &origin) {
            Some(v) => v,
            None => {
                return false; // manages case of empty rock structure
            }
        };
    }
    
    // je regarde ensuite s'il peut bouger (à gauche, puis à droite)
    // et dans cette nouvelle méthode, si je constate que je peux bouger je checke si je ne tombe que de 1, 
    // - si oui je reviens dans cette méthode maj les différentes ref,
    // puis je rechecke s'il peut bouger et je répète cela x fois...
    // - si non, c'est une chute et je lance une recherche tel que je viens de le faire depuis origin, et une fois sol touché je reviens ici et relance le tout
    let mut next_position = (falling_sand_grain.0 - 1, falling_sand_grain.1 + 1);
    if !sand_grains.contains(&next_position) {
        // can move to the left ?
        if can_move(rock_structure_path, &next_position) {
            falling_sand_grain = next_position;

            moving_sand_grain.previous_position = Some(moving_sand_grain.actual_position);
            moving_sand_grain.actual_position = next_position;
            moving_sand_grain.is_falling = false;

            next_position.1 += 1;
            // can move go down ?
            if can_move(rock_structure_path, &next_position) {
                let next_position = match get_next_drop_point(rock_structure_path, &next_position) {
                    Some(v) => v,
                    None => {
                        return false;
                    }
                };
                falling_sand_grain = next_position;

                moving_sand_grain.previous_position = Some(moving_sand_grain.actual_position);
                moving_sand_grain.actual_position = next_position;
                moving_sand_grain.is_falling = true;
            }
        } else {
            next_position.0 = falling_sand_grain.0 + 1;
            // can move to the right ?
            if can_move(rock_structure_path, &next_position) {
                falling_sand_grain = next_position;
    
                moving_sand_grain.previous_position = Some(moving_sand_grain.actual_position);
                moving_sand_grain.actual_position = next_position;
                moving_sand_grain.is_falling = false;
    
                next_position.1 += 1;
                // can move go down ?
                if can_move(rock_structure_path, &next_position) {
                    let next_position = match get_next_drop_point(rock_structure_path, &next_position) {
                        Some(v) => v,
                        None => {
                            return false;
                        }
                    };
                    falling_sand_grain = next_position;
    
                    moving_sand_grain.previous_position = Some(moving_sand_grain.actual_position);
                    moving_sand_grain.actual_position = next_position;
                    moving_sand_grain.is_falling = true;
                }
            } else { // Can't move left or right 
                let is_new_ref = is_new_ref(&moving_sand_grain);
            }
        }
    }
    
    true
}

fn can_move(rock_structure_path: &HashSet<Line>, next_position: &(u32, u32)) -> bool {
    let vertical_lines = rock_structure_path.iter().filter(
        |l| 
        l.direction == Direction::Vertical && l.direction_coordinate == next_position.0 && l.vertex_a <= next_position.1 && l.vertex_b >= next_position.1
    ).collect::<Vec<&Line>>();

    let horizontal_lines = rock_structure_path.iter().filter(
        |l| 
        l.direction == Direction::Horizontal && l.direction_coordinate == next_position.1 && l.vertex_a <= next_position.0 && l.vertex_b >= next_position.0
    ).collect::<Vec<&Line>>();

    vertical_lines.is_empty() && horizontal_lines.is_empty()
}

fn get_next_drop_point(rock_structure_path: &HashSet<Line>, next_position: &(u32, u32)) -> Option<(u32, u32)> {
    let mut vertical_lines = rock_structure_path.iter().filter(
        |l| 
        l.direction == Direction::Vertical && l.direction_coordinate == next_position.0 && l.vertex_a > next_position.1
    ).collect::<Vec<&Line>>();
    vertical_lines.sort_by(|a, b| a.vertex_a.cmp(&b.vertex_a));

    let mut horizontal_lines = rock_structure_path.iter().filter(
        |l| 
        l.direction == Direction::Horizontal && l.direction_coordinate > next_position.1 + 1 && l.vertex_a <= next_position.0 && l.vertex_b >= next_position.0
    ).collect::<Vec<&Line>>();
    horizontal_lines.sort_by(|a, b| a.direction_coordinate.cmp(&b.direction_coordinate));

    let crossed_vertical_line = vertical_lines.get(0);
    let crossed_horizontal_line = get_crossed_horizontal_line(horizontal_lines);

    if crossed_vertical_line.is_some() || crossed_horizontal_line.is_some() {
        if crossed_vertical_line.is_some() && crossed_horizontal_line.is_some() {
            let crossed_vertical_line = crossed_vertical_line.unwrap();
            let crossed_horizontal_line = crossed_horizontal_line.unwrap();
            if crossed_vertical_line.vertex_a >= crossed_horizontal_line.direction_coordinate {
                return Some((next_position.0, crossed_vertical_line.vertex_a - 1));
            } else {
                return Some((next_position.0, crossed_horizontal_line.direction_coordinate - 1));
            }
        } else if crossed_vertical_line.is_some() {
            return Some((next_position.0, crossed_vertical_line.unwrap().vertex_a - 1));
        } else {
            return Some((next_position.0, crossed_horizontal_line.unwrap().direction_coordinate- 1));
        }
    }

    None
}

fn get_crossed_horizontal_line(mut horizontal_lines: Vec<&Line>) -> Option<Line> {
    if horizontal_lines.is_empty() {
        return None;
    }
    let highest = horizontal_lines.get(0).unwrap().direction_coordinate;
    horizontal_lines = horizontal_lines.into_iter().filter(|l| l.direction_coordinate <= highest).collect::<Vec<&Line>>();
    
    if horizontal_lines.len() == 1 {
        return Some(horizontal_lines.get(0).unwrap().clone().clone());
    }

    horizontal_lines.sort_by(|a, b| a.vertex_a.cmp(&b.vertex_a));
    
    let start = horizontal_lines.get(0).unwrap().vertex_a;
    let mut end = 0;

    for horizontal_line in horizontal_lines {
        let vertex_end = horizontal_line.vertex_b;
        if vertex_end > end {
            end = vertex_end;
        }
    }

    let line = Line {
        direction: Direction::Horizontal,
        direction_coordinate: highest,
        vertex_a: start,
        vertex_b: end
    };

    Some(line)
}

fn is_new_ref(moving_sand_grain: &MovingSandGrain) -> bool {
    if let distance = moving_sand_grain.previous_position.is_some() {
        let previous_position = moving_sand_grain.previous_position.as_ref().unwrap();
    };

    false
}