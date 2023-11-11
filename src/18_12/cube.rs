#[derive(Debug, Clone, PartialEq)]
pub struct Cube {
    faces: Vec<Face>,
}

impl Cube {
    pub fn new(x: i32, y: i32, z:i32) -> Self {
        let faces = (1..=6).map(|side| Face::new(side, x, y, z)).collect();
        Self { faces }
    }

    pub fn get_faces(&self) -> &Vec<Face> {
        &self.faces
    }

    pub fn remove_adjacent_faces(&mut self, other: &mut Cube) {
        // dbg!(&self, &other);
        let self_faces = &mut self.faces;
        let other_faces = &mut other.faces;

        'outer: for (index_1, self_face) in self_faces.clone().iter().enumerate() {
            for (index_2, other_face) in other_faces.clone().iter().enumerate() {
                if self_face == other_face {
                    // dbg!(self_face, other_face);
                    self_faces.remove(index_1);
                    other_faces.remove(index_2);
                    break 'outer;
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Face {
    side: char,
    x: i32,
    y: i32,
    z: i32,
}

impl Face {
    pub fn new(side: u8, x: i32, y: i32, z: i32) -> Self {
        let (side, x, y, z) = match side {
            1 => ('z', x, y, z + 1),
            6 => ('z', x, y, z),
            2 => ('y', x, y + 1, z),
            5 => ('y', x, y, z),
            3 => ('x', x + 1, y, z),
            4 => ('x', x, y, z),
            _ => panic!("Side unrecognized")
        };

        Self { side, x, y, z }
    }
}