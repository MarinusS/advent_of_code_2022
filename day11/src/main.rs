use std::{cell::RefCell, str::FromStr};

#[derive(Debug, PartialEq)]
struct MonkeyTest {
    divider: i32,
    target_if_false: i32,
    target_if_true: i32,
}

impl MonkeyTest {
    fn evaluate(&self, worry_level: i32) -> (bool, i32) {
        if worry_level % self.divider == 0 {
            (true, self.target_if_true)
        } else {
            (false, self.target_if_false)
        }
    }

    fn from_str(s: [&str; 3]) -> Result<MonkeyTest, ParsingError> {
        let divider = s[0]
            .trim()
            .strip_prefix("Test: divisible by ")
            .ok_or(ParsingError::Test(
                "Expected monkey test line to start with: 'Test: divisible by ', got: ".to_string()
                    + s[0],
            ))?
            .trim()
            .parse()
            .map_err(|_| {
                ParsingError::Test(
                    "Expected monley test line to have a divider parsable as i32".to_string(),
                )
            })?;

        let target_if_true = s[1]
            .trim()
            .strip_prefix("If true: throw to monkey ")
            .ok_or(ParsingError::Test(
                "Expected monkey test second line to start with: 'If true: throw to monkey '"
                    .to_string(),
            ))?
            .trim()
            .parse()
            .map_err(|_| {
                ParsingError::Test(
                    "Expected monley test line to have a divider parsable as i32".to_string(),
                )
            })?;

        let target_if_false = s[2]
            .trim()
            .strip_prefix("If false: throw to monkey ")
            .ok_or(ParsingError::Test(
                "Expected monkey test third line to start with: 'If false: throw to monkey '"
                    .to_string(),
            ))?
            .trim()
            .parse()
            .map_err(|_| {
                ParsingError::Test(
                    "Expected monley test line to have a divider parsable as i32".to_string(),
                )
            })?;

        Ok(MonkeyTest {
            divider,
            target_if_true,
            target_if_false,
        })
    }
}

#[derive(Debug, PartialEq)]
enum Operator {
    Multiply,
    Add,
}

