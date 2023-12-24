use core::num;
use std::borrow::Borrow;

#[derive(Debug)]
struct Row {
    groups: Vec<usize>,
    line: String,
}

impl Row {
    fn new(line: &str) -> Self {
        let split: Vec<_> = line.split_whitespace().collect();

        Row {
            groups: split[1]
                .split(",")
                .map(|c| c.parse::<usize>().unwrap())
                .collect(),
            line: split[0].to_string(),
        }
    }
}

fn parse(input: &str) -> Vec<Row> {
    input.lines().map(|line| Row::new(line)).collect()
}

fn calculate_arrangement(line: &str, nums: &mut Vec<usize>) -> i64 {
    if line.len() == 0 {
        if nums.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }

    if nums.is_empty() {
        if line.contains("#") {
            return 1;
        } else {
            return 0;
        }
    }

    let mut sum = 0;

    if ".?".contains(line.chars().nth(0).unwrap()) {
        sum += calculate_arrangement(&line[1..], nums);
    }

    if "#?".contains(line.chars().nth(0).unwrap()) {
        if nums.len() <= line.len()
            && line[..nums[0]].contains('.')
            && (nums[0] == line.len() || line.chars().nth(nums[0]).unwrap() == '#')
        {
            sum += calculate_arrangement(&line[nums[0] + 1..], nums);
        }
    }

    sum
}

fn part_1(input: &str) -> i64 {
    let mut rows = parse(input);
    let mut sum = 0;
    for row in rows.iter_mut() {
        println!("{:?}", row);
        sum += calculate_arrangement(&row.line, &mut row.groups);
    }
    sum
}

fn part_2(input: &str) -> i64 {
    0
}

fn main() {
    let example_1 = part_1(include_str!("example.input"));
    println!("example 1 = {}", example_1);
    assert_eq!(example_1, 21);

    let part_1 = part_1(include_str!("input.input"));
    println!("part_1 = {}", part_1);

    // let example_2 = part_2(include_str!("example.input"));
    // println!("example 2 = {}", example_2);
    // assert_eq!(example_2, 1030);
    //
    // let part_2 = part_2(include_str!("input.input"));
    // println!("part_2 = {}", part_2);
}
