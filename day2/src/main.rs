const BLUE: u32 = 14;
const RED: u32 = 12;
const GREEN: u32 = 13;

#[derive(Debug)]
struct Draw {
    greens: u32,
    blues: u32,
    reds: u32,
}

impl Draw {
    fn new(input: &str) -> Result<Self, String> {
        let mut greens = 0;
        let mut blues = 0;
        let mut reds = 0;

        for part in input.split(',') {
            let parts: Vec<&str> = part.trim().split_whitespace().collect();
            let count: u32 = parts[0].parse::<u32>().unwrap();

            match parts[1] {
                "green" => greens = count,
                "blue" => blues = count,
                "red" => reds = count,
                _ => return Err("Invalid color".to_string()),
            }
        }

        Ok(Draw {
            greens,
            blues,
            reds,
        })
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
    valid: bool,
}

fn get_minimum_required(draws: &Vec<Draw>) -> Draw {
    let mut greens: u32 = 1;
    let mut reds: u32 = 1;
    let mut blues: u32 = 1;

    for draw in draws {
        if draw.blues > blues {
            blues = draw.blues;
        }
        if draw.greens > greens {
            greens = draw.greens;
        }
        if draw.reds > reds {
            reds = draw.reds;
        }
    }

    Draw {
        greens,
        blues,
        reds,
    }
}

fn is_valid(draws: &Vec<Draw>) -> bool {
    let valid = draws
        .iter()
        .all(|draw| draw.reds <= RED && draw.blues <= BLUE && draw.greens <= GREEN);

    valid
}

fn get_draws(input: &str) -> Vec<Draw> {
    let mut draws: Vec<Draw> = Vec::new();

    for s in input.split(";") {
        draws.push(Draw::new(s).unwrap());
    }

    draws
}

fn get_games(input: &str) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();

    // Split into Game and Draws
    let mut id: u32 = 1;

    for line in input.lines() {
        let draws = get_draws(line.split(":").last().unwrap());
        let valid = is_valid(&draws);

        games.push(Game { id, draws, valid });

        id += 1;
    }

    games
}

fn part_1(input: &str) -> u32 {
    // Count total games
    let games = get_games(input);

    let valid_games = games
        .iter()
        .filter(|game| game.valid)
        .map(|game| game.id)
        .sum();

    valid_games
}

fn part_2(input: &str) -> u32 {
    let games = get_games(input);
    let min_per_game: Vec<Draw> = games
        .iter()
        .map(|game| get_minimum_required(&game.draws))
        .collect();

    let sum = min_per_game
        .iter()
        .map(|draw| draw.blues * draw.greens * draw.reds)
        .sum();

    sum
}

fn main() {
    let sample_part1 = part_1(include_str!("sample.txt").trim());
    println!("{}", sample_part1);

    let part_1 = part_1(include_str!("input.txt").trim());
    println!("{}", part_1);

    let s_part2 = part_2(include_str!("sample.txt").trim());
    println!("{}", s_part2);

    let part2 = part_2(include_str!("input.txt").trim());
    println!("{}", part2);
}
