use std::{hash::{Hash, Hasher}, clone};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Direction {
    Horizontal, Vertical
}

impl Direction {
    fn new(x1: u32, x2: u32) -> Self {
        if x1 == x2 {
            Direction::Vertical
        } else {
            Direction::Horizontal
        }
    }
}

#[derive(Debug, Eq, Clone)]
pub struct Line {
    pub direction: Direction,
    pub direction_coordinate: u32,
    pub vertex_a: u32,
    pub vertex_b: u32,
}

impl Line {
    pub fn new<'a>(point_a: &'a str, point_b: &'a str) -> Self {
        let ((x_a, y_a), (x_b, y_b)) = Self::get_coordinates_from_point(point_a, point_b);
        let direction = Direction::new(x_a, x_b);
        
        let direction_coordinate = match direction {
            Direction::Horizontal => y_a,
            Direction::Vertical => x_a
        };

        let (vertex_a, vertex_b) = match direction {
            Direction::Horizontal => {
                if x_a <= x_b {
                    (x_a, x_b)
                } else {
                    (x_b, x_a)
                }
            },
            Direction::Vertical => {
                if y_a <= y_b {
                    (y_a, y_b)
                } else {
                    (y_b, y_a)
                }
            }
        };
        
        Line { direction, direction_coordinate, vertex_a, vertex_b }
    }

    fn get_coordinates_from_point<'a>(point_a: &'a str, point_b: &'a str) -> ((u32, u32), (u32, u32)) {
        let tuple_from_point = |point: &'a str| -> (u32, u32) {
            let mut iter = point.split(",");
            (iter.next().unwrap().trim().parse::<u32>().unwrap(), iter.next().unwrap().trim().parse::<u32>().unwrap())
        };

        (tuple_from_point(point_a), tuple_from_point(point_b))
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.direction == other.direction 
        && self.direction_coordinate == other.direction_coordinate 
        && (self.vertex_a == other.vertex_a || self.vertex_a == other.vertex_b)
        && (self.vertex_b == other.vertex_b || self.vertex_b == other.vertex_a)
    }
}

impl Hash for Line {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.direction.hash(state);
        self.direction_coordinate.hash(state);
        // self.vertex_a.hash(state);
        // self.vertex_b.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::line::{Line, Direction};

    #[test]
    fn test_init_line() {
        // Given
        let point_a = " 515,60";
        let point_b = " 515,52 ";

        let point_c = "515,60 ";
        let point_d = "517,60";

        // When
        let vertical_line = Line::new(point_a, point_b);
        let horizontal_line = Line::new(point_d, point_c);

        // Then
        let vertical_line_to_be_obtained = Line {
            direction: Direction::Vertical,
            direction_coordinate: 515,
            vertex_a: 52,
            vertex_b: 60
        };

        let horizontal_line_to_be_obtained = Line {
            direction: Direction::Horizontal,
            direction_coordinate: 60,
            vertex_a: 515,
            vertex_b: 517
        };
        
        assert_eq!(vertical_line, vertical_line_to_be_obtained);
        assert_eq!(horizontal_line, horizontal_line_to_be_obtained);
    }

    #[test]
    fn test_line_uniqueness() {
        // Given
        let point_a = " 515,60";
        let point_b = " 515,52 ";

        let point_c = "515,60 ";
        let point_d = "517,60";

        let point_e = "530,60 ";
        let point_f = "530,41";

        let point_g = "515,60 ";
        let point_h = "515,41";

        let point_i = "515,52";
        let point_j = "515,60";
        

        // When
        let line_1 = Line::new(point_a, point_b);
        let line_2 = Line::new(point_c, point_d);
        let line_3 = Line::new(point_e, point_f);
        let line_4 = Line::new(point_g, point_h);
        let line_5 = Line::new(point_i, point_j);

        let mut lines = HashSet::new();


        // Then
        assert_eq!(lines.insert(line_1), true);
        assert_eq!(lines.insert(line_2), true);
        assert_eq!(lines.insert(line_3), true);
        assert_eq!(lines.insert(line_4), true);
        assert_eq!(lines.insert(line_5), false);

        assert_eq!(lines.len(), 4);
    }

    #[test]
    fn test_ascending_sorting_list_of_lines_according_to_direction() {
        // Given
        let point_a = " 515,60";
        let point_b = " 515,52 ";
        let line_1 = Line::new(point_a, point_b);
        
        let point_c = "515,60 ";
        let point_d = "517,60";
        let line_2 = Line::new(point_c, point_d);

        let point_e = "509,60 ";
        let point_f = "509,41";
        let line_3 = Line::new(point_e, point_f);

        let point_g = "515,60 ";
        let point_h = "515,41";
        let line_4 = Line::new(point_g, point_h);

        let point_i = "515,52";
        let point_j = "515,60";
        let line_5 = Line::new(point_i, point_j);
        
        let mut lines = HashSet::new();
        lines.insert(line_1);
        lines.insert(line_2);
        lines.insert(line_3);
        lines.insert(line_4);
        lines.insert(line_5);

        // When
        let mut horizontal_lines = lines.iter().filter(|l| l.direction == Direction::Horizontal).collect::<Vec<&Line>>();
        horizontal_lines.sort_by(|a, b| a.direction_coordinate.cmp(&b.direction_coordinate));
        
        let mut vertical_lines = lines.iter().filter(|l| l.direction == Direction::Vertical).collect::<Vec<&Line>>();
        vertical_lines.sort_by(|a, b| a.direction_coordinate.cmp(&b.direction_coordinate));


        // Then
        assert_eq!(lines.len(), 4);
        assert_eq!(horizontal_lines.len(), 1);
        assert_eq!(vertical_lines.len(), 3);

        assert!(horizontal_lines.iter().all(|l| l.direction == Direction::Horizontal));
        assert!(vertical_lines.iter().all(|l| l.direction == Direction::Vertical));

        assert_eq!(vertical_lines.get(0).unwrap().direction_coordinate, 509);
    }
}