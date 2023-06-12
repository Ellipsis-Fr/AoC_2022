#[derive(Debug, Eq, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub value: u32 
}

impl Point {
    pub fn new(x: i32, y: i32, value: u32) -> Self {
        Point { x, y, value }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}