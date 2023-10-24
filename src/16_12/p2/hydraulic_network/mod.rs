use std::{collections::HashMap, rc::Rc, cell::RefCell, fmt, sync::{Arc, Weak, Mutex}, thread, clone};

use regex::Regex;

#[derive(Debug)]
pub struct HydraulicNetwork {
    nodes: Vec<Arc<Mutex<Node>>>,
    count_openable_valves: u32
}

impl HydraulicNetwork {
    pub fn new(definition_of_hydraulic_network: Vec<String>) -> Self {
        let mut valves = vec![];
        let valves_information = HydraulicNetwork::parse(definition_of_hydraulic_network);
        let mut count_openable_valves = 0;

        for (valve_name, flow, connected_valves_list) in valves_information {
            if flow > 0 {
                count_openable_valves += 1;
            }
            valves.push(Arc::new(Valve::new(valve_name, flow, connected_valves_list)));
        }

        let nodes = Node::new_pipeline(valves);

        HydraulicNetwork { nodes, count_openable_valves }
    }

    fn parse(definition_of_hydraulic_network: Vec<String>) -> Vec<(String, u32, Vec<String>)> {
        let mut valves_information = vec![];

        for definition_valve in definition_of_hydraulic_network {
            // example line:
            // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
            // valve_information : Valve AA has flow rate=0
            // name_of_connected_nodes_with_list_of_interesting_openable_valve : tunnels lead to valves DD, II, BB
            let (valve_information, name_of_connected_nodes_with_list_of_interesting_openable_valve) = definition_valve.split_once(";").unwrap();

            let regex = Regex::new(r"[A-Z]{2}").unwrap();
            let valve_name = regex.find_iter(valve_information).map(|m| m.as_str().to_owned()).collect::<Vec<String>>().pop().expect("Valve name found");

            let flow = *(&valve_information[(valve_information.find('=').unwrap() + 1)..].parse::<u32>().unwrap());

            let connected_valves_list = regex.find_iter(name_of_connected_nodes_with_list_of_interesting_openable_valve).map(|m| m.as_str().to_owned()).collect::<Vec<String>>();

            valves_information.push((valve_name, flow, connected_valves_list));
        }

        valves_information
    }

    fn get_node_by_valve_name(&self, name: &str) -> Arc<Mutex<Node>> {
        Arc::clone(self.nodes.iter().find(|v| v.lock().unwrap().valve.name == name).expect("Node found"))
    }

    //Autre idée
    pub fn get_max_pressure_can_be_released_for_given_time(&self, name_of_first_valve: &str, time: u32) -> u32 {
        let node_first_valve = self.get_node_by_valve_name(name_of_first_valve);

        let mut possible_results_of_max_pressure = vec![];
        let count_levels_to_go_through = self.count_openable_valves / 2;

        for i in 0..=count_levels_to_go_through {
            possible_results_of_max_pressure.push(self.get_max_pressure(Arc::clone(&node_first_valve), time, i, "M".to_string()));
        }

        possible_results_of_max_pressure.into_iter().max().unwrap_or_default()
    }

    
    fn get_max_pressure(&self, node: Arc<Mutex<Node>>, time: u32, level: u32, role: String) -> u32 {
        if time == 0 {
            return 0;
        }


        let mut possible_results_of_max_pressure = vec![];
        let node_openable_valves = {
            let lock_node = node.lock().unwrap();
            lock_node.openable_valves.clone()
        };


        for (openable_valve_name, openable_valve_distance) in node_openable_valves {
            let time_consumed = match time.checked_sub(openable_valve_distance + 1) {
                None => continue,
                Some(v) => v
            };

            self.open_or_close_valve_in_nodes(&openable_valve_name, 0);
            let next_node = self.get_node_by_valve_name(&openable_valve_name);
            let next_valve_flow = {
                let lock_node = next_node.lock().unwrap();
                lock_node.valve.flow
            };
            let mut pressure = time_consumed * next_valve_flow;

            if role == "M".to_string() {
                if level == 0 {
                    pressure += self.get_max_pressure(Arc::clone(&self.get_node_by_valve_name("AA")), 26, 0, "S".to_string());
                } else {
                    pressure += self.get_max_pressure(Arc::clone(&next_node), time_consumed, level - 1, role.clone());
                }
            } else {
                pressure += self.get_max_pressure(Arc::clone(&next_node), time_consumed, 0, role.clone());
            }

            self.open_or_close_valve_in_nodes(&openable_valve_name, 1);
            possible_results_of_max_pressure.push(pressure);
        }

        possible_results_of_max_pressure.into_iter().max().unwrap_or_default()
    }

    fn open_or_close_valve_in_nodes(&self, valve_name: &str, state: u8) {
        for node in &self.nodes {
            let mut mutable_node = node.lock().unwrap();

            match state {
                0 => mutable_node.close_valve(valve_name),
                1 => mutable_node.open_valve(valve_name),
                _ => panic!("Inconsistent state")
            }
        }
    }
}

