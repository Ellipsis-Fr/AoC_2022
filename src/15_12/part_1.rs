use std::collections::{HashSet, VecDeque};

use AoC_2022::text_file_reader::TextFileReader;
use device::{Sensor, Point};
use num::{integer::sqrt, pow};

mod device;

fn main() {
    println!("Puzzle du 15/12 Partie 1");
    
    let puzzle = get_puzzle();
    let sensors = get_sensors(puzzle);
    
    let row = 2_000_000;
    let coordinate_count_that_cannot_contain_a_beacon_in_a_row = get_coordinate_count_that_cannot_contain_a_beacon_in_a_row(sensors, row);
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

fn get_coordinate_count_that_cannot_contain_a_beacon_in_a_row(sensors: Vec<Sensor>, row: i64) -> usize {
    let mut sensors_or_beacon_in_a_row = HashSet::new();
    
    for sensor in &sensors {
        if sensor.get_position().1 == row {
            sensors_or_beacon_in_a_row.insert(sensor.get_position().0);
        }
        if sensor.get_beacon_position().1 == row {
            sensors_or_beacon_in_a_row.insert(sensor.get_beacon_position().0);
        }
    }
    
    let mut coordinates_occupied_in_a_row: HashSet<i64> = HashSet::new();
    for sensor in &sensors {
        let center = sensor.get_position();
        let radius = sensor.get_distance();

        if (center.1 - radius) <= row && row <= (center.1 + radius) {
            let coordinates_occupied_in_this_perimeter = get_coordinates_from_a_perimeter(center, radius, row);
            coordinates_occupied_in_a_row.extend(coordinates_occupied_in_this_perimeter.iter())
        }
    }

    coordinates_occupied_in_a_row.len() - sensors_or_beacon_in_a_row.len()
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