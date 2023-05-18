use std::fs::read_to_string;
use std::path::Path;
use std::println;

fn search_for_marker(input: &str, marker_size: usize) -> Option<usize> {
    let bytes = input.as_bytes();

    for i in 0..bytes.len() - marker_size {
        let slice = &bytes[i..i + marker_size];

        //The characters are a..z, for a total of 26 characters.
        //That means we can assign each characters to a bit in a u32
        //We can the do an or for between all the characters and the number of ones will be the
        //number of unique characters
        let ones = slice.iter().map(|byte| (1 as u32) << (byte - 'a' as u8));
        let number_of_ones: u32 = ones.fold(0, |acc: u32, byte: u32| acc | byte).count_ones();

        if number_of_ones == marker_size as u32 {
            return Some(i + marker_size);
        }
    }

    None
}

fn get_answer_a(input: &str) -> Option<usize> {
    const START_OF_PACKET_SIZE: usize = 4;
    search_for_marker(input, START_OF_PACKET_SIZE)
}

fn get_answer_b(input: &String) -> Option<usize> {
    const START_OF_MESAGE_SIZE: usize = 14;
    search_for_marker(input, START_OF_MESAGE_SIZE)
}

fn main() {
    let input_path = Path::new("./input/input.txt");
    let input = read_to_string(input_path).expect("Expecting valid input file");

    println!("Part 1: {}", get_answer_a(&input).unwrap());
    println!("Part 2: {}", get_answer_b(&input).unwrap());
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_answer_a() {
        assert_eq!(
            get_answer_a(&("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string())),
            Some(5)
        );

        assert_eq!(
            get_answer_a(&("nppdvjthqldpwncqszvftbrmjlhg".to_string())),
            Some(6)
        );
        assert_eq!(
            get_answer_a(&("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string())),
            Some(10)
        );
        assert_eq!(
            get_answer_a(&("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string())),
            Some(11)
        );
    }

    #[test]
    fn test_get_answer_b() {
        assert_eq!(
            get_answer_b(&("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string())),
            Some(19)
        );

        assert_eq!(
            get_answer_b(&("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string())),
            Some(23)
        );
        assert_eq!(
            get_answer_b(&("nppdvjthqldpwncqszvftbrmjlhg".to_string())),
            Some(23)
        );
        assert_eq!(
            get_answer_b(&("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string())),
            Some(29)
        );
        assert_eq!(
            get_answer_b(&("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string())),
            Some(26)
        );
    }
}
