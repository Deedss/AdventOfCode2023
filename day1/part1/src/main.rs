use std::fs::{self};

fn main() {
    let contents = get_file_contents();

    let number: i32 = get_numbers(&contents).iter().sum();

    println!("{}", number);
}

fn get_file_contents() -> String {
    let args: Vec<String> = std::env::args().collect();

    fs::read_to_string(&args[1]).unwrap_or_else(|err| {
        eprintln!("Problem parsing file: {err}");
        std::process::exit(1);
    })
}

fn get_numbers(contents: &str) -> Vec<i32> {
    let mut numbers = Vec::new();

    for line in contents.lines() {
        let mut chars: Vec<String> = Vec::new();
        for n in line.chars() {
            if n.is_numeric() {
                chars.push(n.to_string());
            }
        }

        if chars.len() == 1 {
            chars.push(chars[0].clone());
        }

        let result = chars.first().unwrap().to_owned() + chars.last().unwrap();

        numbers.push(result.parse::<i32>().unwrap());
    }

    numbers
}
