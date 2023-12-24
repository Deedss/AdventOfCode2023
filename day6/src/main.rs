#[allow(dead_code)]

fn get_values(input: &str, nth: usize) -> Vec<i64> {
    input
        .lines()
        .nth(nth)
        .unwrap()
        .split_whitespace()
        .filter(|s| s.chars().any(char::is_numeric))
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

fn get_val_part2(input: &str, nth: usize) -> i64 {
    let string: String = input
        .lines()
        .nth(nth)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    string.parse::<i64>().unwrap()
}

fn get_possible_solutions(time: i64, distance: i64) -> i64 {
    let mut solution: i64 = 0;

    for i in 1..(time - 1) {
        let remaining_time = time - i;
        if i * remaining_time > distance {
            solution += 1;
        }
    }

    solution
}

fn part_1(input: &str) -> i64 {
    let times = get_values(input, 0);
    let distances = get_values(input, 1);
    let mut solutions: Vec<i64> = Vec::new();

    for i in 0..times.len() {
        solutions.insert(i, get_possible_solutions(times[i], distances[i]))
    }

    solutions.iter().fold(1, |acc, &x| acc * x)
}

fn part_2(input: &str) -> i64 {
    let time = get_val_part2(input, 0);
    let distance = get_val_part2(input, 1);

    get_possible_solutions(time, distance) as i64
}

fn main() {
    let example_1 = part_1(include_str!("example.input"));
    println!("Example part 1 : {}", example_1);
    let part_1 = part_1(include_str!("input.input"));
    println!("Input part 1 : {}", part_1);

    let example_2 = part_2(include_str!("example.input"));
    println!("Example part 2 : {}", example_2);
    let part_2 = part_2(include_str!("input.input"));
    println!("Input part 2 : {}", part_2);
}
