use AoC_2022::text_file_reader::TextFileReader;



fn main() {
    println!("Puzzle du 04/12 Partie 2");
    
    let puzzle = get_puzzle();
    let count_pairs_with_same_assignments = get_count_pairs_with_same_assignments(puzzle);
    println!("Total d'assignement se superposant : {count_pairs_with_same_assignments}");
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("04_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn get_count_pairs_with_same_assignments(vec_puzzle: Vec<String>) -> u32 {
    let mut count = 0;
    
    for pair in vec_puzzle {
        let assignments_by_pair: Vec<&str> = pair.split(",").collect();
        let assignments_elf_1: (u32, u32) = get_assignements_by_elf(assignments_by_pair[0]);
        let assignments_elf_2: (u32, u32) = get_assignements_by_elf(assignments_by_pair[1]);
        
        let (early_assignment, late_assignment) = get_ordered_assignments(assignments_elf_1, assignments_elf_2);
        
        if assignments_overlaps(early_assignment, late_assignment) {
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

fn assignments_overlaps(early_assignment: (u32, u32), late_assignment: (u32, u32)) -> bool {
    late_assignment.0 <= early_assignment.1
}