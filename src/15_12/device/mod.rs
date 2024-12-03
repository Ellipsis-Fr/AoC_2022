use std::collections::{HashSet, VecDeque};

use num::integer::sqrt;
use num::pow;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Point(pub i64, pub i64);

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point(x, y)
    }

    pub fn get_manhattan_distance(&self, other: &Point) -> i64 {
        sqrt(pow(other.0 - self.0, 2)) + sqrt(pow(other.1 - self.1, 2))
    }

    pub fn get_all_point_from_a_perimeter(&self, radius: i64) -> HashSet<Point> {
        // Scope of circle
        let center_x = self.0;
        let min_x = self.0 - radius;
        let max_x = self.0 + radius;
        
        let center_y = self.1;
        let min_y = self.1 - radius;
        let max_y = self.1 + radius;
        
        // let diameter_x = 1 + sqrt(pow(max_x - min_x, 2));
        // let diameter_y = 1 + sqrt(pow(max_y - min_y, 2));

        // let x_coordinates = vec![-1; diameter_x as usize];
        // let y_coordinates = vec![x_coordinates; diameter_y as usize];

        // Creating half-circle points
        let mut points_of_a_half_circle = VecDeque::new();
        let mut first_points = VecDeque::new();
        for x in min_x..=max_x {
            first_points.push_back(Point(x, self.1));
        }

        println!("first points finished");

        for (i, y) in (self.1..=max_y).enumerate() {
            if i == 0 {
                continue;
            }
            let mut points_to_add = first_points.clone();
            for _ in 0..i {
                points_to_add.pop_front();
                points_to_add.pop_back();
            }

            points_to_add.iter_mut().map(|mut p| p.1 = y).count();
            points_of_a_half_circle.append(&mut points_to_add);
        }

        points_of_a_half_circle.append(&mut first_points);

        println!("half-circle finished");


        // Creating circle points
        let mut points_of_a_circle = HashSet::new();
        points_of_a_circle.extend(points_of_a_half_circle.iter());

        for p in points_of_a_half_circle {
            if p.1 == center_y {
                continue;
            }
            let dy = p.1 - center_y;
            let new_y = center_y - dy;
            let point = Point(p.0, new_y);
            points_of_a_circle.insert(point);
        }

        println!("circle finished");


        points_of_a_circle
    }

}


const INIT_BEACON: &str = "closest beacon is at";
#[derive(Debug, Clone, Copy)]
pub struct Beacon {
    pub position: Point,
}

impl Beacon {
    fn new(init_beacon: &str) -> Self {
        let (x, y) = slices_sentence_to_find_coordinate(init_beacon, INIT_BEACON);
        Beacon { position: Point::new(x, y) }
    }

    fn get_position(&self) -> &Point {
        &self.position
    }
}

const INIT_SENSOR: &str = "Sensor at";
#[derive(Debug, Clone, Copy)]
pub struct Sensor {
    position: Point,
    beacon: Beacon,
    distance: i64
}

impl Sensor {
    pub fn new(init_sensor_and_beacon: &str) -> Self {
        let inits = init_sensor_and_beacon.split(":").collect::<Vec<&str>>();
        let init_sensor = inits.get(0).unwrap().trim();
        let init_beacon = inits.get(1).unwrap().trim();

        let beacon = Beacon::new(init_beacon);
        let (x, y) = slices_sentence_to_find_coordinate(init_sensor, INIT_SENSOR);
        let position = Point::new(x, y);

        let distance = position.get_manhattan_distance(beacon.get_position());

        Sensor { position, beacon, distance }
    }

    pub fn get_position(&self) -> &Point {
        &self.position
    }

    pub fn get_beacon(&self) -> &Beacon {
        &self.beacon
    }

    pub fn get_beacon_position(&self) -> &Point {
        &self.beacon.get_position()
    }

    pub fn get_distance(&self) -> i64 {
        self.distance
    }
}


