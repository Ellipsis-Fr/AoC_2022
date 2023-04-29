use AoC_2022::text_file_reader::TextFileReader;

fn main() {
    println!("Puzzle du 08/12 Partie 1");
    
    let puzzle = get_puzzle();
    let (trees_ns, trees_we) = get_trees(puzzle);
    let visible_trees_count = count_visible_trees(trees_ns, trees_we);
    println!("nombre d'arbres visibles : {visible_trees_count}")
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

fn count_visible_trees(trees_ns: Vec<String>, trees_we: Vec<String>) -> u32 {
    let count_all_trees = (trees_ns.len() as u32) * (trees_ns[0].len() as u32);
    let mut count_hidden_trees = 0;
    let mut positions_tallest_trees_in_y = vec![];

    for (range_y, trees) in trees_ns.iter().enumerate() {
        if range_y == 0 || range_y == trees_ns.len() - 1 { continue; }
        
        let mut position_tallest_trees_in_x = (0, 0);
        let trees_in_x = trees.split("").filter_map(|c| c.parse().ok()).collect::<Vec<i32>>();
        
        for (range_x, tree) in trees_in_x.iter().enumerate() {
            if range_x == 0 || range_x == trees.len() - 1 { continue; }

            let is_hidden_in_x = check_if_hidden_and_update_positions_tallest_trees(*tree, range_x, &mut position_tallest_trees_in_x, &trees_in_x);
            let is_hidden_in_y;

            match positions_tallest_trees_in_y.get_mut(range_x - 1) {
                None => {
                    let mut position_tallest_trees_in_y = (0, 0);

                    let trees_in_y = trees_we[trees_ns.len() - 1 - range_x].split("").filter_map(|c| c.parse().ok()).collect::<Vec<i32>>();
                    is_hidden_in_y = check_if_hidden_and_update_positions_tallest_trees(*tree, range_y, &mut position_tallest_trees_in_y, &trees_in_y);

                    positions_tallest_trees_in_y.push(position_tallest_trees_in_y);
                },
                Some(position_tallest_trees_in_y) => {
                    let trees_in_y = trees_we[trees_ns.len() - 1 - range_x].split("").filter_map(|c| c.parse().ok()).collect::<Vec<i32>>();
                    is_hidden_in_y = check_if_hidden_and_update_positions_tallest_trees(*tree, range_y, position_tallest_trees_in_y, &trees_in_y);
                }        
            }

            if is_hidden_in_x && is_hidden_in_y { count_hidden_trees += 1; }
        }
    }

    println!("nombre d'arbres totaux : {count_all_trees}");
    println!("nombre d'arbres cachés : {count_hidden_trees}");

    count_all_trees - count_hidden_trees
}

fn check_if_hidden_and_update_positions_tallest_trees(tree_height: i32, tree_position: usize, mut position_tallest_trees: &mut (usize, usize), trees: &Vec<i32>) -> bool {
    let mut is_hidden = true;

    if tree_height > trees[position_tallest_trees.0] {
        is_hidden = false;
        position_tallest_trees.0 = tree_position;
    }

    if tree_position >= position_tallest_trees.1 {
        position_tallest_trees.1 = tree_position + get_position_of_next_tallest_tree(&trees[tree_position + 1..]);
        if tree_height > trees[position_tallest_trees.1] { is_hidden = false; }
    }

    is_hidden
}

fn get_position_of_next_tallest_tree(trees: &[i32]) -> usize {
    trees.iter().enumerate().max_by_key(|&(_, x)| x).unwrap().0 + 1
}