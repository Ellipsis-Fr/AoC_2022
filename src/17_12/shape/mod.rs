use std::{fmt::Debug, any::Any, rc::Rc};

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

static BORDERS: (i32, i32) = (0, 6);
static FLOOR: i32 = 1;

#[derive(Debug, PartialEq)]
pub struct Coordinate(pub i32, pub i32);

pub struct TetrisFactory;

impl TetrisFactory {
    pub fn new(order: u8, (x, y): (i32, i32)) -> Box<dyn Shape> {
        match order {
            1 => Box::new(TetriminoHorizontalBar::new((x, y))),
            2 => Box::new(TetriminoCross::new((x, y))),
            3 => Box::new(TetriminoJ::new((x, y))),
            4 => Box::new(TetriminoVerticalBar::new((x, y))),
            5 => Box::new(TetriminoSquare::new((x, y))),
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
    fn get_top(&self) -> i32;
    fn get_bottom(&self) -> i32;
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

            fn get_top(&self) -> i32 {
                self.coordinates.iter().map(|c| c.1).max().unwrap()
            }

            fn get_bottom(&self) -> i32 {
                self.coordinates.iter().map(|c| c.1).min().unwrap()
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
    coordinates: Vec<Coordinate>
}

impl TetriminoHorizontalBar {
    fn new((x, y): (i32, i32)) -> Self {
        Self { coordinates : (x..(x+4)).into_iter().map(|x| Coordinate(x, y)).collect() }
    }
}

#[derive(Debug)]
pub struct TetriminoVerticalBar {
    coordinates: Vec<Coordinate>
}

impl TetriminoVerticalBar {
    fn new((x, y): (i32, i32)) -> Self {
        Self { coordinates : (y..(y+4)).into_iter().map(|y| Coordinate(x, y)).collect() }
    }
}

#[derive(Debug)]
pub struct TetriminoCross {
    coordinates: Vec<Coordinate>
}

impl TetriminoCross {
    fn new((x, y): (i32, i32)) -> Self {
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
    coordinates: Vec<Coordinate>
}

impl TetriminoJ {
    fn new((x, y): (i32, i32)) -> Self {
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
    fn new((x, y): (i32, i32)) -> Self {
        let mut coordinates = vec![];

        for i in x..(x+2) {
            for j in y..(y+2) {
                coordinates.push(Coordinate(i, j));
            }
        }

        Self { coordinates }
    }
}