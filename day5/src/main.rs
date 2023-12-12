use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
struct Seed {
    seed: u32,
    soil: u32,
    fertilizer: u32,
    water: u32,
    light: u32,
    temp: u32,
    humidity: u32,
    location: u32,
}

impl Seed {
    fn new(seed: u32) -> Self {
        Seed {
            seed,
            soil: 0,
            fertilizer: 0,
            water: 0,
            light: 0,
            temp: 0,
            humidity: 0,
            location: 0,
        }
    }
}

#[derive(Clone, Debug)]
struct Range {
    destination_start: u32,
    source_start: u32,
    range_len: u32,
}

impl Range {
    fn new(destination_start: u32, source_start: u32, range_len: u32) -> Self {
        Range {
            destination_start,
            source_start,
            range_len,
        }
    }
}

fn parse_map(input: &str) -> HashMap<String, Vec<Range>> {
    let mut map = HashMap::new();
    let mut ranges = Vec::new();
    let mut name = String::new();

    for line in input.lines() {
        if line.contains("map:") {
            if !name.is_empty() {
                map.insert(name.clone(), ranges.clone());
                ranges.clear();
            }
            name = line.to_string();
        } else if !line.is_empty() && !line.contains("seeds:") {
            let temp: Vec<u32> = line
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            ranges.push(Range::new(temp[0], temp[1], temp[2]));
        }

        if !name.is_empty() {
            map.insert(name.clone(), ranges.clone());
        }
    }

    map
}

fn parse_seeds(input: &str) -> Vec<Seed> {
    let line = input.lines().find(|l| l.starts_with("seeds:"));
    let seeds: Vec<Seed> = line
        .unwrap()
        .split_whitespace()
        .filter(|&s| !s.starts_with("seeds:"))
        .map(|s| Seed::new(s.parse::<u32>().unwrap()))
        .collect();

    seeds
}

fn part_1(input: &str) -> u32 {
    let seeds: Vec<Seed> = parse_seeds(input);
    let map = parse_map(input);

    println!("seeds: {:?}", seeds);
    for i in map.iter() {
        println!("{}:{:?}", i.0, i.1);
    }

    0
}

fn main() {
    let example_1 = part_1(include_str!("example.txt"));
    println!("Example part 1 : {}", example_1);
    // let part_1 = part_1(include_str!("input.txt"));
    // println!("Input part 1 : {}", part_1);
    //
    // let example_2 = part_2(include_str!("example.txt"));
    // println!("Example part 2 : {}", example_2);
    // let part_2 = part_2(include_str!("input.txt"));
    // println!("Input part 2 : {}", part_2);
}
