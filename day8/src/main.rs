use std::cmp::max;

type Grid = Vec<Vec<Tree>>;

#[derive(Debug)]
struct Tree {
    x: usize,
    y: usize,
    height: usize,
}

impl Tree {
    fn is_visible(&self, grid: &Grid) -> bool {
        let n_rows = grid.len();
        let n_columns = grid.first().unwrap().len();

        if self.x == 0 || self.x == n_columns - 1 || self.y == 0 || self.y == n_rows - 1 {
            return true;
        }

        let directions = [
            (0..self.x).rev().map(|x| (x, self.y)).collect::<Vec<_>>(),
            (self.x + 1..n_columns).map(|x| (x, self.y)).collect(),
            (0..self.y).rev().map(|y| (self.x, y)).collect(),
            (self.y + 1..n_rows).map(|y| (self.x, y)).collect(),
        ];

        for (cardinal, others) in directions.iter().enumerate() {
            for other_pos in others.iter() {
                let other_tree = &grid[other_pos.1][other_pos.0];
                if self.height <= other_tree.height {
                    break;
                } else if (cardinal == 0 && other_pos.0 == 0)
                    || (cardinal == 1 && other_pos.0 == n_columns - 1)
                    || (cardinal == 2 && other_pos.1 == 0)
                    || (cardinal == 3 && other_pos.1 == n_rows - 1)
                {
                    return true;
                }
            }
        }
        false
    }

    fn visibility_score(&self, grid: &Grid) -> i32 {
        let n_rows = grid.len();
        let n_columns = grid.first().unwrap().len();

        let directions = [
            (0..self.x).rev().map(|x| (x, self.y)).collect::<Vec<_>>(),
            (self.x + 1..n_columns).map(|x| (x, self.y)).collect(),
            (0..self.y).rev().map(|y| (self.x, y)).collect(),
            (self.y + 1..n_rows).map(|y| (self.x, y)).collect(),
        ];

        let mut view = 1;
        for others in directions.iter() {
            let mut cardinal_view = 0;
            for other_pos in others.iter() {
                let other_tree = &grid[other_pos.1][other_pos.0];
                cardinal_view += 1;
                if self.height <= other_tree.height {
                    break;
                }
            }
            view *= cardinal_view
        }

        view
    }
}

fn build_grid_from_string(input: &str) -> Grid {
    let mut grid: Grid = vec![];
    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (x, char) in line.chars().enumerate() {
            row.push(Tree {
                height: char.to_digit(10).unwrap().try_into().unwrap(),
                x,
                y,
            });
        }
        grid.push(row);
    }
    grid
}

//Could be optimized by dyanic programming, each tree can store the max height seen in any
//direction instread of looking it up every time
fn get_answer_a(input: &str) -> i32 {
    let grid = build_grid_from_string(input);
    let mut visibility_grid = String::new();
    let mut height_grid = String::new();

    let mut counter = 0;
    for row in grid.iter() {
        let mut vis_row = String::new();
        let mut height_row = String::new();
        for tree in row.iter() {
            height_row += &format!("{}", tree.height);
            if tree.is_visible(&grid) {
                vis_row += "O";
                counter += 1;
            } else {
                vis_row += "X";
            }
        }
        visibility_grid += &(vis_row + "\n");
        height_grid += &(height_row + "\n");
    }
    println!("{}", visibility_grid);
    println!();
    println!("{}", height_grid);
    counter
}

fn get_answer_b(input: &str) -> i32 {
    let grid = build_grid_from_string(input);

    let mut max_view = 0;
    for row in grid.iter() {
        for tree in row.iter() {
            max_view = max(max_view, tree.visibility_score(&grid));
        }
    }

    max_view
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
        let test_input: &str = "30373\n\
                                25512\n\
                                65332\n\
                                33549\n\
                                35390\n";

        assert_eq!(get_answer_a(test_input), 21);
    }
}
