use std::{collections::HashMap, rc::Rc};

use regex::Regex;

#[derive(Debug)]
pub struct HydraulicNetwork {
    valves: Vec<Rc<Valve>>,
    openable_valve_count: i32
}

impl HydraulicNetwork {
    pub fn new(definition_of_hydraulic_network: Vec<String>) -> Self {
        let mut valves = vec![];
        let valves_information = HydraulicNetwork::parse(definition_of_hydraulic_network);
        let mut openable_valve_count = 0;

        for (valve_name, flow, connected_valves_list) in valves_information {
            if flow > 0 {
                openable_valve_count += 1;
            }
            valves.push(Rc::new(Valve::new(valve_name, flow, connected_valves_list)));
        }
        
        HydraulicNetwork { valves, openable_valve_count }
    }

    fn parse(definition_of_hydraulic_network: Vec<String>) -> Vec<(String, i32, Vec<String>)> {
        let mut valves_information = vec![];

        for definition_valve in definition_of_hydraulic_network {
            // example line:
            // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
            // valve_information : Valve AA has flow rate=0
            // name_of_connected_valves : tunnels lead to valves DD, II, BB
            let (valve_information, name_of_connected_valves) = definition_valve.split_once(";").unwrap();

            let regex = Regex::new(r"[A-Z]{2}").unwrap();
            let valve_name = regex.find_iter(valve_information).map(|m| m.as_str().to_owned()).collect::<Vec<String>>().pop().expect("Valve name found");

            let flow = *(&valve_information[(valve_information.find('=').unwrap() + 1)..].parse::<i32>().unwrap());

            let connected_valves_list = regex.find_iter(name_of_connected_valves).map(|m| m.as_str().to_owned()).collect::<Vec<String>>();

            valves_information.push((valve_name, flow, connected_valves_list));
        }

        valves_information
    }

    fn get_valve_by_name(&self, name: &str) -> Rc<Valve> {
        Rc::clone(self.valves.iter().find(|v| v.name == name).expect("Valve found"))
    }

    pub fn get_max_pressure_can_be_released_for_given_time(&self, name_of_first_valve: &str, time: i32) -> i32 {
        let first_valve = self.get_valve_by_name(name_of_first_valve);
        self.get_max_pressure(first_valve, time, Vec::new(), Vec::new(), "")
        // let mut pressures = vec![];

        // for connected_valve in &first_valve.borrow().name_of_connected_valves {
        //     let connected_valve = self.get_valve_by_name(&connected_valve);
        //     pressures.push(self.get_max_pressure(Rc::clone(&connected_valve), time, 0, &first_valve.borrow().name));
        // }
        
        
        // pressures.into_iter().max_by(|x, y| x.cmp(y)).unwrap()
    }

    fn get_max_pressure(&self, valve: Rc<Valve>, time: i32, current_sequence: Vec<String>, open_valves_name: Vec<String>, previous_valve_name: &str) -> i32 {
        if time == 0 || open_valves_name.len() as i32 == self.openable_valve_count || self.check_unnecessary_movement(&current_sequence) {
            return 0;
        }

        let mut possible_results_of_max_pressure = vec![];

        if valve.flow > 0 && !open_valves_name.contains(&valve.name) {
            let time = time - 1;
            let current_sequence = vec![];
            let mut open_valves_name = open_valves_name.clone();
            open_valves_name.push(valve.name.clone());
            let mut pressure = self.get_max_pressure(Rc::clone(&valve), time, current_sequence, open_valves_name, "");
            pressure += valve.flow * time;
            possible_results_of_max_pressure.push(pressure);
        }

        for connected_valve in &valve.name_of_connected_valves {
            if connected_valve == previous_valve_name {
                continue;
            }
            let time = time - 1;
            let mut current_sequence = current_sequence.clone();
            current_sequence.push(connected_valve.clone());
            let next_valve = self.get_valve_by_name(connected_valve);
            possible_results_of_max_pressure.push(self.get_max_pressure(next_valve, time, current_sequence, open_valves_name.clone(), &valve.name));
        }

        possible_results_of_max_pressure.into_iter().max().unwrap_or_default()

        // let (name, flow, is_open, connected_valves);
        // {
        //     let valve = valve.borrow();
        //     name = valve.name.clone();
        //     flow = valve.flow;
        //     is_open = valve.is_open;
        //     connected_valves = valve.name_of_connected_valves.clone();
        // }

        // if !is_open {
        //     valve.borrow_mut().is_open = true;
        //     pressure_released += 8;
        // }


        // pressure_released
    }

    fn check_unnecessary_movement(&self, current_sequence: &Vec<String>) -> bool {
        let current_sequence_length = current_sequence.len();
        if current_sequence_length < 6 {
            false
        } else {
            let mut current_sequence = current_sequence.clone();
            current_sequence.reverse();
            let three_last_movements = &current_sequence[..3];

            for (index, _) in current_sequence.iter().enumerate().skip(3) {
                if index + 3 > current_sequence_length {
                    return false;
                }
                let previous_movements = &current_sequence[index..(index + 3)];
                if previous_movements == three_last_movements {
                    return true;
                }
            }
            false
        }
    }
}

#[derive(Debug)]
pub struct Valve {
    name: String,
    flow: i32,
    name_of_connected_valves: Vec<String>
}

impl Valve {
    pub fn new(name: String, flow: i32, name_of_connected_valves: Vec<String>) -> Self {
        Self { name, flow, name_of_connected_valves }
    }
}