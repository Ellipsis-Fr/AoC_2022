use std::collections::HashSet;

use AoC_2022::text_file_reader::TextFileReader;
use device::{Sensor, Point};

mod device;

fn main() {
    println!("Puzzle du 15/12 Partie 1");
    
    let puzzle = get_puzzle();
    let sensors = get_sensors(puzzle);
    // println!("Sensors : {:?}", sensors);
    
    let coordinates_of_all_signals = get_coordinates_of_all_signals(&sensors);
    // println!("coordinates_of_all_signals : {:?}", coordinates_of_all_signals);
    
    let row = 2_000_000;
    let coordinate_count_that_cannot_contain_a_beacon_in_a_row = get_coordinate_count_that_cannot_contain_a_beacon_in_a_row(&sensors, coordinates_of_all_signals, row);
    println!("count of positions that cannot contain a beacon in row {row} : {coordinate_count_that_cannot_contain_a_beacon_in_a_row}");

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

fn get_coordinates_of_all_signals(sensors: &Vec<Sensor>) -> HashSet<Point> {
    let mut signal_coordinates = HashSet::new();

    for sensor in sensors {
        let distance_between_sensor_and_beacon = sensor.get_position().get_manhattan_distance(sensor.get_beacon_position());
        println!("distance : {distance_between_sensor_and_beacon}");
        signal_coordinates.extend(&sensor.get_position().get_all_point_from_a_perimeter(distance_between_sensor_and_beacon));
    }


    signal_coordinates
}

fn get_coordinate_count_that_cannot_contain_a_beacon_in_a_row(sensors: &Vec<Sensor>, coordinates_of_all_signals: HashSet<Point>, row: i64) -> usize {
    let mut sensors_or_beacon_in_a_row = HashSet::new();

    for sensor in sensors {
        if sensor.get_position().1 == row {
            sensors_or_beacon_in_a_row.insert(sensor.get_position());
        }
        if sensor.get_beacon_position().1 == row {
            sensors_or_beacon_in_a_row.insert(sensor.get_beacon_position());
        }
    }

    let coordinates_occupied_in_a_row = coordinates_of_all_signals.iter().filter(|coordinate| coordinate.1 == row).collect::<HashSet<&Point>>();
    coordinates_occupied_in_a_row.len() - sensors_or_beacon_in_a_row.len()
}