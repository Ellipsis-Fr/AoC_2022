use std::{fmt, collections::VecDeque};
use operation_str::string_operation;

use crate::operation_str;

const STARTING_ITEMS: &str = "Starting items: ";
const OPERATION: &str = "Operation: ";
const TEST: &str = "Test: divisible by ";
const IF_TRUE: &str = "If true: throw to monkey ";
const IF_FALSE: &str = "If false: throw to monkey ";

#[derive(Debug)]
pub struct Monkey {
    pub items: VecDeque<String>,
    operation: Operation,
    test: Test,
    pub inspection_counter: u32
}

impl Monkey {
    pub fn new(monkey_definition: Vec<String>) -> Self {
        let items = Self::get_items(monkey_definition[0].clone().trim().to_string());
        let operation = Self::get_operation(monkey_definition[1].clone().trim().to_string());
        let test = Test::new(&monkey_definition[2..]);
        
        Monkey {
            items,
            operation,
            test,
            inspection_counter: 0
        }
    }

    fn get_items(mut items_str: String) -> VecDeque<String> {
        items_str.drain(..STARTING_ITEMS.len());
        // println!("{items_str}");
        if items_str.contains(",") {
            items_str.split(", ").into_iter().map(|i| i.trim().to_string()).collect()
        } else {
            let mut items = VecDeque::new();
            items.push_back(items_str.trim().to_string());
            items
        }
    }

    fn get_operation(mut operation_str: String) -> Operation {
        let word_to_find_then_skip = "old ";
        let index_to_start = operation_str.find(word_to_find_then_skip).unwrap() + word_to_find_then_skip.len();
        operation_str.drain(..index_to_start);
        
        let operation_elements = operation_str.split_whitespace().collect::<Vec<&str>>();
        
        match operation_elements[1] {
            "old" => {
                if operation_elements[0] == "+" {
                    // println!("closure : old + old");
                    Operation {
                        operator: String::from("+"),
                        operand_with: String::from("old")
                    }
                } else {
                    // println!("closure : old * old");
                    Operation {
                        operator: String::from("*"),
                        operand_with: String::from("old")
                    }
                }
            },
            _ => {
                let y = operation_elements[1];
                if operation_elements[0] == "+" {
                    // println!("closure : old + {y}");
                    Operation {
                        operator: String::from("+"),
                        operand_with: String::from(y)
                    }
                } else {
                    // println!("closure : old * {y}");
                    Operation {
                        operator: String::from("*"),
                        operand_with: String::from(y)
                    }
                }
            }
        }

    }

    pub fn operates(&mut self) {
        self.inspection_counter += 1;
        let item = &self.items[0];
        self.items[0] = string_operation(&self.operation.operator, item, &self.operation.operand_with);
    }

    pub fn next_monkey(&self) -> u32 {
        let mut item = &self.items[0];
        self.test.test(item)
    }
}

#[derive(Debug)]
struct Operation {
    operator: String,
    operand_with: String
}

#[derive(Debug)]
struct Test {
    divider: u32,
    if_successful: u32,
    if_failed: u32
}

impl Test {
    fn new(test_definition: &[String]) -> Self {
        Test {
            divider: Self::get_divider(test_definition[0].clone().trim().to_string()),
            if_successful: Self::get_monkey_to_give_item(test_definition[1].clone().trim().to_string(), true),
            if_failed: Self::get_monkey_to_give_item(test_definition[2].clone().trim().to_string(), false)
        }
    }

    fn get_divider(mut test_case: String) -> u32 {
        test_case.drain(..TEST.len());
        test_case.parse().unwrap()
    }

    fn get_monkey_to_give_item(mut direction: String, case: bool) -> u32 {
        if case {
            direction.drain(..IF_TRUE.len());
        } else {
            direction.drain(..IF_FALSE.len());
        }

        direction.parse().unwrap()
    }

    fn test(&self, value: &str) -> u32 {
        // if value % self.divider == 0 {
        //     self.if_successful
        // } else {
        //     self.if_failed
        // }
        0
    }
}

