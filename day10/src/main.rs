use core::panic;
use std::{char, collections::VecDeque, vec};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Stop,
}

#[derive(Debug)]
struct Neighbours {
    up: char,
    down: char,
    left: char,
    right: char,
}

#[derive(Debug, Clone)]
struct Node {
    x: usize,
    y: usize,
}

fn get_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_direction(character: char, prev_dir: Direction) -> Direction {
    match character {
        '|' => match prev_dir {
            Direction::Down => Direction::Down,
            Direction::Up => Direction::Up,
            _ => panic!("Invalid direction"),
        },
        '-' => match prev_dir {
            Direction::Left => Direction::Left,
            Direction::Right => Direction::Right,
            _ => panic!("Invalid direction"),
        },
        'L' => match prev_dir {
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
            _ => panic!("Invalid direction"),
        },
        'J' => match prev_dir {
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Up,
            _ => panic!("Invalid direction"),
        },
        '7' => match prev_dir {
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Left,
            _ => panic!("Invalid direction"),
        },
        'F' => match prev_dir {
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Right,
            _ => panic!("Invalid direction"),
        },
        _ => Direction::Stop,
    }
}

fn get_start_position(grid: &Vec<Vec<char>>) -> Node {
    for (x, row) in grid.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            if cell == &'S' {
                return Node { x, y };
            }
        }
    }
    Node { x: 0, y: 0 }
}

fn valid(character: char, direction: Direction) -> bool {
    match direction {
        Direction::Up => match character {
            '|' | '7' | 'F' => true,
            _ => false,
        },
        Direction::Down => match character {
            '|' | 'L' | 'J' => true,
            _ => false,
        },
        Direction::Left => match character {
            '-' | 'F' | 'L' => true,
            _ => false,
        },
        Direction::Right => match character {
            '-' | 'J' | '7' => true,
            _ => false,
        },
        Direction::Stop => false,
    }
}

fn get_neighbors(grid: &Vec<Vec<char>>, pos: &Node) -> Neighbours {
    let up = match pos.x.checked_sub(1) {
        Some(x) => grid[x][pos.y],
        None => '.',
    };
    let down = match pos.x.checked_add(1) {
        Some(x) => grid[x][pos.y],
        None => '.',
    };
    let left = match pos.y.checked_sub(1) {
        Some(y) => grid[pos.x][y],
        None => '.',
    };
    let right = match pos.y.checked_add(1) {
        Some(y) => grid[pos.x][y],
        None => '.',
    };

    Neighbours {
        up,
        down,
        left,
        right,
    }
}

fn get_char(pos: &Node, grid: &Vec<Vec<char>>) -> char {
    grid.get(pos.x).unwrap().get(pos.y).unwrap().clone()
}

fn steps_for_loop(grid: &Vec<Vec<char>>) -> (i64, Vec<Vec<char>>) {
    let mut current = get_start_position(&grid);
    let mut steps = 0;
    let mut pipes = vec![vec!['.'; grid[0].len()]; grid.len()];
    let neighbors = get_neighbors(grid, &current);

    let mut direction = Direction::Stop;
    if valid(neighbors.up, Direction::Up) {
        direction = Direction::Up;
    } else if valid(neighbors.down, Direction::Down) {
        direction = Direction::Down;
    } else if valid(neighbors.left, Direction::Left) {
        direction = Direction::Left;
    } else if valid(neighbors.right, Direction::Right) {
        direction = Direction::Right;
    } else {
        panic!("Invalid initial direction");
    }

    loop {
        pipes[current.x][current.y] = get_char(&current, grid);
        match direction {
            Direction::Up => {
                current = Node {
                    x: current.x - 1,
                    y: current.y,
                };
            }
            Direction::Down => {
                current = Node {
                    x: current.x + 1,
                    y: current.y,
                };
            }
            Direction::Left => {
                current = Node {
                    x: current.x,
                    y: current.y - 1,
                };
            }
            Direction::Right => {
                current = Node {
                    x: current.x,
                    y: current.y + 1,
                };
            }
            Direction::Stop => break,
        }
        direction = get_direction(get_char(&current, grid), direction);
        steps += 1;
    }

    (steps / 2, pipes)
}

fn part_1(input: &str) -> i64 {
    let grid = get_grid(input);
    let longest_route = steps_for_loop(&grid);
    longest_route.0
}

fn find_enclosed(grid: &Vec<Vec<char>>) -> Vec<char> {
    let mut found = vec![];
    let rows = grid.len();
    let cols = grid[0].len();

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '.' {}
        }
    }

    found
}
fn part_2(input: &str) -> i64 {
    let grid = get_grid(input);
    let result = steps_for_loop(&grid);

    for row in result.1.iter() {
        println!("{:?}", row);
    }

    0
}

fn main() {
    let example1 = part_1(include_str!("part_1/example.txt"));
    println!("Example 1: {}", example1);

    let example2 = part_1(include_str!("part_1/example2.txt"));
    println!("Example 2: {}", example2);

    let part_1 = part_1(include_str!("part_1/input.txt"));
    println!("Part 1: {}", part_1);

    let part_2_example1 = part_2(include_str!("part_2/example1.txt"));
    println!("Example 1 part 2: {}", part_2_example1);

    // let part_2_example2 = part_2(include_str!("part_2/example2.txt"));
    // println!("Example 2 part 2: {}", part_2_example2);
}
