use std::collections::{linked_list, LinkedList};

#[allow(dead_code)]
// Idea
// Split a line on . and store values in a struct with index and value
// Get all values from a line and their indices
// Get all indices of symbols and check
#[derive(Debug, Clone)]
struct Value {
    first_index: usize,
    last_index: usize,
    value: String,
    is_symbol: bool,
    added_to_sum: bool,
}

fn get_values(input: &str) -> Vec<Value> {
    let mut values = Vec::new();

    let mut symbol = String::new();
    let mut number = String::new();

    for (index, char) in input.char_indices() {
        if char == '.' {
            if !symbol.is_empty() {
                let val = Value {
                    first_index: index - symbol.len(),
                    last_index: index - 1,
                    value: symbol.clone(),
                    is_symbol: true,
                    added_to_sum: false,
                };
                values.push(val);
                symbol.clear();
            } else if !number.is_empty() {
                let val = Value {
                    first_index: index - number.len(),
                    last_index: index - 1,
                    value: number.clone(),
                    is_symbol: false,
                    added_to_sum: false,
                };
                values.push(val);
                number.clear();
            }
        } else {
            if char.is_numeric() {
                number.push(char);
            } else {
                symbol.push(char);
            }
        }
    }

    values
}

fn check_if_matches(prev: &Value, cur: &Value) -> bool {
    let min_index: usize = match cur.first_index {
        0 => 0,
        _ => cur.first_index - 1,
    };

    let max_index: usize = cur.last_index + 1;
    if prev.is_symbol {
        if prev.first_index >= min_index && prev.last_index <= max_index {
            return true;
        }
    }
    false
}

fn compare_against_prev(prev: &Vec<Value>, current: &mut Vec<Value>) {
    for cur in current.iter_mut() {
        if !cur.is_symbol && !cur.added_to_sum {
            for pre in prev.iter() {
                if check_if_matches(&pre, cur) {
                    cur.added_to_sum = true
                }
            }
        }
    }
}

fn compare_against_next(next: &Vec<Value>, current: &mut Vec<Value>) {
    for cur in current.iter_mut() {
        if !cur.is_symbol && !cur.added_to_sum {
            for n in next.iter() {
                if check_if_matches(&n, cur) {
                    cur.added_to_sum = true
                }
            }
        }
    }
}

fn go_over_list(rows: &mut Vec<Vec<Value>>) -> u32 {
    let temp = rows.clone();
    for (index, row) in rows.iter_mut().enumerate() {
        if index == 0 {
            compare_against_next(&temp[index + 1], row);
        } else if index == temp.len() - 1 {
            compare_against_prev(&temp[index - 1], row);
        } else {
            compare_against_prev(&temp[index - 1], row);
            compare_against_next(&temp[index + 1], row);
        }
    }

    // Flatten, filter, and sum the values
    let result: Vec<Value> = rows
        .iter()
        .flat_map(|row| row.iter())
        .filter(|val| val.added_to_sum)
        .cloned() // Clone each Value (assuming Value is Clone)
        .collect();

    let mut sum = 0u32;

    for value in result {
        match value.value.parse::<u32>() {
            Ok(num) => sum += num,
            Err(_) => eprintln!("Failed to parse '{}' as a number.", value.value),
        }
        println!("{}", sum);
    }

    sum
}

fn part_1(input: &str) -> u32 {
    let mut rows: Vec<Vec<Value>> = Vec::new();

    for line in input.lines() {
        rows.push(get_values(line));
    }

    go_over_list(&mut rows);

    let sum = 0;

    sum
}

fn part_2(input: &str) -> u32 {
    let sum = 0;

    sum
}

fn main() {
    let example1 = part_1(include_str!("sample.txt"));
    println!("{}", example1);

    let part1 = part_1(include_str!("input.txt"));
    println!("{}", part1);
    //
    // let example2 = part_2(include_str!("sample.txt"));
    // println!("{}", example2);
    //
    // let part2 = part_2(include_str!("input.txt"));
    // println!("{}", part2);
}
