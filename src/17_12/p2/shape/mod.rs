use std::{fmt::Debug, any::Any, rc::{Rc, Weak}, cell::RefCell};

/*
Formes Possibles :
#### --- Tetrimino Horizontal Bar

.#.
### --- Tetrimino Croix
.#.

..#
..# --- Tetrimino J
###

#
#   --- Tetrimino Vertical Bar
#
#

##
##  --- Tetrimino Carre

*/

static BORDERS: (i64, i64) = (0, 6);
static FLOOR: i64 = 1;

#[derive(Debug, PartialEq, Clone)]
pub struct Coordinate(pub i64, pub i64);

pub struct TetrisFactory;

impl TetrisFactory {
    pub fn new(order: u8, (x, y): (i64, i64)) -> Box<dyn Shape> {
        match order {
            1 => Box::new(TetriminoHorizontalBar::new((x, y))),
            2 => Box::new(TetriminoCross::new((x, y))),
            3 => Box::new(TetriminoJ::new((x, y))),
            4 => Box::new(TetriminoVerticalBar::new((x, y))),
            5 => Box::new(TetriminoSquare::new((x, y))),
            _ => panic!("unrecognized type")
        }
    }


    pub fn new_with_coordinates(order: u8, coordinates: Vec<Coordinate>) -> Box<dyn Shape> {
        match order {
            1 => Box::new(TetriminoHorizontalBar { coordinates }),
            2 => Box::new(TetriminoCross { coordinates }),
            3 => Box::new(TetriminoJ { coordinates }),
            4 => Box::new(TetriminoVerticalBar { coordinates }),
            5 => Box::new(TetriminoSquare { coordinates }),
            _ => panic!("unrecognized type")
        }
    }
}

pub trait Shape: Debug + Any {
    fn as_any(&self) -> &dyn Any;
    fn shift(&mut self, direction: &str);
    fn back(&mut self, direction: &str);
    fn check_crossing_with_border(&self) -> bool;
    fn check_crossing_with_another_tetrimino(&self, other: Rc<Box<dyn Shape>>) -> bool;
    fn get_top(&self) -> i64;
    fn get_bottom(&self) -> i64;
    fn get_left(&self) -> i64;
    fn get_name(&self) -> String;
    fn get_coordinates(&self) -> &Vec<Coordinate>;
}

