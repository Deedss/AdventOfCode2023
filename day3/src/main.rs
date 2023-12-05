use std::collections::btree_map::IterMut;

#[allow(dead_code)]
// Idea
// Split a line on . and store values in a struct with index and value
// Get all values from a line and their indices
// Get all indices of symbols and check
#[derive(Debug, Clone)]
struct Item {
    first_index: usize,
    last_index: usize,
    value: String,
}

impl Item {
    fn new(index: usize, value: String) -> Self {
        let last_index = index - 1;
        let first_index = index - value.len();
        Item {
            first_index,
            last_index,
            value,
        }
    }

    fn is_symbol(&self) -> bool {
        self.value.chars().all(|c| !c.is_numeric())
    }

    fn is_number(&self) -> bool {
        self.value.chars().all(|c| c.is_numeric())
    }
}

fn parse_row(input: &str) -> Vec<Item> {
    let mut items = Vec::new();
    let mut value = String::new();

    for (i, c) in input.char_indices() {
        if c == '.' && !value.is_empty() {
            items.push(Item::new(i, value));
            value = String::new(); // Re-initialize the string
        } else if c != '.' {
            if c.is_numeric() {
                value.push(c);
            } else {
                if !value.is_empty() {
                    items.push(Item::new(i, value));
                    value = String::new(); // Re-initialize the string
                }
                value.push(c);
                items.push(Item::new(i + 1, value));
                value = String::new(); // Re-initialize the string
            }
        }
    }

    // Handle the final value if it's not empty and not ended with a dot
    if !value.is_empty() {
        items.push(Item::new(input.len(), value));
    }

    items
}

fn has_matching_neighbour(curr: &Vec<Item>, value: &Item, j: usize) -> bool {
    let mut has_neighbour = false;
    let prev_val = curr.get(j.wrapping_sub(1));
    let next_val = curr.get(j.wrapping_add(1));

    has_neighbour = has_neighbour
        || prev_val.is_some_and(|v| v.is_symbol() && v.last_index == (value.first_index - 1));
    has_neighbour = has_neighbour
        || next_val.is_some_and(|v| v.is_symbol() && v.first_index == (value.last_index + 1));

    has_neighbour
}

fn has_match_in_line(line: Option<&Vec<Item>>, value: &Item) -> bool {
    line.and_then(|l| {
        l.iter()
            .find(|&v| {
                if (value.first_index == 0) {
                    v.last_index <= (value.last_index + 1)
                } else {
                    v.first_index >= (value.first_index - 1)
                        && v.last_index <= (value.last_index + 1)
                }
            })
            .filter(|v| v.is_symbol())
    })
    .is_some()
}

fn check_numbers(lines: &[Vec<Item>]) -> u32 {
    let mut total_sum = 0;

    for (i, row) in lines.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if value.is_number() {
                let mut has_edging_symbol = false;
                has_edging_symbol = has_edging_symbol || has_matching_neighbour(row, value, j);
                if i == 0 {
                    has_edging_symbol =
                        has_edging_symbol || has_match_in_line(lines.get(i + 1), value);
                } else if i == lines.len() {
                    has_edging_symbol =
                        has_edging_symbol || has_match_in_line(lines.get(i - 1), value);
                } else {
                    has_edging_symbol =
                        has_edging_symbol || has_match_in_line(lines.get(i - 1), value);
                    has_edging_symbol =
                        has_edging_symbol || has_match_in_line(lines.get(i + 1), value);
                }

                if has_edging_symbol {
                    if let Ok(num) = value.value.parse::<u32>() {
                        total_sum += num;
                    }
                }
            }
        }
    }

    println!("{}", total_sum);

    total_sum
}

fn part_1(input: &str) -> u32 {
    let rows: Vec<Vec<Item>> = input.lines().map(|row| parse_row(row)).collect();

    check_numbers(&rows)
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