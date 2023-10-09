use std::{collections::HashMap, rc::{Rc, Weak}, cell::RefCell, fmt};

use regex::Regex;

#[derive(Debug)]
pub struct HydraulicNetwork {
    valves: Vec<Rc<Valve>>,
    openable_valve_count: i32,
    nodes: Vec<Rc<RefCell<Node>>>,
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

        let nodes = Node::new_pipeline(&valves);


        
        HydraulicNetwork { valves, openable_valve_count, nodes }
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

struct Node {
    valve: Rc<Valve>,
    nodes: Vec<Weak<RefCell<Node>>>,
    openable_valves: Option<HashMap<String, u32>>,
    open_valves: Option<HashMap<String, u32>>,
}

impl Node {
    pub fn new_pipeline(valves: &Vec<Rc<Valve>>) -> Vec<Rc<RefCell<Node>>> {
        let nodes = Node::init_nodes(valves);
        Node::add_connections(&nodes);
        nodes
    }

    fn init_nodes(valves: &Vec<Rc<Valve>>) -> Vec<Rc<RefCell<Node>>> {
        let mut nodes = HashMap::new();
        let check_node_already_connected = |x: &Weak<RefCell<Node>>, y: &str| -> bool {
            match x.upgrade() {
                None => false,
                Some(n) => n.borrow().valve.name == y
            }
        };

        for valve in valves {
            let node = match nodes.get(&valve.name) {
                Some(n) => Rc::clone(n),
                None => {
                    let n = Node::new(valve);
                    nodes.insert(&valve.name, Rc::clone(&n));
                    n
                }
            };
            

            for name_of_connected_valve in &valve.name_of_connected_valves {
                let child_node = match nodes.get(name_of_connected_valve) {
                    Some(n) => Rc::clone(n),
                    None => {
                        let connected_valve = valves.iter().find(|v| v.name == *name_of_connected_valve).unwrap();
                        let n = Node::new(connected_valve);
                        nodes.insert(name_of_connected_valve, Rc::clone(&n));
                        n
                    }
                };

                let node_has_child = node.borrow().nodes.iter().any(|n| check_node_already_connected(n, name_of_connected_valve));
                if !node_has_child {
                    node.borrow_mut().nodes.push(Rc::downgrade(&child_node));
                }

                // let child_has_parent = child_node.borrow().nodes.iter().any(|n| &n.upgrade().unwrap().borrow().valve.name == &valve.name);
                let child_has_parent = child_node.borrow().nodes.iter().any(|n| check_node_already_connected(n, &valve.name));
                if !child_has_parent {
                    child_node.borrow_mut().nodes.push(Rc::downgrade(&node));
                }
            }
        }

        nodes.values().cloned().collect()
    }

