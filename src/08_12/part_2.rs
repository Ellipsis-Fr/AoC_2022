use AoC_2022::text_file_reader::TextFileReader;

fn main() {
    println!("Puzzle du 08/12 Partie 2");
    
    let puzzle = get_puzzle();
    let (trees_ns, trees_we) = get_trees(puzzle);
    let highest_scenic_score = get_highest_scenic_score(trees_ns, trees_we);
    println!("valeur panoramique la plus élevée : {highest_scenic_score}")
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("08_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn get_trees(puzzle: Vec<String>) -> (Vec<String>, Vec<String>) {
    let trees_ns = puzzle.clone();
    let mut trees_we: Vec<String> = Vec::new();

    for trees in puzzle {
        let trees_height = trees.split("").filter(|v| v.len() > 0).collect::<Vec<&str>>();
        for (index, tree_height) in trees_height.into_iter().enumerate() {
            match trees_we.get_mut(index) {
                Some(trees_height) => trees_height.push_str(tree_height),
                None => trees_we.insert(index, String::from(tree_height))
            }
        }
    }

    trees_we.reverse();

    (trees_ns, trees_we)
}

fn get_highest_scenic_score(trees_ns: Vec<String>, trees_we: Vec<String>) -> u32 {
    let mut scenic_score = 0;

    for (range_y, trees) in trees_ns.iter().enumerate() {
        if range_y == 0 || range_y == trees_ns.len() - 1 { continue; }
        
        let trees_in_x = trees.split("").filter_map(|c| c.parse().ok()).collect::<Vec<i32>>();
        
        for (range_x, tree) in trees_in_x.iter().enumerate() {
            if range_x == 0 || range_x == trees.len() - 1 { continue; }

            let scenic_score_in_x = get_scenic_score_and_update_positions_tallest_trees(*tree, range_x, &trees_in_x);
            
            let trees_in_y = trees_we[trees_ns.len() - 1 - range_x].split("").filter_map(|c| c.parse().ok()).collect::<Vec<i32>>();
            let scenic_score_in_y = get_scenic_score_and_update_positions_tallest_trees(*tree, range_y, &trees_in_y);

            if scenic_score_in_x * scenic_score_in_y > scenic_score { scenic_score = scenic_score_in_x * scenic_score_in_y; }
        }
    }

    scenic_score
}

fn get_scenic_score_and_update_positions_tallest_trees(tree_height: i32, tree_position: usize, trees: &Vec<i32>) -> u32 {
    let mut score_scenic;
    
    let previous_position_of_tree_at_leat_as_tall_as_current_tree = get_position_of_tree_at_leat_as_tall_as_current_tree(tree_height, tree_position, &trees[..tree_position], "left");
    score_scenic = (tree_position - previous_position_of_tree_at_leat_as_tall_as_current_tree) as u32;

    let next_position_of_tree_at_leat_as_tall_as_current_tree = get_position_of_tree_at_leat_as_tall_as_current_tree(tree_height, tree_position, &trees[tree_position + 1..], "right");
    score_scenic *= (next_position_of_tree_at_leat_as_tall_as_current_tree - tree_position) as u32;

    score_scenic
}

fn get_position_of_tree_at_leat_as_tall_as_current_tree(tree_height: i32, tree_position: usize, trees: &[i32], side: &str) -> usize {
    let mut position = 0;

    for (index, height) in trees.iter().enumerate() {
        if height >= &tree_height {
            if side == "left" { position = index; }
            else { 
                position = tree_position + index + 1;
                break;
            }
        }
    }

    if side == "right" && position == 0 { position = tree_position + trees.len(); }
    position
}