use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;
use std::todo;

#[derive(Debug, PartialEq)]
enum Commands {
    ChangeDirectory { path: Option<String> },
    List { list: Vec<ListTypes> },
}

#[derive(Debug, PartialEq)]
enum ListTypes {
    File { name: String, size: u32 },
    Dir { name: String },
}

#[derive(Debug)]
struct ParseListTypeError;
impl std::str::FromStr for ListTypes {
    type Err = ParseListTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_ascii_whitespace();
        match words.next() {
            Some("dir") => Ok(ListTypes::Dir {
                name: words.next().ok_or(ParseListTypeError)?.to_string(),
            }),
            x => Ok(ListTypes::File {
                size: match x.ok_or(ParseListTypeError)?.parse::<u32>() {
                    Ok(size) => Ok(size),
                    _ => Err(ParseListTypeError),
                }?,
                name: words.next().ok_or(ParseListTypeError)?.to_string(),
            }),
        }
    }
}

#[derive(Debug, PartialEq)]
struct ParseCommandError;
impl std::str::FromStr for Commands {
    type Err = ParseCommandError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().peekable();
        let first_line = lines.next().ok_or(ParseCommandError)?;
        let mut words = first_line.split_ascii_whitespace();
        if words.next() != Some("$") {
            return Err(ParseCommandError);
        }
        match words.next() {
            Some("cd") => Ok(Commands::ChangeDirectory {
                path: words.next().map(|path| path.to_string()),
            }),
            Some("ls") => Ok(Commands::List {
                list: lines
                    .map(|line| ListTypes::from_str(line).unwrap())
                    .collect(),
            }),
            _ => Err(ParseCommandError),
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<Commands>, ParseCommandError> {
    let mut commands = Vec::new();

    let mut lines = input.lines().peekable();
    let mut curr_command = String::new();
    while let Some(line) = lines.next() {
        curr_command.clear();
        curr_command.push_str(line);
        loop {
            let next_line = lines.peek();
            if next_line.is_none() || next_line.unwrap().starts_with('$') {
                break;
            }
            curr_command.push('\n');
            let line = lines.next().unwrap();
            curr_command.push_str(line);
        }
        commands.push(Commands::from_str(&curr_command)?)
    }

    Ok(commands)
}

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    size: u32,
    childs_idx: HashMap<&'a str, usize>,
    parent_idx: Option<usize>,
}

impl<'a> Node<'a> {
    fn new(name: &'a str, size: u32) -> Self {
        let childs_idx = HashMap::new();
        let parent_idx = None;

        Self {
            name,
            size,
            childs_idx,
            parent_idx,
        }
    }
}

fn build_file_tree(commands: &Vec<Commands>) -> Vec<Node> {
    let mut nodes: Vec<Node> = vec![Node::new("/", 0)];

    let mut curr_node_idx = 0;
    for command in commands {
        println!("Command: {:?}", command);
        match command {
            Commands::ChangeDirectory { path } => match path {
                Some(str) => match str.as_str() {
                    "/" => {}
                    ".." => {
                        curr_node_idx = nodes[curr_node_idx]
                            .parent_idx
                            .expect("Failed to get parent directory")
                    }
                    str => {
                        curr_node_idx = *nodes[curr_node_idx]
                            .childs_idx
                            .get(str)
                            .unwrap_or_else(|| panic!("No directory with name: {}", str));
                    }
                },
                None => panic!("No path provided"),
            },
            Commands::List { list } => {
                for child in list {
                    let (name, size) = match child {
                        ListTypes::File { name, size } => (name, *size),
                        ListTypes::Dir { name } => (name, 0),
                    };
                    let new_node_idx = nodes.len();
                    let mut new_node = Node::new(name, size);
                    new_node.parent_idx = Some(curr_node_idx);
                    nodes.push(new_node);
                    nodes[curr_node_idx].childs_idx.insert(name, new_node_idx);
                }
            }
        }
    }
    nodes
}

fn get_answer_a(commands: &Vec<Commands>) -> u32 {
    let nodes = build_file_tree(commands);

    //0 is root always root
    let curr_node_idx = 0;

    todo!()
}

fn main() {
    let input_path = Path::new("./input/input.txt");
    let input = read_to_string(input_path).expect("Expecting valid input file");
    todo!()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    const TEST_INPUT: &str = include_str!("./test_input.txt");

    #[test]
    fn test_parse_input() {
        let expected_commands: Vec<Commands> = vec![
            Commands::ChangeDirectory {
                path: Some("/".to_string()),
            },
            Commands::List {
                list: vec![
                    ListTypes::Dir {
                        name: "a".to_string(),
                    },
                    ListTypes::File {
                        name: "b.txt".to_string(),
                        size: 14848514,
                    },
                    ListTypes::File {
                        name: "c.dat".to_string(),
                        size: 8504156,
                    },
                    ListTypes::Dir {
                        name: "d".to_string(),
                    },
                ],
            },
            Commands::ChangeDirectory {
                path: Some("a".to_string()),
            },
            Commands::List {
                list: vec![
                    ListTypes::Dir {
                        name: "e".to_string(),
                    },
                    ListTypes::File {
                        name: "f".to_string(),
                        size: 29116,
                    },
                    ListTypes::File {
                        name: "g".to_string(),
                        size: 2557,
                    },
                    ListTypes::File {
                        name: "h.lst".to_string(),
                        size: 62596,
                    },
                ],
            },
            Commands::ChangeDirectory {
                path: Some("e".to_string()),
            },
            Commands::List {
                list: vec![ListTypes::File {
                    name: "i".to_string(),
                    size: 584,
                }],
            },
            Commands::ChangeDirectory {
                path: Some("..".to_string()),
            },
            Commands::ChangeDirectory {
                path: Some("..".to_string()),
            },
            Commands::ChangeDirectory {
                path: Some("d".to_string()),
            },
            Commands::List {
                list: vec![
                    ListTypes::File {
                        name: "j".to_string(),
                        size: 4060174,
                    },
                    ListTypes::File {
                        name: "d.log".to_string(),
                        size: 8033020,
                    },
                    ListTypes::File {
                        name: "d.ext".to_string(),
                        size: 5626152,
                    },
                    ListTypes::File {
                        name: "k".to_string(),
                        size: 7214296,
                    },
                ],
            },
        ];
        assert_eq!(get_answer_a(&expected_commands), 95437);
        assert_eq!(
            parse_input(&String::from_str(TEST_INPUT).unwrap()),
            Ok(expected_commands)
        );
    }
}
