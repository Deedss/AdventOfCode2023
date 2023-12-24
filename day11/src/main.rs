use std::vec;

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Pair {
    start: Pos,
    end: Pos,
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn expand(galaxies: &Vec<Vec<char>>, times: i64) -> Vec<Vec<char>> {
    // Determine empty rows/colums
    let mut empty_rows: Vec<usize> = vec![];
    let mut empty_cols: Vec<usize> = vec![];

    //
    let mut temp: Vec<Vec<char>> = vec![vec!['.'; galaxies[0].len()]; galaxies.len()];

    for (x, row) in galaxies.iter().enumerate() {
        if row.iter().all(|&c| c == '.') {
            empty_rows.push(x);
        }
        for (y, cell) in row.iter().enumerate() {
            temp[y][x] = *cell;
        }
    }

    for (y, col) in temp.iter().enumerate() {
        if col.iter().all(|&c| c == '.') {
            empty_cols.push(y);
        }
    }

    let mut new_galaxies: Vec<Vec<char>> = vec![];

    // Expand original
    for (x, row) in galaxies.iter().enumerate() {
        let mut new_row: Vec<char> = vec![];
        for (y, cell) in row.iter().enumerate() {
            new_row.push(cell.clone());
            if empty_cols.contains(&y) {
                for i in 0..times {
                    new_row.push('.');
                }
            }
        }
        new_galaxies.push(new_row.clone());
        if empty_rows.contains(&x) {
            for i in 0..times {
                new_galaxies.push(vec![]);
            }
        }
        new_row.clear();
    }

    new_galaxies.to_vec()
}

fn get_pairs(galaxies: &Vec<Vec<char>>) -> Vec<Pair> {
    let mut positions = vec![];
    // Get all points with a #
    for (x, row) in galaxies.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            if '#' == *cell {
                positions.push(Pos { x, y });
            }
        }
    }

    // Get all pairs of points with a #
    let mut pairs = vec![];
    for (i, pos) in positions.iter().enumerate() {
        for p in i..positions.len() {
            if p != i {
                pairs.push(Pair {
                    start: pos.clone(),
                    end: positions[p],
                });
            }
        }
    }
    pairs
}

fn diff(a: &Pos, b: Pos) -> i64 {
    let x_diff = (a.x as isize - b.x as isize).abs() as i64;
    let y_diff = (a.y as isize - b.y as isize).abs() as i64;

    x_diff + y_diff
}

fn part_1(input: &str, times: i64) -> i64 {
    let galaxies = expand(&parse(input), times);
    let pairs = get_pairs(&galaxies);
    let sum = pairs.iter().map(|p| diff(&p.start, p.end)).sum::<i64>();
    sum
}
fn part_2(input: &str, times: i64) -> i64 {
    let galaxies = expand(&parse(input), times);
    let pairs = get_pairs(&galaxies);
    let sum = pairs.iter().map(|p| diff(&p.start, p.end)).sum::<i64>();
    sum
}

fn main() {
    let example_1 = part_1(include_str!("example.txt"), 1);
    println!("example 1 = {}", example_1);
    assert_eq!(example_1, 374);

    let part_1 = part_1(include_str!("input.txt"), 1);
    println!("part_1 = {}", part_1);

    let example_2 = part_2(include_str!("example.txt"), 9);
    println!("example 2 = {}", example_2);
    assert_eq!(example_2, 1030);

    let example_3 = part_2(include_str!("example.txt"), 99);
    println!("example 3 = {}", example_3);
    assert_eq!(example_3, 8410);

    let part_2 = part_2(include_str!("input.txt"), 999999);
    println!("part_2 = {}", part_2);
}
