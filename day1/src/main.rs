use color_eyre::eyre::Context;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::println;

fn read_input(path: &str) -> color_eyre::Result<String> {
    let input = std::fs::read_to_string(path).wrap_err(format!("Reading {path}"))?;
    Ok(input)
}

fn get_elf_calories(input: &String) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|food_collection| {
            food_collection
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum()
        })
        .collect()
}

/// Finds the Elf carrying the most Calories and returns the total Calories that Elf carrying
fn get_most_calories(input: &String) -> u32 {
    *get_elf_calories(input).iter().max().unwrap()
}

/// Finds the top n Elfs carrying the most Calories and returns the total Calories that these Elfs are carrying
fn get_top_n_calories(input: &String, n: usize) -> Option<u32> {
    let elf_calories = get_elf_calories(input);

    if elf_calories.len() < n {
        return None;
    };

    let mut heap = BinaryHeap::new();
    let mut cal_it = elf_calories.iter();

    for _ in 0..n {
        heap.push(Reverse(*cal_it.next().unwrap()));
    }

    for calories in cal_it {
        heap.push(Reverse(*calories));
        heap.pop();
    }

    Some(heap.iter().map(|x| x.0).sum())
}

fn main() {
    color_eyre::install().unwrap();

    let input_path = "input/input.txt";
    let input = read_input(input_path).unwrap();

    println!("First question: {}", get_most_calories(&input));
    println!(
        "Second question: {}",
        get_top_n_calories(&input, 3).unwrap()
    );
}
