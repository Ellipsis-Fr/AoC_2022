use std::collections::{HashSet, VecDeque};

use AoC_2022::text_file_reader::TextFileReader;
use device::{Sensor, Point};
use num::{integer::sqrt, pow};

mod device;

fn main() {
    println!("Puzzle du 15/12 Partie 2");
    
    let puzzle = get_puzzle();
    let sensors = get_sensors(puzzle);

    let limit_x = (0, 4_000_000);
    let limit_y = (0, 4_000_000);

    let distress_beacon_coordinates = get_distress_beacon_coordinates(&sensors, limit_x, limit_y);
    match distress_beacon_coordinates {
        Some(d) => println!("Distress beacon coordinates : ({:?}, {:?}),\nTuning frequency : {:?}", d.0, d.1, d.0 * 4_000_000 + d.1),
        None => println!("No coordinates found")
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
    let mut distress_beacon_coordinates = None;
    const STEP_AWAY: i64 = 1;
    
    'row: for y in limits_y.0..=limits_y.1 {
        let mut next_x = limits_x.0;        
        'column: loop {
            if next_x > limits_x.1 {
                break;
            }
            let x = next_x;

            for sensor in sensors {
                let sensor_position = sensor.get_position();
                let radius = sensor.get_distance();
                let manhattan_distance = sensor_position.get_manhattan_distance(&Point(x, y));
                
                if manhattan_distance > radius {
                    continue;
                } else if manhattan_distance == radius {
                    if x >= sensor_position.0 {
                        next_x = STEP_AWAY + x;
                        continue 'column;
                    } else {
                        let dx = (sensor_position.0 - x).abs();
                        let dy = (sensor_position.1 - y).abs();
                        next_x = STEP_AWAY + x + dx + radius - dy;
                        continue 'column;
                    }
                } else {
                    if x >= sensor_position.0 {
                        let dx = (sensor_position.0 - x).abs();
                        let dy = (sensor_position.1 - y).abs();
                        next_x = STEP_AWAY + x + radius - (dx + dy);
                        continue 'column;
                    } else {
                        let dx = (sensor_position.0 - x).abs();
                        let dy = (sensor_position.1 - y).abs();
                        next_x = STEP_AWAY + x + dx + radius - dy;
                        continue 'column;
                    }
                }
            }

            distress_beacon_coordinates = Some(Point(x, y));
            break 'row;
        }
    }
    
    distress_beacon_coordinates
}