use regex::Regex;

#[derive(Debug, PartialEq)]
enum Order {
    LEFT, RIGHT, SAME
}

#[derive(Debug)]
pub struct Tree {
    root: Option<TreeNode>,
}

impl Tree {
    pub fn new(signals: Vec<String>) -> Self {
        let mut tree = Tree { root: None };
        
        for signal in signals {
            tree.add_node(signal);
        }

        tree
    }

    pub fn add_node(&mut self, value: String) {
        let new_node = TreeNode::new(value);
        match self.root.as_mut() {
            Some(root) => root.find_where_to_add_node(new_node),
            None => self.root = Some(new_node)
        }       
    }

    pub fn get_list_of_sorted_node(&self) -> Vec<String> {
        let mut sorted_nodes = vec![];

        match self.root.as_ref() {
            Some(root) => self.fill_list_of_sorted_node(root, &mut sorted_nodes),
            None => ()
        }     
        
        sorted_nodes.reverse();
        sorted_nodes
    }

    fn fill_list_of_sorted_node(&self, current_node: &TreeNode, sorted_nodes: &mut Vec<String>) {
        if current_node.left.is_some() {
            self.fill_list_of_sorted_node(current_node.left.as_ref().unwrap(), sorted_nodes);
        }

        sorted_nodes.push(current_node.value.clone());

        if current_node.right.is_some() {
            self.fill_list_of_sorted_node(current_node.right.as_ref().unwrap(), sorted_nodes);
        }
        
    }
}

#[derive(Debug)]
struct TreeNode {
    value: String,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(value: String) -> Self {
        TreeNode { value, left: None, right: None }
    }

    fn find_where_to_add_node(&mut self, new_node: TreeNode) {
        match Self::cmp_nodes_value((self.value.clone(), new_node.value.clone())) {
            Order::LEFT | Order::SAME => {
                match self.left.as_mut() {
                    Some(left_node) => left_node.find_where_to_add_node(new_node),
                    None => self.left = Some(Box::new(new_node))
                }
            },
            Order::RIGHT => {
                match self.right.as_mut() {
                    Some(right_node) => right_node.find_where_to_add_node(new_node),
                    None => self.right = Some(Box::new(new_node))
                }
            }
        }
    }

    fn cmp_nodes_value(pair: (String, String)) -> Order {
        let left_signal = Self::get_comparable_pair(pair.0);
        let right_signal = Self::get_comparable_pair(pair.1);
    
        let left_split = Self::get_signal_values(left_signal);
        let right_split = Self::get_signal_values(right_signal);
    
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
                let inner_signal_order = Self::cmp_nodes_value((left_value, right_value));
                if inner_signal_order != Order::SAME {
                    return inner_signal_order;
                }
            } else {
                if left_value_is_a_inner_signal {
                    right_value = format!("[{right_value}]");
                } else {
                    left_value = format!("[{left_value}]");
                }
                let inner_signal_order = Self::cmp_nodes_value((left_value, right_value));
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
}