fn main() {
    let contents = include_str!("../input.input").trim();

    let part2: u32 = contents.lines().map(process_line).sum();

    println!("{}", part2);
}

#[derive(Debug, Copy, Clone)]
struct Item {
    index: usize,
    num: u32,
}

fn process_line(input: &str) -> u32 {
    let searches = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("0", 0),
    ];

    let mut results: Vec<_> = searches
        .iter()
        .filter_map(|(word, num)| input.find(word).map(|index| Item { index, num: *num }))
        .collect();

    let mut reverse: Vec<_> = searches
        .iter()
        .filter_map(|(word, num)| input.rfind(word).map(|index| Item { index, num: *num }))
        .collect();

    results.append(&mut reverse);

    results.sort_by_key(|item| item.index);

    let value: u32 = results.first().unwrap().num * 10 + results.last().unwrap().num;

    value
}