fn slices_sentence_to_find_coordinate(init_sentence: &str, sentence_to_slice: &str) -> (i64, i64) {
    let start = init_sentence.find(sentence_to_slice).unwrap();
    let end = start + init_sentence.len();

    let init_sentence = init_sentence.to_owned();
    init_sentence.to_owned().replace_range(start..end, "");
    let coordinates_str = init_sentence.split(",").collect::<Vec<&str>>();

    let mut coordinates = vec![];
    for coordinate_str in coordinates_str {
        let index = coordinate_str.find("=").unwrap() + 1;
        let coordinate = &coordinate_str[index..];
        coordinates.push(coordinate.parse::<i64>().unwrap());
    }


    (*coordinates.get(0).unwrap(), *coordinates.get(1).unwrap())
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::Point;

    #[test]
    fn test_calcul_manhattan_distance() {
        // Given 
        let point_a = Point (0, 0);
        let point_b = Point (-6, -3);
        let point_c = Point (3, 5);
        let point_d = Point (-4, 3);
        let point_e = Point (-2, -4);
        let point_f = Point (5, -2);
        let point_g = Point (5, 3);
        let point_h = Point (0, 8);


        // When
        let manhattan_distance_a_b = point_a.get_manhattan_distance(&point_b);
        let manhattan_distance_a_c = point_a.get_manhattan_distance(&point_c);
        let manhattan_distance_a_d = point_a.get_manhattan_distance(&point_d);
        let manhattan_distance_a_e = point_a.get_manhattan_distance(&point_e);
        let manhattan_distance_a_f = point_a.get_manhattan_distance(&point_f);
        let manhattan_distance_a_g = point_a.get_manhattan_distance(&point_g);
        let manhattan_distance_a_h = point_a.get_manhattan_distance(&point_h);
        
        let manhattan_distance_a_a = point_a.get_manhattan_distance(&point_a);
        let manhattan_distance_b_a = point_b.get_manhattan_distance(&point_a);
        let manhattan_distance_f_a = point_f.get_manhattan_distance(&point_a);
        let manhattan_distance_h_a = point_h.get_manhattan_distance(&point_a);
        
        let manhattan_distance_b_c = point_b.get_manhattan_distance(&point_c);
        let manhattan_distance_b_d = point_b.get_manhattan_distance(&point_d);
        let manhattan_distance_b_e = point_b.get_manhattan_distance(&point_e);
        let manhattan_distance_b_f = point_b.get_manhattan_distance(&point_f);
        let manhattan_distance_b_g = point_b.get_manhattan_distance(&point_g);
        let manhattan_distance_b_h = point_b.get_manhattan_distance(&point_h);

        let manhattan_distance_c_d = point_c.get_manhattan_distance(&point_d);
        let manhattan_distance_c_e = point_c.get_manhattan_distance(&point_e);
        let manhattan_distance_c_f = point_c.get_manhattan_distance(&point_f);
        let manhattan_distance_c_g = point_c.get_manhattan_distance(&point_g);
        let manhattan_distance_c_h = point_c.get_manhattan_distance(&point_h);

        let manhattan_distance_d_e = point_d.get_manhattan_distance(&point_e);
        let manhattan_distance_d_f = point_d.get_manhattan_distance(&point_f);
        let manhattan_distance_d_g = point_d.get_manhattan_distance(&point_g);
        let manhattan_distance_d_h = point_d.get_manhattan_distance(&point_h);

        let manhattan_distance_e_d = point_e.get_manhattan_distance(&point_d);

        let manhattan_distance_e_f = point_e.get_manhattan_distance(&point_f);
        let manhattan_distance_e_g = point_e.get_manhattan_distance(&point_g);
        let manhattan_distance_e_h = point_e.get_manhattan_distance(&point_h);

        let manhattan_distance_f_g = point_f.get_manhattan_distance(&point_g);
        let manhattan_distance_f_h = point_f.get_manhattan_distance(&point_h);

        let manhattan_distance_g_h = point_g.get_manhattan_distance(&point_h);


        // Then
        assert_eq!(9, manhattan_distance_a_b);
        assert_eq!(8, manhattan_distance_a_c);
        assert_eq!(7, manhattan_distance_a_d);
        assert_eq!(6, manhattan_distance_a_e);
        assert_eq!(7, manhattan_distance_a_f);
        assert_eq!(8, manhattan_distance_a_g);
        assert_eq!(8, manhattan_distance_a_h);

        assert_eq!(0, manhattan_distance_a_a);
        assert_eq!(9, manhattan_distance_b_a);
        assert_eq!(7, manhattan_distance_f_a);
        assert_eq!(8, manhattan_distance_h_a);

        assert_eq!(17, manhattan_distance_b_c);
        assert_eq!(8, manhattan_distance_b_d);
        assert_eq!(5, manhattan_distance_b_e);
        assert_eq!(12, manhattan_distance_b_f);
        assert_eq!(17, manhattan_distance_b_g);
        assert_eq!(17, manhattan_distance_b_h);

        assert_eq!(9, manhattan_distance_c_d);
        assert_eq!(14, manhattan_distance_c_e);
        assert_eq!(9, manhattan_distance_c_f);
        assert_eq!(4, manhattan_distance_c_g);
        assert_eq!(6, manhattan_distance_c_h);

        assert_eq!(9, manhattan_distance_d_e);
        assert_eq!(14, manhattan_distance_d_f);
        assert_eq!(9, manhattan_distance_d_g);
        assert_eq!(9, manhattan_distance_d_h);

        assert_eq!(9, manhattan_distance_e_d);

        assert_eq!(9, manhattan_distance_e_f);
        assert_eq!(14, manhattan_distance_e_g);
        assert_eq!(14, manhattan_distance_e_h);


        assert_eq!(5, manhattan_distance_f_g);
        assert_eq!(15, manhattan_distance_f_h);

        assert_eq!(10, manhattan_distance_g_h);
    }

    #[test]
    fn test_calcul_manhattan_distance_with_value_from_example() {
        // Given
        let s = Point(8, 7);
        let b = Point(2, 10);


        // When
        let manhattan_distance_s_to_b = s.get_manhattan_distance(&b);
        let manhattan_distance_b_to_s = b.get_manhattan_distance(&s);

        let manhattan_distance_s_to_s = s.get_manhattan_distance(&s);

        // Then
        assert_eq!(9, manhattan_distance_s_to_b);
        assert_eq!(manhattan_distance_s_to_b, manhattan_distance_b_to_s);

        assert_eq!(0, manhattan_distance_s_to_s);
    }

    #[test]
    fn test_get_all_point_from_a_perimeter() {
        // Given
        let s = Point(5, 12);
        let b = Point(7, 11);

        // let s = Point(2924811, 3544081);
        // let b = Point(3281893, 3687621);

        let points = [
            Point(5, 12), 
            Point(5, 13), Point(6, 13), Point(7, 13), Point(5, 14), Point(6, 14), Point(5, 15),
            Point(6, 10), Point(6, 11), Point(7, 11), Point(6, 12), Point(7, 12), Point(8, 12),
            Point(5, 9), Point(4, 10), Point(5, 10), Point(3, 11), Point(4, 11), Point(5, 11),
            Point(2, 12), Point(3, 12), Point(4, 12), Point(3, 13), Point(4, 13), Point(4, 14),
        ].iter().copied().collect::<HashSet<Point>>();

        // When
        let manhattan_distance_s_to_b = s.get_manhattan_distance(&b);
        println!("manhattan distance : {manhattan_distance_s_to_b}");
        let calculated_points = s.get_all_point_from_a_perimeter(manhattan_distance_s_to_b);
        println!("{:?}", calculated_points);


        // Then
        assert_eq!(points.len(), calculated_points.len());

        for point in points {
            assert!(calculated_points.contains(&point));
        }
    }
}