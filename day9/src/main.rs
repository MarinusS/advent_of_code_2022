use itertools::Itertools;
use std::cell::RefCell;
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

    fn translate(&mut self, dir: Direction) {
        match dir {
            Direction::U => self.y += 1,
            Direction::D => self.y -= 1,
            Direction::R => self.x += 1,
            Direction::L => self.x -= 1,
        }
    }

    fn run_planck_physics(head: Self, tail: &mut Self) {
        if Position::are_touching(head, *tail) {
            return;
        }

        //If the head is ever two steps directly up, down, left, or right from the tail,
        //the tail must also move one step in that direction so it remains close enough
        if tail.x == head.x {
            if tail.y < head.y - 1 {
                tail.y = head.y - 1;
            } else if tail.y > head.y + 1 {
                tail.y = head.y + 1;
            }
        } else if tail.y == head.y {
            if tail.x < head.x - 1 {
                tail.x = head.x - 1;
            } else if tail.x > head.x + 1 {
                tail.x = head.x + 1;
            }
        }
        //Otherwise, if the head and tail aren't touching and aren't in the same row or column,
        //the tail always moves one step diagonally to keep up:
        else {
            match tail.y.cmp(&head.y) {
                std::cmp::Ordering::Less => tail.y += 1,
                std::cmp::Ordering::Greater => tail.y -= 1,
                std::cmp::Ordering::Equal => {}
            }
            match tail.x.cmp(&head.x) {
                std::cmp::Ordering::Less => tail.x += 1,
                std::cmp::Ordering::Greater => tail.x -= 1,
                std::cmp::Ordering::Equal => {}
            }
        }
    }
}

struct Rope {
    head: Position,
    tail: Position,
}

impl Rope {
    fn new(pos: Position) -> Self {
        Rope {
            head: pos,
            tail: pos,
        }
    }

    fn move_head(&mut self, dir: Direction) {
        self.head.translate(dir);
        Position::run_planck_physics(self.head, &mut self.tail);
    }

    #[allow(dead_code)]
    pub fn print_on_grid(&self, n_columns: u32, n_rows: u32) {
        let mut lines: Vec<Vec<char>> = (0..n_rows)
            .map(|_| (0..n_columns).map(|_| '.').collect())
            .collect();

        let tail = self.tail;
        let head = self.head;
        lines[n_rows as usize - 1 - tail.y as usize][tail.x as usize] = 'T';
        lines[n_rows as usize - 1 - head.y as usize][head.x as usize] = 'H';
        if tail == head {
            lines[n_rows as usize - 1 - tail.y as usize][tail.x as usize] = 'B';
        }
        let grid = (lines
            .iter()
            .map(|line| line.iter().collect())
            .collect::<Vec<String>>())
        .join("\n");

        println!("{grid}")
    }
}

struct LongRope {
    knots: Vec<RefCell<Position>>,
}

impl LongRope {
    fn new(pos: Position, len: u32) -> Self {
        LongRope {
            knots: (0..len).map(|_| RefCell::new(pos)).collect(),
        }
    }

    fn move_head(&mut self, dir: Direction) {
        self.knots[0].borrow_mut().translate(dir);

        for (head, tail) in self.knots.iter().tuple_windows() {
            Position::run_planck_physics(*head.borrow(), &mut tail.borrow_mut())
        }
    }

    #[allow(dead_code)]
    fn print_on_grid(&self, n_columns: u32, n_rows: u32) {
        let mut lines: Vec<Vec<char>> = (0..n_rows)
            .map(|_| (0..n_columns).map(|_| '.').collect())
            .collect();

        for (i, knot) in self.knots.iter().enumerate().rev() {
            lines[n_rows as usize - 1 - knot.borrow().y as usize][knot.borrow().x as usize] =
                if i == 0 {
                    'H'
                } else {
                    char::from_digit(i as u32, 10).unwrap()
                };
        }

        let grid = (lines
            .iter()
            .map(|line| line.iter().collect())
            .collect::<Vec<String>>())
        .join("\n");

        println!("{grid}")
    }
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
    let mut rope = Rope::new(Position { x: 0, y: 0 });
    let actions = parse_moves(input);
    let mut visited_by_tail = HashSet::new();

    for action in actions {
        //println!("Action: {:?}", action);
        for _ in 0..action.units {
            rope.move_head(action.dir);
            visited_by_tail.insert(rope.tail);
            //rope.print_on_grid(6, 5);
            //println!()
        }
    }

    visited_by_tail.len() as i32
}

fn get_answer_b(input: &str) -> i32 {
    let mut long_rope = LongRope::new(Position { x: 11, y: 5 }, 10);
    let actions = parse_moves(input);
    let mut visited_by_tail = HashSet::new();

    for action in actions {
        //println!("Action: {:?}", action);
        for _ in 0..action.units {
            long_rope.move_head(action.dir);
            visited_by_tail.insert(*long_rope.knots.last().unwrap().borrow());
        }
        //long_rope.print_on_grid(30, 30);
        //println!()
    }

    visited_by_tail.len() as i32
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

    #[test]
    fn test_get_answer_b() {
        let test_input: &str = "R 5\n\
                                U 8\n\
                                L 8\n\
                                D 3\n\
                                R 17\n\
                                D 10\n\
                                L 25\n\
                                U 20\n";
        //let test_input: &str = "R 4\n\
        //                        U 4\n\
        //                        L 3\n\
        //                        D 1\n\
        //                        R 4\n\
        //                        D 1\n\
        //                        L 5\n\
        //                        R 2\n";
        assert_eq!(get_answer_b(test_input), 36);
    }
}