impl Operator {
    fn from_str(s: &str) -> Result<Operator, ParsingError> {
        match s {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Multiply),
            str => Err(ParsingError::Operation(
                "Expected known operator, got : ".to_string() + str,
            )),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Var {
    Old,
    Const(i32),
}

impl Var {
    fn from_str(s: &str) -> Result<Var, ParsingError> {
        match s {
            "old" => Ok(Var::Old),
            str => Ok(Var::Const(str.parse::<i32>().map_err(|_| {
                ParsingError::Operation("Expected variable to be parsable as i32".to_string())
            })?)),
        }
    }

    fn evaluate(&self, old: i32) -> i32 {
        match self {
            Var::Const(x) => *x,
            Var::Old => old,
        }
    }
}

#[derive(Debug, PartialEq)]
struct MonkeyOperation {
    var1: Var,
    var2: Var,
    operator: Operator,
}

impl MonkeyOperation {
    fn evaluate(&self, old: i32) -> i32 {
        match self.operator {
            Operator::Add => self.var1.evaluate(old) + self.var2.evaluate(old),
            Operator::Multiply => self.var1.evaluate(old) * self.var2.evaluate(old),
        }
    }

    fn from_str(s: &str) -> Result<MonkeyOperation, ParsingError> {
        let tokens = s
            .trim()
            .strip_prefix("Operation: new =")
            .ok_or(ParsingError::StartingItems(
                "Expected operation line to start with 'Operation: new='".to_string(),
            ))?
            .split_whitespace()
            .collect::<Vec<_>>();

        if tokens.len() != 3 {
            return Err(ParsingError::Operation(
                "Expected monkey opertaion to be two variable and one operator".to_string(),
            ));
        }

        Ok(MonkeyOperation {
            var1: Var::from_str(tokens[0])?,
            operator: Operator::from_str(tokens[1])?,
            var2: Var::from_str(tokens[2])?,
        })
    }
}

#[derive(Debug, PartialEq)]
enum ParsingError {
    ID(String),
    StartingItems(String),
    Operation(String),
    Test(String),
}

#[derive(Debug, PartialEq)]
struct Monkey {
    id: i32,
    items: Vec<i32>,
    operation: MonkeyOperation,
    test: MonkeyTest,
    n_items_inpsected: u32,
}

impl Monkey {
    fn new(
        id: i32,
        starting_items: Vec<i32>,
        operation: MonkeyOperation,
        test: MonkeyTest,
    ) -> Monkey {
        Monkey {
            id,
            items: starting_items,
            operation,
            test,
            n_items_inpsected: 0,
        }
    }

    fn parse_id_line(s: &str) -> Result<i32, ParsingError> {
        let mut fields = s
            .strip_suffix(':')
            .ok_or(ParsingError::ID(
                "Expected Monkey ID line to end with :".to_string(),
            ))?
            .split_whitespace();

        if fields.next() != Some("Monkey") {
            return Err(ParsingError::ID(
                "Expected Monkey ID line to start with Monkey".to_string(),
            ));
        }
        fields
            .next()
            .ok_or(ParsingError::ID(
                "Expected an ID number field as the second word".to_string(),
            ))?
            .parse::<i32>()
            .map_err(|_| ParsingError::ID("Expected a valid ID as second field".to_string()))
    }

    fn parse_starting_items_line(s: &str) -> Result<Vec<i32>, ParsingError> {
        s.trim()
            .strip_prefix("Starting items:")
            .ok_or(ParsingError::StartingItems(
                "Expected starting items line to start with 'Starting items:'".to_string(),
            ))?
            .split(',')
            .map(|field| {
                field.trim().parse::<i32>().map_err(|_| {
                    ParsingError::StartingItems(
                        "Expected a valid item in starting items field".to_string(),
                    )
                })
            })
            .collect::<Result<Vec<_>, _>>()
    }

    fn parse_operation_line(s: &str) -> Result<MonkeyOperation, ParsingError> {
        MonkeyOperation::from_str(s)
    }

    fn parse_test_lines(s: [&str; 3]) -> Result<MonkeyTest, ParsingError> {
        MonkeyTest::from_str(s)
    }
}

impl FromStr for Monkey {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        //Parse ID
        let id =
            Monkey::parse_id_line(lines.next().expect("Expected a first line with Monkey id"))?;
        let starting_items = Monkey::parse_starting_items_line(
            lines
                .next()
                .expect("Expected a second line with Monkey starting items"),
        )?;

        let operation = Monkey::parse_operation_line(
            lines
                .next()
                .expect("Expected a third line with Monkey operation"),
        )?;

        let test = Monkey::parse_test_lines([
            lines
                .next()
                .expect("Expected a fourth line with Monkey operation"),
            lines
                .next()
                .expect("Expected a fifth line with Monkey operation"),
            lines
                .next()
                .expect("Expected a sixth line with Monkey operation"),
        ])?;

        Ok(Monkey::new(id, starting_items, operation, test))
    }

    type Err = ParsingError;
}

struct MonkeyGroup {
    monkeys: Vec<RefCell<Monkey>>,
}

impl MonkeyGroup {
    fn run_round(&self, verbosity: i32, boredom: bool) {
        for monkey in &self.monkeys {
            let mut monkey = monkey.borrow_mut();
            if verbosity >= 2 {
                println!("Monkey {}:", monkey.id);
            }
            for item in &monkey.items {
                let new_worry_level = monkey.operation.evaluate(*item);
                let mut bored_worry_level = new_worry_level;
                if boredom {
                    bored_worry_level = new_worry_level / 3;
                }
                let (test_result, target) = monkey.test.evaluate(bored_worry_level);
                self.monkeys[target as usize]
                    .borrow_mut()
                    .items
                    .push(bored_worry_level);
                if verbosity >= 2 {
                    println!("  Monkey inspects an item with a worry level of {item}");
                    println!(
                        "    New worry level is {:?} {:?} {:?} = {new_worry_level}",
                        monkey.operation.var1, monkey.operation.operator, monkey.operation.var2
                    );
                    if boredom {
                        println!("    Monkey gets bored with the item. Worry level is divided by 3 to {bored_worry_level}");
                    }
                    println!("    Worry level {bored_worry_level} is divisible by {}: {test_result}, throwing item to monkey {target}", monkey.test.divider);
                }
            }

            monkey.n_items_inpsected += monkey.items.len() as u32;
            monkey.items.clear();
        }

        if verbosity >= 1 {
            println!("After the round, the monkeys are holding items with these worry levels:");
            for monkey in &self.monkeys {
                let monkey = monkey.borrow();
                println!(
                    "Monkey {}: {}",
                    monkey.id,
                    monkey
                        .items
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
        }
    }

    fn get_most_active(&self, n: usize) -> Vec<(i32, u32)> {
        let mut partial_copy: Vec<_> = self
            .monkeys
            .iter()
            .map(|m| (m.borrow().id, m.borrow().n_items_inpsected))
            .collect();
        partial_copy.sort_by(|a, b| b.1.cmp(&a.1));
        partial_copy[..std::cmp::min(partial_copy.len(), n)].to_vec()
    }
}

fn main() {
    //let test_input: &str = include_str!("test_input.txt");
    let input_path = "input/input.txt";
    let input = std::fs::read_to_string(input_path).unwrap();
    let monkeys: Vec<_> = input
        .split("\n\n")
        .map(|s| Monkey::from_str(s).unwrap())
        .map(RefCell::new)
        .collect();

    let group = MonkeyGroup { monkeys };
    for round in 1..=20 {
        println!("Round {round}:");
        group.run_round(2, true);
        println!()
    }

    let most_active = group.get_most_active(10);
    let leaderboard = most_active
        .iter()
        .map(|(id, n_traded)| format!("Monkey {id} inspected items {n_traded} times."))
        .collect::<Vec<_>>()
        .join("\n");
    println!("Activity leaderboard:\n{}", leaderboard);
    println!(
        "Answer to question A: {}",
        most_active[0].1 * most_active[1].1
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_monkey_id_line() {
        let tests: Vec<_> = vec![
            ("Monkey 0:", 0),
            ("Monkey 1:", 1),
            ("Monkey 10:", 10),
            ("Monkey 24:", 24),
        ];
        for test in tests {
            assert_eq!(Monkey::parse_id_line(test.0), Ok(test.1));
        }
    }

    #[test]
    fn test_parse_monkey_starting_items_line() {
        let tests: Vec<_> = vec![
            ("Starting items: 79, 98", vec![79, 98]),
            ("Starting items: 54, 65, 75, 74", vec![54, 65, 75, 74]),
            ("Starting items: 74", vec![74]),
        ];
        for test in tests {
            assert_eq!(Monkey::parse_starting_items_line(test.0), Ok(test.1));
        }
    }

    #[test]
    fn test_parse_monkey_operation_line() {
        let tests: Vec<_> = vec![
            (
                "Operation: new = old * 19",
                MonkeyOperation {
                    var1: Var::Old,
                    var2: Var::Const(19),
                    operator: Operator::Multiply,
                },
            ),
            (
                "Operation: new = 20 + old",
                MonkeyOperation {
                    var1: Var::Const(20),
                    var2: Var::Old,
                    operator: Operator::Add,
                },
            ),
            (
                "Operation: new = old * old",
                MonkeyOperation {
                    var1: Var::Old,
                    var2: Var::Old,
                    operator: Operator::Multiply,
                },
            ),
            (
                "Operation: new = old + old",
                MonkeyOperation {
                    var1: Var::Old,
                    var2: Var::Old,
                    operator: Operator::Add,
                },
            ),
        ];

        for test in tests {
            assert_eq!(Monkey::parse_operation_line(test.0), Ok(test.1));
        }
    }

    #[test]
    fn test_parse_monkey_test_lines() {
        let tests: Vec<_> = vec![
            (
                [
                    "Test: divisible by 23",
                    "  If true: throw to monkey 2",
                    "  If false: throw to monkey 3",
                ],
                MonkeyTest {
                    divider: 23,
                    target_if_true: 2,
                    target_if_false: 3,
                },
            ),
            (
                [
                    "Test: divisible by 3",
                    "  If true: throw to monkey 7",
                    "  If false: throw to monkey 2",
                ],
                MonkeyTest {
                    divider: 3,
                    target_if_true: 7,
                    target_if_false: 2,
                },
            ),
        ];

        for test in tests {
            assert_eq!(Monkey::parse_test_lines(test.0), Ok(test.1));
        }
    }

    #[test]
    fn test_parse_monkey() {
        let tests: Vec<_> = vec![(
            "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 3
    If false: throw to monkey 5",
            Monkey {
                id: 0,
                operation: MonkeyOperation {
                    var1: Var::Old,
                    var2: Var::Const(19),
                    operator: Operator::Multiply,
                },
                test: MonkeyTest {
                    divider: 23,
                    target_if_false: 5,
                    target_if_true: 3,
                },
                items: vec![79, 98],
                n_items_inpsected: 0,
            },
        )];

        for test in tests {
            assert_eq!(Monkey::from_str(test.0), Ok(test.1));
        }
    }
}
