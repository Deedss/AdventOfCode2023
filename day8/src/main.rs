use num_integer::Integer;
use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
    Right,
    Left,
}

#[derive(Debug, Clone)]
struct Destination {
    left: String,
    right: String,
}

impl Destination {
    fn new(input: &str) -> Self {
        let split: String = input
            .trim()
            .replace("(", "")
            .replace(")", "")
            .split(",")
            .collect();

        Destination {
            left: split[0..3].to_string(),
            right: split[4..split.len()].to_string(),
        }
    }
}

fn get_moves(input: &str) -> Vec<Direction> {
    input
        .lines()
        .nth(0)
        .unwrap()
        .chars()
        .map(|c| match &c {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!("Invalid character:{}", c),
        })
        .collect()
}

fn get_map(input: &str) -> HashMap<String, Destination> {
    let mut values = HashMap::new();
    input.lines().skip(2).for_each(|line| {
        let split: Vec<_> = line.split("=").collect();
        values.insert(split[0].trim().to_string(), Destination::new(split[1]));
    });
    values
}

fn part_1(input: &str) -> i64 {
    let moves = get_moves(input);
    let values = get_map(input);
    let mut current = values.get_key_value("AAA").unwrap();
    let mut steps = 0;
    let mut moves_cycle = moves.iter().cycle();

    while current.0 != "ZZZ" {
        match moves_cycle.next() {
            Some(Direction::Right) => {
                current = values.get_key_value(current.1.right.as_str()).unwrap()
            }
            Some(Direction::Left) => {
                current = values.get_key_value(current.1.left.as_str()).unwrap()
            }
            _ => continue,
        }
        steps += 1;
    }
    steps
}

fn get_all_nodes_ending_with_a(input: &str) -> Vec<(String, Destination)> {
    let mut nodes = Vec::new();
    input.lines().skip(2).for_each(|l| {
        let split: Vec<_> = l.split("=").collect();
        if let Some('A') = split[0].trim().chars().last() {
            nodes.push((split[0].trim().to_string(), Destination::new(split[1])));
        }
    });
    nodes
}

fn find_common_denominator(numbers: &[i64]) -> Option<i64> {
    // Ensure the list is not empty
    if numbers.is_empty() {
        return None;
    }

    // Use the fold method to find the common denominator
    let common_denominator = numbers[1..]
        .iter()
        .fold(numbers[0], |acc, &num| acc.lcm(&num));

    Some(common_denominator)
}

fn part_2(input: &str) -> i64 {
    let moves = get_moves(input);
    let values = get_map(input);
    let starting_nodes = get_all_nodes_ending_with_a(input);
    let mut nodes = Vec::new();
    starting_nodes
        .iter()
        .for_each(|n| nodes.push(values.get_key_value(&n.0).unwrap()));
    let mut steps: Vec<i64> = vec![];

    for node in nodes.iter_mut() {
        let mut moves_cycle = moves.iter().cycle();
        let mut step = 0;
        let current_node = node;

        while current_node.0.chars().last().unwrap() != 'Z' {
            match moves_cycle.next() {
                Some(Direction::Right) => {
                    *current_node = values.get_key_value(current_node.1.right.as_str()).unwrap()
                }
                Some(Direction::Left) => {
                    *current_node = values.get_key_value(current_node.1.left.as_str()).unwrap()
                }
                _ => continue,
            }
            step += 1;
        }

        steps.push(step);
    }

    find_common_denominator(&steps).unwrap()
}

fn main() {
    // let example_1 = part_1(include_str!("example1.txt"));
    // println!("steps example_1:{}", example_1);
    // assert_eq!(example_1, 2);
    //
    // let example_2 = part_1(include_str!("example2.txt"));
    // println!("steps example_2:{}", example_2);
    // assert_eq!(example_2, 6);
    //
    // let part_1 = part_1(include_str!("input.txt"));
    // println!("part_1:{}", part_1);

    let example_3 = part_2(include_str!("example3.txt"));
    println!("steps example_3:{}", example_3);
    assert_eq!(example_3, 6);

    let part_2 = part_2(include_str!("input.txt"));
    println!("steps part_2 : {}", part_2);
}
