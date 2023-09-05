use std::collections::{HashSet, VecDeque};

use AoC_2022::text_file_reader::TextFileReader;
use device::{Sensor, Point};
use num::{integer::sqrt, pow};

mod device;

fn main() {
    println!("Puzzle du 15/12 Partie 2");
    
    let puzzle = get_puzzle();
    let sensors = get_sensors(puzzle);
    
    let limit_x = (0, 20);
    let limit_y = (0, 20);

    let distress_beacon_coordinates = get_distress_beacon_coordinates(&sensors, limit_x, limit_y);
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

fn get_distress_beacon_coordinates(sensors: &Vec<Sensor>, limits_x: (i64, i64), limits_y: (i64, i64)) -> Option<Point> {
    let dx = 1 + limits_x.1 - limits_x.0;
    let dy = 1 + limits_y.1 - limits_y.0;
    
    // Map
    let rows = vec!['.'; dx as usize];
    let mut map = vec![rows; dy as usize];

    for sensor in sensors {
        let circle = sensor.get_position().get_all_point_from_a_perimeter(sensor.get_distance());

        for coordinate in circle {
            let x = coordinate.0;
            let y = coordinate.1;

            if x < limits_x.0 || x > limits_x.1 || y < limits_y.0 || y > limits_y.1 {
                continue;
            }

            // let a = map.get_mut(y as usize).unwrap().get_mut(x as usize).unwrap();
            // *a = 'X';
            *(map.get_mut(y as usize).unwrap().get_mut(x as usize).unwrap()) = 'X';
        }
        println!("Capteur fait");
    }

    let mut distress_beacon_coordinates = None;
    for (y, row) in map.iter().enumerate() {
        if row.contains(&'.') {
            let x = row.iter().position(|&r| r == '.').unwrap();
            distress_beacon_coordinates = Some(Point(x as i64, y as i64));
        }
    }

    distress_beacon_coordinates
}