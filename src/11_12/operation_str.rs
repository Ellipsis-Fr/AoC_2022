pub fn string_operation(operator: &str, operand_one: &str, operand_two: &str) -> String {
    let length_longest_operand = if operand_one.len() > operand_two.len() {
        operand_one.len()
    } else {
        operand_two.len()
    };

    let operand_one = &operand_one.chars().rev().collect::<String>();
    let operand_two = &operand_two.chars().rev().collect::<String>();

    let mut result = "".to_string();
    let mut a_ten = 0;
    for index in 0..length_longest_operand {
        let first_unit = operand_one.chars().nth(index).unwrap_or('0').to_digit(10).unwrap();
        let second_unit = operand_two.chars().nth(index).unwrap_or('0').to_digit(10).unwrap();

        if (first_unit == 0 || second_unit == 0) && a_ten == 0 {
            if first_unit == 0 {
                let mut remaining_digits = operand_two.to_string();
                remaining_digits.drain(..index);
                result.push_str(&remaining_digits);
            } else {
                let mut remaining_digits = operand_one.to_string();
                // println!("init of remaining_digits : {remaining_digits}");
                remaining_digits.drain(..index);
                // println!("remaining_digits : {remaining_digits}");
                result.push_str(&remaining_digits);
            }
            break;
        }
        let mut sum = first_unit + second_unit + a_ten;
        a_ten = 0;
        
        if sum > 9 {
            a_ten = 1;
            sum %= 10;
        }

        result.push_str(&sum.to_string());
    }

    result.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_operation_addition() {
        let result = string_operation("+", "118", "1319");
        
        assert_eq!(result, String::from("1437"));
    }
}