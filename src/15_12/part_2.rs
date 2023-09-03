use std::collections::{HashSet, VecDeque};

use AoC_2022::text_file_reader::TextFileReader;
use device::{Sensor, Point};
use num::{integer::sqrt, pow};

mod device;

fn main() {
    println!("Puzzle du 15/12 Partie 2");
    
    let puzzle = get_puzzle();
    let sensors = get_sensors(puzzle);
    
    let row = 2_000_000;
    let distress_beacon_coordinates = get_distress_beacon_coordinates(sensors, row);
    if distress_beacon_coordinates.is_some() {
        let distress_beacon_coordinates = distress_beacon_coordinates.unwrap();

        println!("Distress beacon coordinates : ({:?}, {:?}),\nTuning frequency : {:?}", distress_beacon_coordinates.0, distress_beacon_coordinates.1, distress_beacon_coordinates.0 * 4_000_000 + distress_beacon_coordinates.1);
    }
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("15_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn get_sensors(puzzle: Vec<String>) -> Vec<Sensor> {
    let mut sensors = vec![];
    
    for p in puzzle {
        sensors.push(Sensor::new(&p));
    }

    sensors
}

fn get_distress_beacon_coordinates(sensors: Vec<Sensor>, row: i64) -> Option<Point> {
    let distress_beacon_coordinates;
    let mut shift = 0;

    loop {
        if shift > 4_000_000 {
            println!("Aucune position possible trouvée");
            distress_beacon_coordinates = None;
            break;
        }

        let mut available_coordinates = get_available_coordinates(&sensors, row, shift);
        
        if !available_coordinates.is_empty() {
            if available_coordinates.len() > 1 {
                println!("comportement étonnant, car plusieurs positions possibles...");
                println!("{:?}", available_coordinates);
            }
            distress_beacon_coordinates = available_coordinates.pop();
            break;
        }
        shift += 1;
    }

    distress_beacon_coordinates
}

fn get_available_coordinates(sensors: &Vec<Sensor>, row: i64, shift: i64) -> Vec<Point> {
    let limits = (0, 4_000_000);
    let mut available_coordinates = vec![];

    for i in 0..2 {
        let y = if i == 0 {
            row - shift
        } else {
            row + shift
        };

        let mut sensors_or_beacon_in_a_row = HashSet::new();
        
        for sensor in sensors {
            if sensor.get_position().1 == y {
                sensors_or_beacon_in_a_row.insert(sensor.get_position().0);
            }
            if sensor.get_beacon_position().1 == y {
                sensors_or_beacon_in_a_row.insert(sensor.get_beacon_position().0);
            }
        }
        
        let mut coordinates_occupied_in_a_row: HashSet<i64> = HashSet::new();
        for sensor in sensors {
            let center = sensor.get_position();
            let radius = sensor.get_distance();
    
            if (center.1 - radius) <= y && y <= (center.1 + radius) {
                let coordinates_occupied_in_this_perimeter = get_coordinates_from_a_perimeter(center, radius, y);
                coordinates_occupied_in_a_row.extend(coordinates_occupied_in_this_perimeter.iter())
            }
        }

        let mut coordinates_occupied: HashSet<i64> = HashSet::new();
        coordinates_occupied.extend(sensors_or_beacon_in_a_row.iter());
        coordinates_occupied.extend(coordinates_occupied_in_a_row.iter());

        println!("at y : {}, coordinates_occupied before filter it {:?}", y, coordinates_occupied.len());
        coordinates_occupied = coordinates_occupied.into_iter().filter(|x| *x >= limits.0 && *x <= limits.1).collect();
        println!("at y : {}, coordinates_occupied {:?}", y, coordinates_occupied.len());
        if coordinates_occupied.len() < (limits.1 + 1) as usize {
            let x_coordinates = (0..=4_000_000).into_iter().collect::<Vec<i64>>();
            let x_available_coordinates = x_coordinates.into_iter().filter(|x| !coordinates_occupied.contains(x)).collect::<Vec<i64>>();
            
            for x in x_available_coordinates {
                available_coordinates.push(Point(x, y));
            }
        }

        if shift == 0 {
            break;
        }
    }

    available_coordinates
}

fn get_coordinates_from_a_perimeter(center: &Point, radius: i64, row: i64) -> VecDeque<i64> {
    let mut coordinates = VecDeque::new();
    let min_x = center.0 - radius;
    let max_x = center.0 + radius;

    for x in min_x..=max_x {
        coordinates.push_back(x);
    }

    if row == center.1 {
        return coordinates;
    }

    let shift = sqrt(pow(center.1 - row, 2));
    for _ in 0..shift {
        coordinates.pop_front();
        coordinates.pop_back();
    }

    coordinates
}