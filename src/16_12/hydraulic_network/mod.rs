use std::{collections::{HashMap, HashSet}, rc::{Rc, Weak}, cell::RefCell, fmt};

use regex::{Regex, CaptureNames};

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
            // name_of_connected_nodes_with_list_of_interesting_openable_valve : tunnels lead to valves DD, II, BB
            let (valve_information, name_of_connected_nodes_with_list_of_interesting_openable_valve) = definition_valve.split_once(";").unwrap();

            let regex = Regex::new(r"[A-Z]{2}").unwrap();
            let valve_name = regex.find_iter(valve_information).map(|m| m.as_str().to_owned()).collect::<Vec<String>>().pop().expect("Valve name found");

            let flow = *(&valve_information[(valve_information.find('=').unwrap() + 1)..].parse::<i32>().unwrap());

            let connected_valves_list = regex.find_iter(name_of_connected_nodes_with_list_of_interesting_openable_valve).map(|m| m.as_str().to_owned()).collect::<Vec<String>>();

            valves_information.push((valve_name, flow, connected_valves_list));
        }

        valves_information
    }

    fn get_node_by_valve_name(&self, name: &str) -> Rc<RefCell<Node>> {
        Rc::clone(self.nodes.iter().find(|v| v.borrow().valve.name == name).expect("Node found"))
    }

    pub fn get_max_pressure_can_be_released_for_given_time(&self, name_of_first_valve: &str, time: i32) -> i32 {
        let node_first_valve = self.get_node_by_valve_name(name_of_first_valve);
        self.get_max_pressure(node_first_valve, time, Vec::new(), Vec::new(), "")
    }

    fn get_max_pressure(&self, node: Rc<RefCell<Node>>, time: i32, current_sequence: Vec<String>, open_valves_name: Vec<String>, previous_valve_name: &str) -> i32 {
        if time == 0 || open_valves_name.len() as i32 == self.openable_valve_count || self.check_unnecessary_movement(&current_sequence) {
            return 0;
        }

        let mut possible_results_of_max_pressure = vec![];

        let valve = Rc::clone(&node.borrow().valve);

        if valve.flow > 0 && !open_valves_name.contains(&valve.name) {
            let time = time - 1;
            let current_sequence = vec![];
            let mut open_valves_name = open_valves_name.clone();
            open_valves_name.push(valve.name.clone());
            self.open_or_close_valve_in_nodes(&valve.name, 0);
            let mut pressure = self.get_max_pressure(Rc::clone(&node), time, current_sequence, open_valves_name, "");
            self.open_or_close_valve_in_nodes(&valve.name, 1);
            pressure += valve.flow * time;
            possible_results_of_max_pressure.push(pressure);
        }

        let name_of_connected_nodes_with_list_of_interesting_openable_valve = self.get_name_of_connected_valves(Rc::clone(&node),  previous_valve_name);

        for connected_valve in name_of_connected_nodes_with_list_of_interesting_openable_valve {
            let time = time - 1;
            let mut current_sequence = current_sequence.clone();
            current_sequence.push(connected_valve.clone());
            let next_node = self.get_node_by_valve_name(&connected_valve);
            possible_results_of_max_pressure.push(self.get_max_pressure(next_node, time, current_sequence, open_valves_name.clone(), &valve.name));
        }

        possible_results_of_max_pressure.into_iter().max().unwrap_or_default()
    }

    fn open_or_close_valve_in_nodes(&self, valve_name: &str, state: u8) {
        for node in &self.nodes {
            let mut mutable_node = node.borrow_mut();
            if mutable_node.valve.name == valve_name {
                continue;
            }

            match state {
                0 => mutable_node.close_valve(valve_name),
                1 => mutable_node.open_valve(valve_name),
                _ => panic!("Inconsistent state")
            }
        }
    }

    fn get_name_of_connected_valves(&self, node: Rc<RefCell<Node>>, previous_valve_name: &str) -> Vec<String> {
        let mut name_of_connected_nodes_with_list_of_interesting_openable_valve: HashMap<String, HashMap<String, u32>> = HashMap::new();

        let node_openable_valves = &node.borrow().openable_valves;
        let node_connected_nodes = &node.borrow().nodes;
        let _node_valve_name = &node.borrow().valve.name;

        for connected_node in node_connected_nodes {
            let connected_node = connected_node.upgrade().unwrap();
            let connected_node_openable_valves = &connected_node.borrow().openable_valves;
            let connected_node_valve_name = &connected_node.borrow().valve.name;

            if connected_node_valve_name == previous_valve_name {
                continue;
            } else {
                let mut connected_node_openable_valves_interesting;
                if node_openable_valves.get(connected_node_valve_name).is_some() {
                    /*
                    si notre noeud actuel dispose du nom de la vanne connecté en tant que vanne ouvrable c'est que celle-ci est à un de distance,
                    exemple depuis D vers C, car C est une vanne avec un débit directement connectée à D, donc si cette vanne n'est pas encore ouverte il est forcément interessant d'étudier ce cheminement
                    
                    Avant ajout on vérifie si cette voie n'a pas déjà été inscrite par l'intermédiaire d'un autre noeud, si tel est le cas alors on supprime la voie référençant cette dernière car forcément plus longe
                    exemple depuis D vers C qui aurait constaté via A que C est accessible en 2 coups. Alors nous supprimons de A ce cheminement car plus long.
                    Si la voie ainsi supprimée fait que le noeud la référençant n'a plus d'autre voie on la supprime
                     */
                    connected_node_openable_valves_interesting = self.get_connected_node_openable_valves_interesting(&node_openable_valves, &connected_node_openable_valves);
                    connected_node_openable_valves_interesting.insert(connected_node_valve_name.clone(), 0);
                } else {
                    connected_node_openable_valves_interesting = self.get_connected_node_openable_valves_interesting(&node_openable_valves, connected_node_openable_valves);
                }

                if connected_node_openable_valves_interesting.is_empty() {
                    continue;
                }
                
                /*
                Ici on compare la liste des vannes accessibles depuis une autre vanne (sans débit, sinon serait tombé dans le 'if' précédent) avec celle déjà notée
                Exemple depuis A si B et D ouvert :
                dans la liste des voies possibles, pour B apparait déjà : C en 1 et E en 3. Là nous obtenons depuis D : C en 1 et E en 1
                donc nous conserverons l'accès vers B pour C mais y supprimerons la référence à E car D en a une de plus courte. Et si jamais B menait à C en + de 1 alors B aurait totalement disparu
                 */
                if name_of_connected_nodes_with_list_of_interesting_openable_valve.is_empty() {
                    name_of_connected_nodes_with_list_of_interesting_openable_valve.insert(connected_node_valve_name.clone(), connected_node_openable_valves_interesting);
                } else {
                    
                    let mut connected_node_openable_valves_interesting_after_comparison_with_list_of_nodes_with_interesting_openable_valve = HashMap::new();
                    'outer: for (next_valve_name_interesting, next_valve_distance_interesting) in connected_node_openable_valves_interesting.clone()  {
                        let temp_name_of_connected_nodes_with_list_of_interesting_openable_valve = name_of_connected_nodes_with_list_of_interesting_openable_valve.clone();
                        for (temp_name_of_connected_node, temp_next_valves_name_and_distance) in temp_name_of_connected_nodes_with_list_of_interesting_openable_valve {
                            if !temp_next_valves_name_and_distance.contains_key(&next_valve_name_interesting) {
                                /* 
                                "continue" car cela signifie donc que la potentielle voie intéressante disposée par ce connected_node n'existe pas pour ce noeud enregistré
                                dans la liste "name_of_connected_nodes_with_list_of_interesting_openable_valve", mais peut déjà l'être pour un autre noeud de cette liste 
                                */
                                continue;
                            }
                            
                            let temp_next_valve_distance = *temp_next_valves_name_and_distance.get(&next_valve_name_interesting).unwrap();
                            if temp_next_valve_distance < next_valve_distance_interesting {
                                /* 
                                "continue 'outer" car cela signifie donc que la potentielle voie intéressante disposée par ce connected_node est déjà la/une des raisons d'être
                                d'une vanne présente dans la liste et comme cette dernière propose un accès plus court alors cette voie n'est plus interressante du tout.
                                */
                                continue 'outer;
                            }
                            
                            if temp_next_valve_distance == next_valve_distance_interesting {
                                let temp_connected_valve_number_of_valves_reachable = temp_next_valves_name_and_distance.len();
                                let connected_node_openable_valves_interesting_sum = connected_node_openable_valves_interesting.len();

                                if temp_connected_valve_number_of_valves_reachable >= connected_node_openable_valves_interesting_sum {
                                    connected_node_openable_valves_interesting.remove(&next_valve_name_interesting);
                                    continue 'outer; // s'il est plus avantageux de ne pas ajouter cette nouvelle voie
                                }
                            }

                            /*
                            si j'arrive ici c'est qu'il faudra ajouter ce noeud et la valve associée à la liste "name_of_connected_nodes_with_list_of_interesting_openable_valve",  
                            donc, je dois supprimer de liste actuelle cette vanne
                            */
                            let next_valves_name_and_distance = name_of_connected_nodes_with_list_of_interesting_openable_valve.get_mut(&temp_name_of_connected_node).unwrap();
                            next_valves_name_and_distance.remove(&next_valve_name_interesting);
                            if next_valves_name_and_distance.is_empty() {
                                name_of_connected_nodes_with_list_of_interesting_openable_valve.remove(&temp_name_of_connected_node);
                            }

                            break;
                            // forcément terminé avec un "continue outer'" ou plutôt un "break" sur la boucle intérieur pour arriver à l'ajout...
                        }
                        /*
                        si j'arrive ici c'est que : 
                            - j'ai traversé toute la liste "name_of_connected_nodes_with_list_of_interesting_openable_valve" sans trouver une seule fois l'existence de la vanne intéressante, ou,
                            - je l'ai trouvé dans la liste comme étant accessible depuis un autre noeud mais pour une distance plus longue 
                        alors je l'ajoute
                         */
                        connected_node_openable_valves_interesting_after_comparison_with_list_of_nodes_with_interesting_openable_valve.insert(next_valve_name_interesting, next_valve_distance_interesting);
                    }

                    if connected_node_openable_valves_interesting_after_comparison_with_list_of_nodes_with_interesting_openable_valve.is_empty() {
                        continue;
                    }

                    name_of_connected_nodes_with_list_of_interesting_openable_valve.insert(connected_node_valve_name.clone(), connected_node_openable_valves_interesting_after_comparison_with_list_of_nodes_with_interesting_openable_valve);
                }
            } 
        }

        name_of_connected_nodes_with_list_of_interesting_openable_valve.keys().cloned().collect()
    }

    fn get_connected_node_openable_valves_interesting(&self, node_openable_valves: &HashMap<String, u32>, connected_node_openable_valves: &HashMap<String, u32>) -> HashMap<String, u32> {
        let mut connected_node_openable_valves_interesting = HashMap::new();
        
        for (node_opennable_valve_name, node_openable_valve_distance) in node_openable_valves {
            let option_connected_node_openable_valve_distance = connected_node_openable_valves.get(node_opennable_valve_name);
            match option_connected_node_openable_valve_distance {
                None => continue,
                Some(distance) => {
                    if distance < node_openable_valve_distance {
                        connected_node_openable_valves_interesting.insert(node_opennable_valve_name.clone(), *distance);
                    }
                }
            }
        }
        connected_node_openable_valves_interesting
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
    openable_valves: HashMap<String, u32>,
    closed_valves: HashMap<String, u32>,
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
        Rc::new(RefCell::new(Node { valve: Rc::clone(&valve), nodes: vec![], openable_valves: HashMap::new(), closed_valves: HashMap::new() }))
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

                if !openable_valves.is_empty() {

                    let ref_children_nodes = &ref_node.borrow().nodes;
                    let child_node_has_been_edited = Node::add_openable_valves_to_children_nodes(ref_children_nodes, openable_valves);
                    
                    if child_node_has_been_edited {
                        node_has_been_edited = true;
                    }
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

        let self_openable_valves = &mut self.openable_valves;
        let self_valve_name = &self.valve.name;

        for (valve_name, distance) in openable_valves {
            let distance = *distance + 1;
            if self_valve_name == valve_name
                ||
                (self_openable_valves.contains_key(valve_name) && self_openable_valves.get(valve_name).unwrap() <= &distance) 
            {
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
    flow: i32,
    name_of_connected_valves: Vec<String>
}

impl Valve {
    pub fn new(name: String, flow: i32, name_of_connected_valves: Vec<String>) -> Self {
        Self { name, flow, name_of_connected_valves }
    }
}