use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;

fn main() -> Result<()> {
    let lines = lines_from_file("guesses.txt");
    let mut guesses: HashMap<String, i8> = HashMap::new();

    for line in lines {
        if let Ok(line) = line {
            let numbers = get_sorted_numbers_from_line(line);
            let key = get_key(numbers);
            guesses
                .entry(key)
                .and_modify(|guess| *guess += 1)
                .or_insert(1);
        }
    }

    for (key, value) in guesses {
        println!("{} {}", key, value);
    }

    Ok(())
}

fn lines_from_file(filename: impl AsRef<Path>) -> Lines<BufReader<File>> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    return buf.lines();
}

fn get_sorted_numbers_from_line(line: String) -> Vec<i8> {
    let mut numbers: Vec<i8> = line
        .split_whitespace()
        .map(|x| x.parse::<i8>().unwrap())
        .collect();
    numbers.sort();
    return numbers;
}

fn get_key(numbers: Vec<i8>) -> String {
    return numbers
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("-");
}
