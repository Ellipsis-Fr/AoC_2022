use std::{rc::Rc, cell::RefCell};

use AoC_2022::text_file_reader::TextFileReader;
use cube::Cube;

mod cube;

fn main() {
    println!("Puzzle du 18/12 Partie 1");
    
    let puzzle = get_puzzle();
    let mut cubes = create_cubes(puzzle);
    // println!("{cubes:?}");
    let visible_faces = count_visible_faces(&mut cubes);
    println!("{visible_faces}");
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("18_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn create_cubes(puzzle: Vec<String>) -> Vec<Cube> {
    let mut cubes = vec![];

    for p in puzzle {
        let coordinates = p.split(",").map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
        cubes.push(Cube::new(coordinates[0], coordinates[1], coordinates[2]));
    }

    cubes
}

fn count_visible_faces(cubes: &mut Vec<Cube>) -> u32 {
    // let rc_cubes = Rc::new(RefCell::new(cubes));
    // let ref_cubes = cubes.clone();

    let rc_cubes = Rc::new(RefCell::new(cubes.clone()));

    for (index, _) in cubes.clone().iter().enumerate() {
        if index + 1 == cubes.len() {
            continue;
        }
        
        let mut borrowed_cubes = rc_cubes.borrow_mut();
        for (index_2, _) in cubes.clone().iter().enumerate().skip(index + 1) {
            {
                let ref_cube = borrowed_cubes.get_mut(index).unwrap();
                let compared_cube = cubes.get_mut(index_2).unwrap();
                ref_cube.remove_adjacent_faces(compared_cube);
            }

            {
                let ref_cube = borrowed_cubes.get_mut(index_2).unwrap();
                let compared_cube = cubes.get_mut(index).unwrap();
                ref_cube.remove_adjacent_faces(compared_cube);
            }

        }
    }
    
    // cubes.iter().map(|c| c.get_faces().len() as u32).collect::<Vec<u32>>().iter().sum()
    let borrowed_cubes = rc_cubes.borrow();
    borrowed_cubes.iter().map(|c| c.get_faces().len() as u32).collect::<Vec<u32>>().iter().sum()
}