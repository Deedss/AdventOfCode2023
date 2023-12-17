#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Card {
    numbers: Vec<u32>,
    winning: Vec<u32>,
    matches: u32,
    points: u32,
}

impl Card {
    fn new(numbers: Vec<u32>, winning: Vec<u32>) -> Self {
        let matches: u32 = winning
            .iter()
            .filter(|&v| numbers.contains(v))
            .count()
            .try_into()
            .unwrap();
        let points: u32 = if matches > 0 {
            2u32.pow(matches - 1)
        } else {
            0u32
        };
        Card {
            numbers,
            winning,
            matches,
            points,
        }
    }
}

fn parse_card(input: &str) -> Card {
    let mut vars: Vec<_> = input.split(" ").collect();
    vars.retain(|&var| !var.contains("Card") && !var.contains(":"));

    let delimiter_index = vars.iter().position(|&v| v == "|").unwrap();
    let (before, after) = vars.split_at(delimiter_index);

    let numbers: Vec<u32> = before
        .iter()
        .filter_map(|&s| s.parse::<u32>().ok())
        .collect();

    let winning: Vec<u32> = after
        .iter()
        .filter_map(|&s| s.parse::<u32>().ok())
        .collect();

    Card::new(numbers, winning)
}

fn parse_cards(input: &str) -> Vec<Card> {
    input.lines().map(|row| parse_card(row)).collect()
}

fn part_1(input: &str) -> u32 {
    let cards = parse_cards(input);
    let total_sum = cards.iter().map(|card| card.points).sum();

    total_sum
}

fn part_2(input: &str) -> u32 {
    let cards = parse_cards(input);

    let mut counts = vec![1; cards.len()];
    for i in 0..cards.len() {
        for j in 0..cards[i].matches as usize {
            counts[i + j + 1] += counts[i];
        }
    }

    counts.iter().sum()
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
