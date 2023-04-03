use AoC_2022::text_file_reader::{self, TextFileReader};


fn main() {
    println!("Puzzle du 04/12");
    
    let puzzle = get_puzzle();
    let vec_puzzle = convert_in_vec(puzzle);
    let count_pairs_with_same_assignments = get_count_pairs_with_same_assignments(vec_puzzle);
    println!("Total d'assignement se superposant : {count_pairs_with_same_assignments}");
}

fn get_puzzle() -> String {
    let mut text_file_reader = TextFileReader::new("04_12_p1.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content().to_owned()
}

fn convert_in_vec(puzzle: String) -> Vec<String> {
    puzzle.lines().map(str::to_string).collect()
}

fn get_count_pairs_with_same_assignments(vec_puzzle: Vec<String>) -> u32 {
    let mut count = 0;
    
    for pair in vec_puzzle {
        let assignments_by_pair: Vec<&str> = pair.split(",").collect();
        let assignments_elf_1: (u32, u32) = get_assignements_by_elf(assignments_by_pair[0]);
        let assignments_elf_2: (u32, u32) = get_assignements_by_elf(assignments_by_pair[1]);
        
        let (early_assignment, late_assignment) = get_ordered_assignments(assignments_elf_1, assignments_elf_2);
        
        if assignment_contains_the_other(early_assignment, late_assignment) {
            count += 1;
        }
    }

    count
}

fn get_assignements_by_elf(assignments: &str) -> (u32, u32) {
    let assignments: Vec<u32> = assignments.split("-").map(|x| x.parse().unwrap()).collect();
    (assignments[0], assignments[1])
}

fn get_ordered_assignments(assignments_elf_1: (u32, u32), assignments_elf_2: (u32, u32)) -> ((u32, u32), (u32, u32)) {
    if assignments_elf_1.0 > assignments_elf_2.0 || (assignments_elf_1.0 == assignments_elf_2.0 && assignments_elf_1.1 < assignments_elf_2.1) {
        return (assignments_elf_2, assignments_elf_1);
    }
    (assignments_elf_1, assignments_elf_2)
}

fn assignment_contains_the_other(early_assignment: (u32, u32), late_assignment: (u32, u32)) -> bool {
    late_assignment.0 <= early_assignment.1 && early_assignment.1 >= late_assignment.1
}