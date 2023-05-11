use std::{collections::HashSet, cell::RefCell};

use AoC_2022::text_file_reader::TextFileReader;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn add(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
    }

    fn diff(&mut self, other: &Point) -> Point {
        Point {
            x: (self.x - other.x).abs(),
            y: (self.y - other.y).abs()
        }
    }

    fn isAdjacentTo(&self, other: &Point) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1 
    }
}

fn main() {
    println!("Puzzle du 09/12 Partie 2");
    
    let puzzle = get_puzzle();
    let points_visited_by_tail = get_all_points_visited_by_tail(puzzle);
    println!("Nombre de positions visitées au moins une fois : {}", points_visited_by_tail.len());
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("09_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn get_all_points_visited_by_tail(puzzle: Vec<String>) -> HashSet<Point> {
    let rope = get_rope();

    let mut points = HashSet::new();
    points.insert(Point {x: 0, y: 0});

    for movement in puzzle {
        let (direction, repetition) = define_movement(movement);

        for _ in 0..repetition {
            let mut ref_mut_rope = rope.borrow_mut();
            let head = ref_mut_rope.get_mut(0).unwrap();
            head.add(&direction);

            let mut clone_head = head.clone();

            for i in 1..10 {
                let actual_piece_of_rope =  ref_mut_rope.get_mut(i).unwrap();
                
                if !actual_piece_of_rope.isAdjacentTo(&clone_head) {
                    move_piece_of_rope(actual_piece_of_rope, &clone_head);
                    clone_head = actual_piece_of_rope.clone();
                    if i == 9 { points.insert(actual_piece_of_rope.clone()); }
                } else { break; }
            }

        }
    }


    points
}

fn get_rope() -> RefCell<Vec<Point>> {
    let mut rope = RefCell::new(vec![]);
    
    for _ in 0..10 {
        rope.borrow_mut().push(Point {x: 0, y: 0});
    }

    rope
}

fn define_movement(movement: String) -> (Point, u32) {
    let detail = movement.split_whitespace().collect::<Vec<&str>>();
    (get_direction(detail[0]), detail[1].parse::<u32>().unwrap())
}

fn get_direction(direction_str: &str) -> Point {
    let point;
    match direction_str {
        "L" => point = Point {x: -1, y: 0},
        "R" => point = Point {x: 1, y: 0},
        "U" => point = Point {x: 0, y: 1},
        "D" => point = Point {x: 0, y: -1},
        _ => panic!("Direction non reconnue : {}", direction_str)
    }

    point
}

fn move_piece_of_rope(actual_piece_of_rope: &mut Point, clone_head: &Point) {
    let diff = actual_piece_of_rope.diff(&clone_head);
    if diff.x == 0 || diff.y == 0 { 
        if diff.x == 0 {
            let point = if clone_head.y - actual_piece_of_rope.y > 0 {
                Point {x: 0, y: 1}
            } else {
                Point {x: 0, y: -1}
            };
            actual_piece_of_rope.add(&point); 
        } else {
            let point = if clone_head.x - actual_piece_of_rope.x > 0 {
                Point {x: 1, y: 0}
            } else {
                Point {x: -1, y: 0}
            };
            actual_piece_of_rope.add(&point); 
        }
    }
    else {
        let dx = match (clone_head.x - actual_piece_of_rope.x).is_negative() {
            true => -1,
            false => 1
        };

        let dy = match (clone_head.y - actual_piece_of_rope.y).is_negative() {
            true => -1,
            false => 1
        };
        actual_piece_of_rope.add(&Point { x: dx, y: dy });
    }
}