    fn new(valve: &Rc<Valve>) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { valve: Rc::clone(&valve), nodes: vec![], openable_valves: None, open_valves: None }))
    }

    fn add_connections(nodes: &Vec<Rc<RefCell<Node>>>) {

        loop {
            let mut node_has_been_edited = false;
            
            for ref_node in nodes {
                // regarder si le noeud dispose de vanne(s) ouvrable(s) accessible(s)
                //      si oui, pour chacune, ajouter aux vannes connectées cette vanne ouvrable accessible, sous condition :
                //          - que la vanne connectée ne soit pas la vanne ouvrable accessible
                //          - que la vanne connectée ne dispose pas déjà de cette vanne, ou si elle dispose ne conserver que la plus petite distance

                // ensuite, 
                // regarder si le noeud lui même dispose d'une vanne ouvrable
                //      si oui, ajouter aux vannes connectées cette vanne ouvrable accessible, sous condition :
                //          - que la vanne connectée ne dispose pas déjà de cette vanne, ou si elle dispose ne conserver que la plus petite distance

                // dans tous les cas, quand une condition est remplie on passe node_has_been_edited à true
                let openable_valves = {
                    let borrow_node = ref_node.borrow();
                    borrow_node.openable_valves.clone()
                };

                if openable_valves.is_some() {

                    let ref_children_nodes = &ref_node.borrow().nodes;
                    let child_node_has_been_edited = Node::add_openable_valves_to_children_nodes(ref_children_nodes, openable_valves.unwrap());
                    
                    if child_node_has_been_edited {
                        node_has_been_edited = true;
                    }

                    // Origin
                    // let ref_children_nodes = &ref_node.borrow().nodes;
                    // for weak_ref_child_node in ref_children_nodes {
                    //     let child_node_has_been_edited = match weak_ref_child_node.upgrade() {
                    //         None => false,
                    //         Some(ref_child_node) => {
                    //             let mut borrow_mut_child_node = ref_child_node.borrow_mut();
                    //             borrow_mut_child_node.add_openable_valves(openable_valves.as_ref().unwrap())
                    //         }                            
                    //     };

                    //     if child_node_has_been_edited {
                    //         node_has_been_edited = true;
                    //     }
                    // }
                }


                let flow = {
                    let borrow_node = ref_node.borrow();
                    borrow_node.valve.flow
                };

                if flow > 0 {
                    let valve_name = {
                        let borrow_node = ref_node.borrow();
                        borrow_node.valve.name.clone()
                    };
                    let mut open = HashMap::new();
                    open.insert(valve_name, 0);

                    let ref_children_nodes = &ref_node.borrow().nodes;
                    let child_node_has_been_edited = Node::add_openable_valves_to_children_nodes(ref_children_nodes, open);
                    
                    if child_node_has_been_edited {
                        node_has_been_edited = true;
                    }
                }
            }


            if !node_has_been_edited {
                break;
            }
        }
    }

    fn add_openable_valves_to_children_nodes(ref_children_nodes: &Vec<Weak<RefCell<Node>>>, openable_valves: HashMap<String, u32>) -> bool {
        let mut edited = false;
        
        for weak_ref_child_node in ref_children_nodes {
            let child_node_has_been_edited = match weak_ref_child_node.upgrade() {
                None => false,
                Some(ref_child_node) => {
                    let mut borrow_mut_child_node = ref_child_node.borrow_mut();
                    borrow_mut_child_node.add_openable_valves(&openable_valves)
                }                            
            };
    
            if child_node_has_been_edited {
                edited = true;
            }
        }

        edited
    }

    fn add_openable_valves(&mut self, openable_valves: &HashMap<String, u32>) -> bool {
        let mut has_added = false;

        let self_openable_valves = self.openable_valves.clone().unwrap_or_default();
        let self_valve_name = &self.valve.name;

        for (valve_name, distance) in openable_valves {
            let distance = *distance + 1;
            if self_valve_name == valve_name
                ||
                (self_openable_valves.contains_key(valve_name) && self_openable_valves.get(valve_name).unwrap() <= &distance) 
            {
                continue;
            }
            match &mut self.openable_valves {
                Some(v_d) => {
                    v_d.insert(valve_name.clone(), distance);
                },
                None => {
                    let mut open = HashMap::new();
                    open.insert(valve_name.clone(), distance);
                    self.openable_valves = Some(open)
                }
            }
            has_added = true;
        }

        has_added
    }
}


impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
        .field("valve name", &self.valve.name)
        // .field("nodes", &self.nodes.iter().for_each(|n| {
        //     match n.upgrade() {
        //         Some(v) => {
        //             let a = &v.borrow().valve.name;
        //             print!("{a}");
        //         },
        //         None => ()
        //     }
        // }))
        .field("openable_valves", &self.openable_valves)
        // .field("open_valves", &self.open_valves)
        .finish()
    }
}

#[derive(Debug)]
struct Valve {
    name: String,
    flow: i32,
    name_of_connected_valves: Vec<String>
}

impl Valve {
    pub fn new(name: String, flow: i32, name_of_connected_valves: Vec<String>) -> Self {
        Self { name, flow, name_of_connected_valves }
    }
}