macro_rules! impl_Shape {
    ($struct_type:ty) => {
        impl Shape for $struct_type {
            fn as_any(&self) -> &dyn Any {
                self
            }

            fn shift(&mut self, direction: &str) {
                match direction {
                    ">" => self.coordinates.iter_mut().for_each(|c| c.0 += 1),
                    "<" => self.coordinates.iter_mut().for_each(|c| c.0 -= 1),
                    "v" => self.coordinates.iter_mut().for_each(|c| c.1 -= 1),
                    "^" => self.coordinates.iter_mut().for_each(|c| c.1 += 1),
                    _ => panic!("direction not recognized")
                }
            }

            fn back(&mut self, direction: &str) {
                match direction {
                    ">" => self.coordinates.iter_mut().for_each(|c| c.0 -= 1),
                    "<" => self.coordinates.iter_mut().for_each(|c| c.0 += 1),
                    "v" => self.coordinates.iter_mut().for_each(|c| c.1 += 1),
                    "^" => self.coordinates.iter_mut().for_each(|c| c.1 -= 1),
                    _ => panic!("direction not recognized")
                }
            }
        
            fn check_crossing_with_border(&self) -> bool {
                let position_y_axis = self.coordinates.iter().map(|c| c.1).min().unwrap();
                let position_x_axis_min = self.coordinates.iter().map(|c| c.0).min().unwrap();
                let position_x_axis_max = self.coordinates.iter().map(|c| c.0).max().unwrap();
        
                position_y_axis >= FLOOR && position_x_axis_min >= BORDERS.0 && position_x_axis_max <= BORDERS.1
            }

            fn check_crossing_with_another_tetrimino(&self, other: Rc<Box<dyn Shape>>) -> bool {
                if let Some(other_tetrimino) = other.as_any().downcast_ref::<TetriminoHorizontalBar>() {
                    for c in &self.coordinates {
                        if other_tetrimino.coordinates.contains(c) {
                            // println!("{c:?}");
                            return false;
                        }
                    }
        
                    return true;
                }

                if let Some(other_tetrimino) = other.as_any().downcast_ref::<TetriminoCross>() {
                    for c in &self.coordinates {
                        if other_tetrimino.coordinates.contains(c) {
                            // println!("{c:?}");
                            return false;
                        }
                    }
        
                    return true;
                }

                if let Some(other_tetrimino) = other.as_any().downcast_ref::<TetriminoJ>() {
                    for c in &self.coordinates {
                        if other_tetrimino.coordinates.contains(c) {
                            // println!("{c:?}");
                            return false;
                        }
                    }
        
                    return true;
                }

                if let Some(other_tetrimino) = other.as_any().downcast_ref::<TetriminoVerticalBar>() {
                    for c in &self.coordinates {
                        if other_tetrimino.coordinates.contains(c) {
                            // println!("{c:?}");
                            return false;
                        }
                    }
        
                    return true;
                }

                if let Some(other_tetrimino) = other.as_any().downcast_ref::<TetriminoSquare>() {
                    for c in &self.coordinates {
                        if other_tetrimino.coordinates.contains(c) {
                            // println!("{c:?}");
                            return false;
                        }
                    }
        
                    return true;
                }
        
                panic!("unrecognized type")
            }

            fn get_top(&self) -> i64 {
                self.coordinates.iter().map(|c| c.1).max().unwrap()
            }

            fn get_bottom(&self) -> i64 {
                self.coordinates.iter().map(|c| c.1).min().unwrap()
            }

            fn get_left(&self) -> i64 {
                let bottom = self.get_bottom();
                let mut x_positions = vec![];

                for c in &self.coordinates {
                    if c.1 > bottom {
                        continue;
                    }
                    x_positions.push(c.0);
                }
                *(x_positions.iter().min().unwrap())
            }

            fn get_name(&self) -> String {
                if let Some(_) = self.as_any().downcast_ref::<TetriminoHorizontalBar>() {
                    "TetriminoHorizontalBar".to_string()
                } else if let Some(_) = self.as_any().downcast_ref::<TetriminoCross>() {
                    "TetriminoCross".to_string()
                } else if let Some(_) = self.as_any().downcast_ref::<TetriminoJ>() {
                    "TetriminoJ".to_string()
                } else if let Some(_) = self.as_any().downcast_ref::<TetriminoVerticalBar>() {
                    "TetriminoVerticalBar".to_string()
                } else if let Some(_) = self.as_any().downcast_ref::<TetriminoSquare>() {
                    "TetriminoSquare".to_string()
                } else {
                    panic!("unrecognized type")
                }
            }

            fn get_coordinates(&self) -> &Vec<Coordinate> {
                &self.coordinates
            }
        }
    }
}

impl_Shape! {TetriminoHorizontalBar}
impl_Shape! {TetriminoCross}
impl_Shape! {TetriminoJ}
impl_Shape! {TetriminoVerticalBar}
impl_Shape! {TetriminoSquare}


#[derive(Debug)]
pub struct TetriminoHorizontalBar {
    pub coordinates: Vec<Coordinate>
}

impl TetriminoHorizontalBar {
    fn new((x, y): (i64, i64)) -> Self {
        Self { coordinates : (x..(x+4)).into_iter().map(|x| Coordinate(x, y)).collect() }
    }
}

#[derive(Debug)]
pub struct TetriminoVerticalBar {
    pub coordinates: Vec<Coordinate>
}

impl TetriminoVerticalBar {
    fn new((x, y): (i64, i64)) -> Self {
        Self { coordinates : (y..(y+4)).into_iter().map(|y| Coordinate(x, y)).collect() }
    }
}

#[derive(Debug)]
pub struct TetriminoCross {
    pub coordinates: Vec<Coordinate>
}

impl TetriminoCross {
    fn new((x, y): (i64, i64)) -> Self {
        let mut coordinates = vec![];

        for (index_i, i) in (x..(x+3)).enumerate() {
            for (index_j, j) in (y..(y+3)).enumerate() {
                if (index_i == 0 || index_i == 2) && index_j != 1 {
                    continue;
                }
                coordinates.push(Coordinate(i, j));
            }
        }

        Self { coordinates }
    }
}

#[derive(Debug)]
pub struct TetriminoJ {
    pub coordinates: Vec<Coordinate>
}

