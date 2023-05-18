use std::cell::RefCell;
use std::fs::read_to_string;
use std::path::Path;
use std::{format, println, vec, write};

struct Action {
    crates_to_move: usize,
    from_pile: usize,
    to_pile: usize,
}

type MyCrate = char;
type Pile = Vec<MyCrate>;
struct Piles {
    piles: Vec<RefCell<Pile>>,
}

impl std::fmt::Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(Action): Moving {} crates from pile {} to pile {}.",
            self.crates_to_move, self.from_pile, self.to_pile
        )
    }
}

impl std::fmt::Debug for Piles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for (i, pile) in self.piles.iter().map(|pile| pile.borrow()).enumerate() {
            output += &format!("(Pile: {}): {:?}\n", i, pile);
        }
        write!(f, "{}", output)
    }
}

fn parse_config_line(line: &str, num_piles: usize) -> Vec<MyCrate> {
    let mut output = vec![' '; num_piles];
    let chars: Vec<_> = line.chars().collect();

    for i in 0..num_piles {
        output[i] = chars[i * 4 + 1];
    }

    return output;
}

fn parse_init_piles(init_piles: &str) -> Piles {
    let mut lines = init_piles.lines().rev();
    let first_line = lines
        .next()
        .expect("Initiziale configuration expected at beginning of the file")
        .to_string();

    let (num_piles, check): (usize, usize) =
        ((first_line.len() + 1) / 4, (first_line.len() + 1) % 4);

    assert!(
        check == 0,
        "The initial configuration could not be read properly"
    );

    let mut config: Vec<Vec<MyCrate>> = Vec::new();
    for line in lines {
        config.push(parse_config_line(line, num_piles));
    }

    //init_config is now a vector where each element is a horizontal line!()
    //Change it so that each element is a vertical pile and clean up by removing air spaces
    //Basiccaly a transpose, but we also remove the air spaces
    let mut piles = Vec::new();
    let max_height = config.len();

    for i in 0..num_piles {
        let mut pile: Pile = Vec::new();
        for j in 0..max_height {
            let my_crate = config[j][i];
            if my_crate == ' ' {
                break;
            }
            pile.push(my_crate);
        }

        piles.push(RefCell::new(pile));
    }

    Piles { piles }
}

fn parse_action(line: &str) -> Action {
    let splits: Vec<_> = line.split(" ").collect();
    let mut numbers: Vec<usize> = Vec::new();

    for i in [1, 3, 5] {
        numbers.push(splits[i].parse().expect("Expecting valid move line."));
    }

    Action {
        crates_to_move: numbers[0],
        from_pile: numbers[1] - 1,
        to_pile: numbers[2] - 1,
    }
}

fn parse_input(input: &String) -> (&str, &str) {
    let wrong_frmt_msg = "Expecting input format where the inital configuration and moves are seperated by an empty line";
    let (init_piles, moves) = input.split_once("\n\n").expect(wrong_frmt_msg);
    return (init_piles, moves);
}

fn execute_action_a(piles: &mut Piles, action: &Action) {
    let crate_it = (0..action.crates_to_move).map(|_| {
        piles.piles[action.from_pile]
            .borrow_mut()
            .pop()
            .expect(&format!(
                "Was expeting a crate in pile {}",
                action.from_pile
            ))
    });

    for my_crate in crate_it {
        piles.piles[action.to_pile].borrow_mut().push(my_crate);
    }
}

fn execute_action_b(piles: &mut Piles, action: &Action) {
    let crate_it = (0..action.crates_to_move)
        .map(|_| {
            piles.piles[action.from_pile]
                .borrow_mut()
                .pop()
                .expect(&format!(
                    "Was expeting a crate in pile {}",
                    action.from_pile
                ))
        })
        .collect::<Vec<_>>() //Have to collect to map before the reverse or else does not work
        .into_iter()
        .rev();

    for my_crate in crate_it {
        piles.piles[action.to_pile].borrow_mut().push(my_crate);
    }
}

fn get_top_crates(piles: &Piles) -> String {
    piles
        .piles
        .iter()
        .map(|pile| *pile.borrow().last().unwrap())
        .collect()
}

fn get_answer_a(input: &String) -> String {
    let (init_piles, moves) = parse_input(&input);
    let mut piles = parse_init_piles(init_piles);
    let actions = moves.lines().map(|line| parse_action(line));

    for action in actions {
        execute_action_a(&mut piles, &action);
    }

    get_top_crates(&piles)
}

fn get_answer_b(input: &String) -> String {
    let (init_piles, moves) = parse_input(&input);
    let mut piles = parse_init_piles(init_piles);
    let actions = moves.lines().map(|line| parse_action(line));

    for action in actions {
        execute_action_b(&mut piles, &action);
    }

    get_top_crates(&piles)
}

fn main() {
    let input_path = Path::new("./input/input.txt");
    let input = read_to_string(input_path).expect("Expecting valid input file");

    println!("Part 1: {}", get_answer_a(&input));
    println!("Part 2: {}", get_answer_b(&input));
}
