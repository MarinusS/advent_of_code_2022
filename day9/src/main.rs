use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Direction {
    U,
    D,
    L,
    R,
}

#[derive(Debug)]
struct Action {
    dir: Direction,
    units: i32,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn are_touching(p1: Position, p2: Position) -> bool {
        (p1.x - p2.x).abs() <= 1 && (p1.y - p2.y).abs() <= 1
    }
}

struct Rope {
    head: Position,
    tail: Position,
}

impl Rope {
    fn new() -> Self {
        let pos = Position { x: 0, y: 0 };

        Rope {
            head: pos,
            tail: pos,
        }
    }

    fn ends_touch(&self) -> bool {
        Position::are_touching(self.head, self.tail)
    }

    fn move_head(&mut self, dir: Direction) {
        match dir {
            Direction::U => {
                self.head.y += 1;
                if !self.ends_touch() {
                    self.tail = Position {
                        x: self.head.x,
                        y: self.head.y - 1,
                    }
                }
            }

            Direction::D => {
                self.head.y -= 1;
                if !self.ends_touch() {
                    self.tail = Position {
                        x: self.head.x,
                        y: self.head.y + 1,
                    }
                }
            }

            Direction::R => {
                self.head.x += 1;
                if !self.ends_touch() {
                    self.tail = Position {
                        x: self.head.x - 1,
                        y: self.head.y,
                    }
                }
            }

            Direction::L => {
                self.head.x -= 1;
                if !self.ends_touch() {
                    self.tail = Position {
                        x: self.head.x + 1,
                        y: self.head.y,
                    }
                }
            }
        }
    }
}

fn print_rope(rope: &Rope, n_columns: u32, n_rows: u32) {
    let mut grid = String::new();
    for y in 0..n_rows {
        let mut row = String::new();
        for x in 0..n_columns {
            let cur_pos = Position {
                x: x.try_into().unwrap(),
                y: <u32 as TryInto<i32>>::try_into(n_columns).unwrap()
                    - 1
                    - <u32 as TryInto<i32>>::try_into(y).unwrap(),
            };
            if cur_pos == rope.head {
                row += "H";
            } else if cur_pos == rope.tail {
                row += "T"
            } else {
                row += ".";
            }
        }
        grid += &(row + "\n");
    }

    println!("{grid}");
}

fn parse_line(line: &str) -> Action {
    let (dir, norm) = line
        .split_once(' ')
        .expect("Expected direction and norm from the input to be seperated by a space");
    let norm: i32 = norm
        .parse()
        .expect("Expected to be able to parse norm of the move into i32");

    match dir {
        "U" => Action {
            dir: Direction::U,
            units: norm,
        },
        "D" => Action {
            dir: Direction::D,
            units: norm,
        },
        "L" => Action {
            dir: Direction::L,
            units: norm,
        },
        "R" => Action {
            dir: Direction::R,
            units: norm,
        },
        _ => panic!("Unkonwn direction encounted when parsing input"),
    }
}

fn parse_moves(input: &str) -> Vec<Action> {
    input.lines().map(parse_line).collect()
}

fn get_answer_a(input: &str) -> i32 {
    let mut rope = Rope::new();
    let actions = parse_moves(input);
    let mut visited_by_tail = HashSet::new();

    for action in actions {
        //println!("Action: {:?}", action);
        for _ in 0..action.units {
            rope.move_head(action.dir);
            visited_by_tail.insert(rope.tail);
            //println!("Tail pos: {:?}", rope.tail);
            //print_rope(&rope, 6, 6);
        }
    }

    visited_by_tail.len() as i32
}

fn get_answer_b(input: &str) -> i32 {
    todo!()
}

fn main() {
    let input_path = "input/input.txt";
    let input = std::fs::read_to_string(input_path).unwrap();

    println!("Answer A: {}", get_answer_a(&input));
    println!("Answer B: {}", get_answer_b(&input));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_answer_a() {
        let test_input: &str = "R 4\n\
                                U 4\n\
                                L 3\n\
                                D 1\n\
                                R 4\n\
                                D 1\n\
                                L 5\n\
                                R 2\n";

        assert_eq!(get_answer_a(test_input), 13);
    }
}
