use std::{sync::{Mutex, Arc}, thread};

use point::Point;

use super::point;

#[derive(Debug)]
pub struct Tree {
    pub root_node: Arc<Mutex<Node>>,
    pub end_node: Option<Arc<Mutex<Node>>>,
    pub nodes: Vec<Arc<Mutex<Node>>>
}

impl Tree {
    pub fn new((y, x): (i32, i32), value: u32) -> Self {
        let root_node = Arc::new(Mutex::new(Node::new((y, x), value)));
        let nodes = vec![Arc::clone(&root_node)];
        Tree { root_node, end_node: None, nodes  }
    }

    pub fn get_existing_node(&self, node: Node) -> Option<Arc<Mutex<Node>>> { // si le noeud existe déjà il faut réussir à retourner un lien vers lui pour ajouter la bonne référence au noeud parent
        for existing_node in &self.nodes {
            if node == *existing_node.lock().unwrap() {
                return Some(Arc::clone(existing_node));
            }
        }

        None
    }

    pub fn get_existing_node_by_position(&self, (y, x): (i32, i32)) -> Option<Arc<Mutex<Node>>> { // si le noeud existe déjà il faut réussir à retourner un lien vers lui pour ajouter la bonne référence au noeud parent
        
        for existing_node in &self.nodes {
            let existing_point = &existing_node.lock().unwrap().point;
            if y == existing_point.y && x == existing_point.x {
                return Some(Arc::clone(existing_node));
            }
        }

        None
    }

    pub fn add_node(&mut self, node: Arc<Mutex<Node>>) {
        if node.lock().unwrap().point.value == 123 {
            self.end_node = Some(Arc::clone(&node));
        }
        self.nodes.push(node);
    }

    pub fn print(&self) {
        for node in &self.nodes {
            let lock_node = node.lock().unwrap();
            let point = lock_node.point.clone();
            println!("{:?}", point);
        }
    }

    pub fn count_step(&self) {
        // let node = self.root_node.lock().unwrap();
        Node::count(Arc::clone(&self.root_node), 0, Arc::clone(self.end_node.as_ref().unwrap())); 
    }

    pub fn print_step(&self) {
        let lock_end_node = self.end_node.as_ref().unwrap().lock().unwrap();
        println!("{:?}", lock_end_node.position);
    }
}

#[derive(Debug)]
pub struct Node {
    pub point: Point,
    pub children: Vec<Arc<Mutex<Node>>>,
    pub position: u32 // correspond au nombre de pas
}

impl Node {
    pub fn new((y, x): (i32, i32), value: u32) -> Self {
        Node { 
            point: Point::new(x, y, value),
            children: vec![],
            position: 0
        }
    }

    pub fn add_node(&mut self, child_node: Arc<Mutex<Node>>) { // si le noeud existait déjà il faut que "child_node" référence ce noeud existant
        self.children.push(child_node);
    }

    pub fn count(node: Arc<Mutex<Node>>, mut step: u32, end_node: Arc<Mutex<Node>>) {
        let lock_end_node = end_node.lock().unwrap();
        if lock_end_node.position != 0 && lock_end_node.position < step {
            println!("{step}");
            println!("fini");
            return;
        } 
        drop(lock_end_node);

        step += 1;
        let lock_node = node.lock().unwrap();
        // let children_nodes = lock_node.children;

        if lock_node.point.value == 123 {
            println!("{step}");
        }
        
        let mut nodes_to_continue = vec![];
        for child_node in lock_node.children.iter() {
            let mut lock_child_node = child_node.lock().unwrap();
            let position = lock_child_node.position;

            if position == 0 || step < position {
                lock_child_node.position = step;
                nodes_to_continue.push(Arc::clone(&child_node));
            }
        }
        drop(lock_node);

        let mut handlers = vec![];
        for node in nodes_to_continue {
            let actual_node = Arc::clone(&node);
            let end_node = Arc::clone(&end_node);
            drop(node);
            let handler = thread::spawn(move || {
                Node::count(actual_node, step, end_node);
            });
            handlers.push(handler);
        }

        for handler in handlers {
            handler.join().unwrap();
        }

    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}