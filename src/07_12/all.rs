use std::{rc::Rc, cell::{RefCell, Ref, RefMut}};

use AoC_2022::text_file_reader::TextFileReader;
use tree_structure::{TreeStructure, Folder, File};
mod tree_structure;

fn main() {
    println!("Puzzle du 07/12");
    let puzzle = get_puzzle();
    let root_folder = create_file_system(puzzle);
    root_folder.borrow_mut().folder_size_calculation();
    println!("TOTAL SIZE : {}", root_folder.borrow().size);
    println!("Part 1 : SIZE FOLDER LESS THAN 100000 Bytes : {}", root_folder.borrow().calculation_of_total_space_occupied_by_folders_of_less_than_100000_bytes());

    let total_space: u32 = 70_000_000;
    let space_need: u32 = 30_000_000;
    let actual_space: u32 = total_space - root_folder.borrow().size;
    let space_to_find: u32 = space_need - actual_space;

    println!("space to find {}", space_to_find);

    let folder_to_delete = root_folder.borrow().find_folder_to_delete(space_to_find, root_folder.borrow().clone());
    println!("Part 2 : dossier à supprimer : {}, taille : {}", folder_to_delete.name, folder_to_delete.size);
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("07_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn create_file_system(puzzle: Vec<String>) -> Rc<RefCell<Folder>> {
    let root_folder = Rc::new(RefCell::new(Folder::new("/", None)));
    let mut current_folder: Rc<RefCell<Folder>> = Rc::clone(&root_folder);
    
    for (index, terminal_output) in puzzle.iter().enumerate() {
        let output = terminal_output.split_whitespace().collect::<Vec<&str>>();
        match output[0] {
            "$" => {
                match output[1] {
                    "ls" => continue,
                    "cd" => {
                        match output[2] {
                            "/" => continue,
                            ".." => {
                                if current_folder.borrow().parent_folder.is_some() {
                                    let parent = Rc::clone(current_folder.borrow().parent_folder.as_ref().unwrap());
                                    current_folder = parent;
                                }                                
                                /* 
                                // Pas réussi avec les match 
                                // match current_folder {
                                //     None => continue,
                                //     Some(folder) => {
                                //         match &folder.borrow().parent {
                                //             None => continue,
                                //             Some(parent) => {
                                //                 current_folder = Some(Rc::clone(&parent));
                                //                 continue;
                                //             }
                                //         };
                                //     }
                                // } */
                            },
                            folder_name => {
                                let subfolder = current_folder.borrow().get_subfolder_by_name(folder_name);
                                current_folder = subfolder;
                            }
                        }
                    },
                    _ => panic!("autre commande que \"cd\" ou \"ls\" trouvée : {}, ligne : {index}", output[2])
                }
            },
            "dir" => {
                current_folder.borrow_mut().add_subfolder(&Rc::new(RefCell::new(Folder::new(output[1], Some(&current_folder)))));
            },
            v => {
                let file_size = v.parse::<u32>().unwrap();
                let file = File::new(output[1], file_size, &current_folder);
                current_folder.borrow_mut().add_file(file)
            }
        }
    }

    root_folder
}

/* fn create_file_system(puzzle: Vec<String>) -> TreeStructure {
    let mut file_system = init_file_system();
    let mut current_folder: Option<Rc<RefCell<Folder>>> = None;
    
    for (index, terminal_output) in puzzle.iter().enumerate() {
        let output = terminal_output.split_whitespace().collect::<Vec<&str>>();
        match output[0] {
            "$" => {
                match output[1] {
                    "ls" => continue,
                    "cd" => {
                        match output[2] {
                            ".." => {
                                if current_folder.is_some() {
                                    let current_folder_to_look_for_parent = current_folder.clone().unwrap().clone();
                                    if current_folder_to_look_for_parent.borrow().parent_folder.is_some() {
                                        let parent = current_folder_to_look_for_parent.borrow().parent_folder.as_ref().unwrap().clone();
                                        current_folder = Some(Rc::clone(&parent));
                                    }
                                }
                                
                                /* 
                                // Pas réussi avec les match 
                                // match current_folder {
                                //     None => continue,
                                //     Some(folder) => {
                                //         match &folder.borrow().parent {
                                //             None => continue,
                                //             Some(parent) => {
                                //                 current_folder = Some(Rc::clone(&parent));
                                //                 continue;
                                //             }
                                //         };
                                //     }
                                // } */
                            },
                            folder_name => {
                                if let Some(folder) = file_system.get_folder_by_name(folder_name) {
                                    current_folder = Some(Rc::clone(&folder));
                                }
                            }
                        }
                    },
                    _ => panic!("autre commande que \"cd\" ou \"ls\" trouvée : {}, ligne : {index}", output[2])
                }
            },
            "dir" => {
                let new_folder = Rc::new(RefCell::new(Folder::new(output[1], current_folder.as_ref())));
                current_folder.as_ref().unwrap().borrow_mut().add_subfolder(&new_folder);
                file_system.add(&new_folder);
            },
            v => {
                let file_size = v.parse::<u32>().unwrap();
                let file = File::new(output[1], file_size, Rc::clone(current_folder.as_ref().unwrap()));
                current_folder.as_mut().unwrap().borrow_mut().add_file(file)
            }
        }
    }

    file_system
}

fn init_file_system() -> TreeStructure {
    let mut file_system = TreeStructure::new();
    let root_folder = Rc::new(RefCell::new(Folder::new("/", None)));
    file_system.add(&root_folder);
    file_system
} */