impl TetriminoJ {
    fn new((x, y): (i64, i64)) -> Self {
        let mut coordinates = vec![];

        for (index_i, i) in (x..(x+3)).enumerate() {
            for (index_j, j) in (y..(y+3)).enumerate() {
                if (index_i == 0 || index_i == 1) && index_j != 0 {
                    continue;
                }
                coordinates.push(Coordinate(i, j));
            }
        }

        Self { coordinates }
    }
}

#[derive(Debug)]
pub struct TetriminoSquare {
    coordinates: Vec<Coordinate>
}

impl TetriminoSquare {
    fn new((x, y): (i64, i64)) -> Self {
        let mut coordinates = vec![];

        for i in x..(x+2) {
            for j in y..(y+2) {
                coordinates.push(Coordinate(i, j));
            }
        }

        Self { coordinates }
    }
}

#[derive(Debug, Clone)]
pub struct CycleGuesser {
    pub tetrimino: Rc<Box<dyn Shape>>,
    pub tetrimino_name: String,
    pub x_position: i64,
    pub index_jet: usize,
    pub total_jet: u32,
    pub height: i64,
    pub ptr_tetriminos_index: u8,
}

impl CycleGuesser {
    pub fn new(tetrimino: Rc<Box<dyn Shape>>, index_jet: usize, total_jet: u32, ptr_tetriminos_index: u8) -> Self {
        let tetrimino_name = tetrimino.get_name();
        let height = tetrimino.get_top();
        let mut x_position = tetrimino.get_left();

        CycleGuesser { tetrimino, tetrimino_name, x_position, index_jet, total_jet, height, ptr_tetriminos_index }
    }
}

impl PartialEq for CycleGuesser {
    fn eq(&self, other: &Self) -> bool {
        self.tetrimino_name == other.tetrimino_name && self.x_position == other.x_position && self.index_jet == other.index_jet && self.total_jet == other.total_jet && self.height == other.height
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    pub node: Option<Rc<RefCell<Node<T>>>>,
    // data: Option<T>,
    // previous: Option<T>,
    // next: Option<T>,
    pub current_index: i64
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        // LinkedList { data: None, previous: None, next: None, current_index: 0 }
        LinkedList { node: None, current_index: -1 }

    }

    pub fn add(&mut self, data: T) {
        match self.node {
            None => {
                self.node = Some(Rc::new(RefCell::new(Node::new(data, None))));
                self.current_index += 1;
            },
            Some(_) => {
                let last_node = self.get(self.current_index).unwrap();
                let new_node = Rc::new(RefCell::new(Node::new(data, Some(Rc::downgrade(&last_node)))));
                last_node.borrow_mut().next = Some(new_node);
                self.current_index += 1;
            }
        }
    }

    pub fn get(&self, index: i64) -> Result<Rc<RefCell<Node<T>>>, MyError> {
        if index < 0 || index > self.current_index {
            Err(MyError { message: "index is out of range".to_string() })
        } else {
            let mut node = Rc::clone(&self.node.as_ref().unwrap());
            
            for _ in 0..index {
                node = {
                    let borrow_node = node.borrow();
                    Rc::clone(borrow_node.next.as_ref().unwrap())
                }
            }

            Ok(Rc::clone(&node))
        }
    }

    pub fn size(&self) -> u32 {
        let mut size = 0;

        match self.node.as_ref() {
            None => (),
            Some(d) => {
                let mut next = Rc::clone(d);
                let mut vec = vec![];
                loop {
                    size += 1;
                    // let borrow_node = {
                    //     let n = Rc::clone(&next);
                    //     n.borrow()
                    // };
                    // let borrow_node = next.borrow();
                    next = if vec.is_empty() {
                        next
                    } else {
                        vec.pop().unwrap()
                    };

                    match next.borrow().next.as_ref() {
                        None => break,
                        Some(n) => {
                            vec.push(Rc::clone(&n));
                            // next = Rc::clone(&n);
                        }
                    }
                }
            }
        }

        size
    }
}

#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    pub previous: Option<Weak<RefCell<Node<T>>>>,
    pub next: Option<Rc<RefCell<Node<T>>>>
}

impl<T> Node<T> {
    pub fn new(data: T, previous: Option<Weak<RefCell<Node<T>>>>) -> Self {
        Self {data, previous, next: None }
    }
}

#[derive(Debug)]
pub struct MyError {
    message: String,
}

impl MyError {
    fn new(message: &str) -> MyError {
        MyError {
            message: message.to_string(),
        }
    }
}
