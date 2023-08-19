use color_eyre::eyre::Context;

fn read_input(path: &str) -> color_eyre::Result<String> {
    let input = std::fs::read_to_string(path).wrap_err(format!("Reading {path}"))?;
    Ok(input)
}

fn main() {
    color_eyre::install().unwrap();

    let input_path = "input/input.txt";
    let input = read_input(input_path).unwrap();

    println!("First question: {}", 0);
    println!("Second question: {}", 0);
}
