use std::fs::read_to_string;
use std::path::Path;
use std::println;

fn get_item_value(item: char) -> i32 {
    let value: u8;

    if item as u8 >= b'a' {
        value = (item as u8) - ('a' as u8) + 1;
    } else {
        value = (item as u8) - ('A' as u8) + 27;
    }

    return value as i32;
}

fn get_answer_a(input: &String) -> i32 {
    let duplicate_items: Vec<_> = input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(compartment_1, compartment_2)| {
            compartment_1
                .chars()
                .find(|item| compartment_2.contains(*item))
                .unwrap()
        })
        .collect();

    return duplicate_items
        .iter()
        .map(|item| get_item_value(*item))
        .sum();
}

fn get_answer_b(input: &String) -> i32 {
    let lines: Vec<_> = input.lines().collect::<Vec<_>>();

    lines
        .chunks(3)
        .map(|chunk| {
            assert!(chunk.len() == 3, "Always expecting chunks of size 3");
            chunk[0]
                .chars()
                .find(|item| chunk[1].contains(*item) && chunk[2].contains(*item))
                .expect("Always expecting a common item in chunks")
        })
        .map(|item| get_item_value(item))
        .sum()
}

fn main() {
    let input_path = Path::new("./input/input.txt");
    let input = read_to_string(input_path).expect("Expecting valid input file");

    println!("Phase 1: {}", get_answer_a(&input));
    println!("Phase 2: {}", get_answer_b(&input));
}
