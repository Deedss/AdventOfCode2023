fn get_values(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>()
}

fn sum_of_extrapolated_values(values: &Vec<i64>) -> i64 {
    let mut new_values: Vec<Vec<i64>> = vec![values.clone()];
    let mut sum = 0;

    while new_values.iter().last().unwrap().iter().any(|&v| v != 0) {
        let mut next_vec: Vec<i64> = vec![];

        let mut iter = new_values.last().unwrap().windows(2);
        while let Some(window) = iter.next() {
            next_vec.push(window[1] - window[0]);
        }

        new_values.push(next_vec);
    }

    new_values.reverse();
    for i in 0..new_values.len() - 1 {
        let val = new_values[i + 1].iter().last().unwrap() + new_values[i].iter().last().unwrap();
        new_values[i + 1].push(val);
    }
    sum = *new_values.iter().last().unwrap().iter().last().unwrap();
    sum
}

fn part_1(input: &str) -> i64 {
    let values = get_values(input);
    let mut sum = 0;

    for val in values.iter() {
        sum += sum_of_extrapolated_values(val);
    }

    sum
}

fn part_2(input: &str) -> i64 {
    let mut values = get_values(input);
    let mut sum = 0;

    for val in values.iter_mut() {
        val.reverse();
        sum += sum_of_extrapolated_values(val);
    }
    sum
}

fn main() {
    let example1 = part_1(include_str!("example"));
    println!("Example 1: {}", example1);

    let part_1 = part_1(include_str!("input"));
    println!("Part 1: {}", part_1);

    let example2 = part_2(include_str!("example"));
    println!("Example 2: {}", example2);

    let part_2 = part_2(include_str!("input"));
    println!("Part 2: {}", part_2);
}
