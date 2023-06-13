use regex::Regex;

use AoC_2022::text_file_reader::TextFileReader;

#[derive(Debug, PartialEq)]
enum Order {
    LEFT, RIGHT, SAME
}

fn main() {
    println!("Puzzle du 13/12 Partie 1");
    
    let puzzle = get_puzzle();
    let pairs = get_pairs(puzzle);
    let ordered_pair_counter = get_count_of_ordered_pairs(pairs);
    println!("nombre de paires ordonnées : {ordered_pair_counter}");
}

fn get_puzzle() -> Vec<String> {
    let mut text_file_reader = TextFileReader::new("13_12.txt");
    text_file_reader.read_file_text().expect("Lecture de l'entrée réussie");
    text_file_reader.get_content_as_list_split_by_newline()
}

fn get_pairs(puzzle: Vec<String>) -> Vec<(String, String)> {
    let mut pairs = Vec::new();
    
    let mut iter_puzzle = puzzle.iter();

    loop {
        let left = match iter_puzzle.next() {
            Some(pair) => {
                if pair.is_empty() {
                    continue;
                }
                pair.clone()
            },
            None => break
        };

        let right = iter_puzzle.next().unwrap().clone();
        pairs.push((left, right));
    }

    pairs
}

fn get_count_of_ordered_pairs(pairs: Vec<(String, String)>) -> u32 {
    let mut ordered_pair_counter = 0;
    for (index, pair) in pairs.into_iter().enumerate() {
        if inspect_pair(pair) == Order::LEFT {
            ordered_pair_counter += index as u32 + 1;
        }
    }
    
    ordered_pair_counter
}


fn  inspect_pair(pair: (String, String)) -> Order {
    let left_signal = get_comparable_pair(pair.0);
    let right_signal = get_comparable_pair(pair.1);

    let left_split = get_signal_values(left_signal);
    let right_split = get_signal_values(right_signal);

    let mut left_split_iter = left_split.iter();
    let mut right_split_iter = right_split.iter();

    let pattern_of_inner_signal = Regex::new(r"[\[\]]").unwrap();

    loop {
        let mut left_value = match left_split_iter.next() {
            Some(value) => value.to_string(),
            None => "".to_string()
        };

        let mut right_value = match right_split_iter.next() {
            Some(value) => value.to_string(),
            None => "".to_string()
        };

        if left_value.is_empty() || right_value.is_empty() {
            if left_value.is_empty() && right_value.is_empty() {
                return Order::SAME;
            } else {
                if left_value.is_empty() {
                    return Order::LEFT;
                } else {
                    return Order::RIGHT;
                }
            }
        }

        let left_value_is_a_inner_signal = pattern_of_inner_signal.is_match(&left_value);
        let right_value_is_a_inner_signal = pattern_of_inner_signal.is_match(&right_value);

        if !left_value_is_a_inner_signal && !right_value_is_a_inner_signal {
            let left_value_u32: u32 = left_value.parse().unwrap();
            let right_value_u32: u32 = right_value.parse().unwrap();
            
            if left_value_u32 < right_value_u32 {
                return Order::LEFT;
            } else if left_value_u32 > right_value_u32 {
                return Order::RIGHT;
            }
        } else if left_value_is_a_inner_signal && right_value_is_a_inner_signal {
            let inner_signal_order = inspect_pair((left_value, right_value));
            if inner_signal_order != Order::SAME {
                return inner_signal_order;
            }
        } else {
            if left_value_is_a_inner_signal {
                right_value = format!("[{right_value}]");
            } else {
                left_value = format!("[{left_value}]");
            }
            let inner_signal_order = inspect_pair((left_value, right_value));
            if inner_signal_order != Order::SAME {
                return inner_signal_order;
            }
        }
    }
}

fn get_comparable_pair(mut pair: String) -> String {
    pair.remove(0);
    pair.pop();
    pair
}

fn get_signal_values(signal: String) -> Vec<String> {
    let mut values = vec![];

    let mut chars_iter = signal.char_indices();
    let mut is_inner_signal = false;
    let mut inner_signal_counter = 0;
    let mut inner_signal_start = 0;

    loop {
        match chars_iter.next() {
            Some((index, c)) => {
                if c.is_ascii_digit() {
                    if !is_inner_signal {
                        match chars_iter.next() {
                            Some((_, following_c)) => {
                                if following_c.is_ascii_digit() {
                                    values.push(format!("{c}{following_c}"));
                                } else if following_c == ',' {
                                    values.push(c.to_string())
                                }
                            },
                            None => values.push(c.to_string())
                        }
                    }
                } else if c == '[' {
                    if !is_inner_signal {
                        is_inner_signal = true;
                        inner_signal_start = index;
                    } else {
                        inner_signal_counter += 1;
                    }
                } else if c == ']' {
                    if inner_signal_counter > 0 {
                        inner_signal_counter -= 1;
                    } else {
                        is_inner_signal = false;
                        let inner_signal = signal[inner_signal_start..=index].chars().collect();
                        values.push(inner_signal);
                    }
                }
            },
            None => break
        }
    }
    values
}