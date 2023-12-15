#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Range {
    dest: i64,
    src: i64,
    range: i64,
}

impl Range {
    fn new(dest: i64, src: i64, range: i64) -> Self {
        Range { dest, src, range }
    }
}

fn parse_map(input: &str) -> Vec<Vec<Range>> {
    input
        .split("\n\n")
        .map(|section| {
            section
                .lines()
                .skip(1)
                .map(|line| {
                    let mut values = line.split_whitespace().map(|s| s.parse::<i64>().unwrap());
                    let dest = values.next().unwrap();
                    let src = values.next().unwrap();
                    let range = values.next().unwrap();
                    Range::new(dest, src, range)
                })
                .collect()
        })
        .collect()
}

fn parse_seeds(input: &str) -> Vec<i64> {
    let line = input.lines().find(|l| l.starts_with("seeds:"));
    let seeds: Vec<i64> = line
        .unwrap()
        .split_whitespace()
        .filter(|&s| !s.starts_with("seeds:"))
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    seeds
}

fn part_1(input: &str) -> i64 {
    let mut seeds = parse_seeds(input);
    let mut map = parse_map(input);
    let mut lowest = i64::MAX;

    for seed in seeds.iter_mut() {
        for ranges in map.iter_mut() {
            for range in ranges.iter_mut() {
                if seed >= &mut range.src && seed <= &mut (range.src + range.range - 1) {
                    *seed = *seed + range.dest - range.src;
                    break;
                }
            }
        }
        println!("seed:{}", seed);

        if seed < &mut lowest {
            lowest = *seed;
        }
    }

    lowest
}

fn part_2(input: &str) -> i64 {
    let mut seeds = parse_seeds(input);
    let mut new_seeds: Vec<i64> = Vec::new();
    let mut iter = seeds.iter_mut();

    while let Some(seed) = iter.next() {
        if let Some(next_seed) = iter.next() {
            let tmp: Vec<_> = (*seed..(*seed + *next_seed + 1)).collect();
            new_seeds.extend(tmp);
        }
    }

    let mut map = parse_map(input);
    let mut lowest = i64::MAX;

    for seed in new_seeds.iter_mut() {
        for ranges in map.iter_mut() {
            for range in ranges.iter_mut() {
                if seed >= &mut range.src && seed <= &mut (range.src + range.range - 1) {
                    *seed = *seed + range.dest - range.src;
                    break;
                }
            }
        }

        if seed < &mut lowest {
            lowest = *seed;
        }
    }

    lowest
}

fn main() {
    let example_1 = part_1(include_str!("example.txt"));
    println!("Example part 1 : {}", example_1);
    let part_1 = part_1(include_str!("input.txt"));
    println!("Input part 1 : {}", part_1);

    let example_2 = part_2(include_str!("example.txt"));
    println!("Example part 2 : {}", example_2);
    let part_2 = part_2(include_str!("input.txt"));
    println!("Input part 2 : {}", part_2);
}