struct Node {
    valve: Arc<Valve>,
    nodes: Vec<Weak<Mutex<Node>>>,
    openable_valves: HashMap<String, u32>,
    closed_valves: HashMap<String, u32>,
}

impl Node {
    pub fn new_pipeline(valves: Vec<Arc<Valve>>) -> Vec<Arc<Mutex<Node>>> {
        let nodes = Node::init_nodes(valves);
        Node::add_connections(&nodes);
        nodes
    }

    fn init_nodes(valves: Vec<Arc<Valve>>) -> Vec<Arc<Mutex<Node>>> {
        let mut nodes = HashMap::new();
        let check_node_already_connected = |x: &Weak<Mutex<Node>>, y: &str| -> bool {
            match x.upgrade() {
                None => false,
                Some(n) => n.lock().unwrap().valve.name == y
            }
        };

        for valve in &valves {
            let node = match nodes.get(&valve.name) {
                Some(n) => Arc::clone(n),
                None => {
                    let n = Node::new(&valve);
                    nodes.insert(&valve.name, Arc::clone(&n));
                    n
                }
            };
            

            for name_of_connected_valve in &valve.name_of_connected_valves {
                let child_node = match nodes.get(name_of_connected_valve) {
                    Some(n) => Arc::clone(n),
                    None => {
                        let connected_valve = valves.iter().find(|v| v.name == *name_of_connected_valve).unwrap();
                        let n = Node::new(connected_valve);
                        nodes.insert(name_of_connected_valve, Arc::clone(&n));
                        n
                    }
                };

                let node_has_child = node.lock().unwrap().nodes.iter().any(|n| check_node_already_connected(n, name_of_connected_valve));
                if !node_has_child {
                    node.lock().unwrap().nodes.push(Arc::downgrade(&child_node));
                }

                // let child_has_parent = child_node.borrow().nodes.iter().any(|n| &n.upgrade().unwrap().borrow().valve.name == &valve.name);
                let child_has_parent = child_node.lock().unwrap().nodes.iter().any(|n| check_node_already_connected(n, &valve.name));
                if !child_has_parent {
                    child_node.lock().unwrap().nodes.push(Arc::downgrade(&node));
                }
            }
        }

        nodes.values().cloned().collect()
    }

    fn new(valve: &Arc<Valve>) -> Arc<Mutex<Node>> {
        Arc::new(Mutex::new(Node { valve: Arc::clone(&valve), nodes: vec![], openable_valves: HashMap::new(), closed_valves: HashMap::new() }))
    }

    fn add_connections(nodes: &Vec<Arc<Mutex<Node>>>) {

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
                    let borrow_node = ref_node.lock().unwrap();
                    borrow_node.openable_valves.clone()
                };

                if !openable_valves.is_empty() {

                    let ref_children_nodes = &ref_node.lock().unwrap().nodes;
                    let child_node_has_been_edited = Node::add_openable_valves_to_children_nodes(ref_children_nodes, openable_valves);
                    
                    if child_node_has_been_edited {
                        node_has_been_edited = true;
                    }
                }


                let flow = {
                    let borrow_node = ref_node.lock().unwrap();
                    borrow_node.valve.flow
                };

                if flow > 0 {
                    let valve_name = {
                        let borrow_node = ref_node.lock().unwrap();
                        borrow_node.valve.name.clone()
                    };
                    let mut open = HashMap::new();
                    open.insert(valve_name, 0);

                    let ref_children_nodes = &ref_node.lock().unwrap().nodes;
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

    fn add_openable_valves_to_children_nodes(ref_children_nodes: &Vec<Weak<Mutex<Node>>>, openable_valves: HashMap<String, u32>) -> bool {
        let mut edited = false;
        
        for weak_ref_child_node in ref_children_nodes {
            let child_node_has_been_edited = match weak_ref_child_node.upgrade() {
                None => false,
                Some(ref_child_node) => {
                    let mut borrow_mut_child_node = ref_child_node.lock().unwrap();
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

        let self_openable_valves = &mut self.openable_valves;
        let self_valve_name = &self.valve.name;

        for (valve_name, distance) in openable_valves {
            let distance = if self_valve_name == valve_name {
                *distance - 1
            } else {
                *distance + 1
            };

            if self_openable_valves.contains_key(valve_name) && self_openable_valves.get(valve_name).unwrap() <= &distance {
                continue;
            }
            self_openable_valves.insert(valve_name.clone(), distance);
            has_added = true;
        }

        has_added
    }

    fn close_valve(&mut self, valve_name: &str) {
        let distance = self.openable_valves.remove(valve_name).unwrap();
        self.closed_valves.insert(valve_name.to_string(), distance);
    }

    fn open_valve(&mut self, valve_name: &str) {
        let distance = self.closed_valves.remove(valve_name).unwrap();
        self.openable_valves.insert(valve_name.to_string(), distance);
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
    flow: u32,
    name_of_connected_valves: Vec<String>
}

impl Valve {
    pub fn new(name: String, flow: u32, name_of_connected_valves: Vec<String>) -> Self {
        Self { name, flow, name_of_connected_valves }
    }
}