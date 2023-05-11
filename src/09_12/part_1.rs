use std::collections::HashSet;

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
    println!("Puzzle du 09/12 Partie 1");
    
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
    let mut head = Point {x: 0, y: 0};
    let mut tail = head.clone();

    let mut points = HashSet::new();
    points.insert(tail.clone());

    for movement in puzzle {
        let (direction, repetition) = define_movement(movement);

        for _ in 0..repetition {
            head.add(&direction);

            if !tail.isAdjacentTo(&head) {
                let diff = tail.diff(&head);
                if diff.x == 0 || diff.y == 0 { tail.add(&direction); }
                else {
                    let dx = match (head.x - tail.x).is_negative() {
                        true => -1,
                        false => 1
                    };

                    let dy = match (head.y - tail.y).is_negative() {
                        true => -1,
                        false => 1
                    };

                    tail.add(&Point { x: dx, y: dy });
                }
                points.insert(tail.clone());
            }
        }
    }


    points
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