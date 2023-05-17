use std::fs::read_to_string;
use std::ops::RangeInclusive;
use std::path::Path;
use std::println;

trait RangesExtensions {
    fn contains_range(&self, other: &Self) -> bool;

    fn overlaps_range(&self, other: &Self) -> bool;
}

impl<T> RangesExtensions for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn overlaps_range(&self, other: &Self) -> bool {
        self.contains(other.start())
            || self.contains(other.end())
            || other.contains(self.start())
            || other.contains(self.end())
    }
}

fn parse_range(range: &str) -> RangeInclusive<i32> {
    let end_points: Vec<_> = range.split("-").collect();
    assert!(end_points.len() == 2, "Expecting valid range format");

    let mut end_points_it = end_points.iter().map(|end_point| {
        end_point
            .parse::<i32>()
            .expect("Expecting valid range end points")
    });

    end_points_it.next().unwrap()..=end_points_it.next().unwrap()
}

fn parse_line(line: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let line_split: Vec<_> = line.split(",").collect();
    assert!(line_split.len() == 2, "Expeting valid line format");

    let mut ranges_it = line_split.iter().map(|range| parse_range(range));
    (ranges_it.next().unwrap(), ranges_it.next().unwrap())
}

fn get_answer_a(input: &String) -> i32 {
    input
        .lines()
        .map(|line| parse_line(line))
        .map(|ranges| {
            (ranges.0.contains_range(&ranges.1) || ranges.1.contains_range(&ranges.0)) as i32
        })
        .sum()
}

fn get_answer_b(input: &String) -> i32 {
    input
        .lines()
        .map(|line| parse_line(line))
        .map(|ranges| ranges.0.overlaps_range(&ranges.1) as i32)
        .sum()
}

fn main() {
    let input_path = Path::new("./input/input.txt");
    let input = read_to_string(input_path).expect("Expecting valid input file");

    println!("Part 1: {}", get_answer_a(&input));
    println!("Part 2: {}", get_answer_b(&input));
}
