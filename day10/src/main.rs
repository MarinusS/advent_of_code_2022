#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Addx(i32),
}

struct Device {
    x: i32,
    rom: Vec<Instruction>,
    inst_pointer: usize,
    cycle: u32,
    cur_inst_cycle_count: i32,
    cur_instruction: Instruction,
    cur_inst_done: bool,
}

impl Device {
    fn new(rom: Vec<Instruction>) -> Device {
        Device {
            x: 1,
            cycle: 0,
            rom,
            inst_pointer: 0,
            cur_instruction: Instruction::Noop,
            cur_inst_done: true,
            cur_inst_cycle_count: 0,
        }
    }

    fn load_next_instruction(&mut self) {
        if self.cur_inst_done && self.inst_pointer < self.rom.len() {
            self.cur_instruction = self.rom[self.inst_pointer];
            self.cur_inst_done = false;
            self.cur_inst_cycle_count = 0;
            self.inst_pointer += 1;
        }
    }

    fn run_instruction(&mut self) {
        self.cur_inst_cycle_count += 1;
        match self.cur_instruction {
            Instruction::Noop => {
                if self.cur_inst_cycle_count == 1 {
                    self.cur_inst_done = true;
                }
            }
            Instruction::Addx(value) => {
                if self.cur_inst_cycle_count == 2 {
                    self.x += value;
                    self.cur_inst_done = true;
                }
            }
        }
    }

    fn run_cycle(&mut self) -> (i32, bool) {
        if self.inst_pointer >= self.rom.len() && self.cur_inst_done {
            return (self.x, true);
        }

        self.cycle += 1;
        self.load_next_instruction();
        let x_during_cycle = self.x;
        self.run_instruction();

        (x_during_cycle, false)
    }

    fn run_all(&mut self) -> Vec<i32> {
        let mut reg_values = vec![];

        #[allow(unused_assignments)]
        let (mut x, mut done) = self.run_cycle();

        while !done {
            let cycle = self.cycle as i32;
            println!("Cycle {cycle}: x = {x} signal = {}", cycle * x);
            (x, done) = self.run_cycle();
            reg_values.push(x);
        }
        let cycle = self.cycle as i32;
        reg_values.push(x);
        println!("Cycle {cycle}: x = {x} signal = {}", cycle * x);
        reg_values
    }

    fn draw_crt(&mut self, width: u32, height: u32) -> Vec<Vec<char>> {
        let mut screen: Vec<Vec<char>> = (0..height)
            .map(|_| (0..width).map(|_| '.').collect())
            .collect();

        for cursor in 0..width * height {
            self.run_cycle();
            let x = cursor % width;
            let y = cursor / width;

            if self.x - 1 <= x as i32 && x as i32 <= self.x + 1 {
                screen[y as usize][x as usize] = '#';
            }
        }
        screen
    }
}

fn parse_line(line: &str) -> Instruction {
    let mut fields = line.split(' ');
    match fields.next() {
        Some("noop") => Instruction::Noop,
        Some("addx") => Instruction::Addx(
            fields
                .next()
                .expect("Expected a second field after addx instruction")
                .parse()
                .expect("Expected a field parsable as int"),
        ),
        _ => panic!("Expected valid instruction"),
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(parse_line).collect()
}

fn get_answer_a(input: &str) -> i32 {
    let instructions = parse_input(input);
    let mut device = Device::new(instructions);
    let reg_values = device.run_all();

    //for (cycle, x) in reg_values.iter().enumerate() {
    //    println!(
    //        "Cycle: {} | X: {x} | sig: {}",
    //        (cycle + 1),
    //        (cycle as i32 + 1) * x
    //    );
    //}

    let cycles_of_interest: [usize; 6] = [20, 60, 100, 140, 180, 220];
    cycles_of_interest
        .iter()
        .map(|c| {
            let x = reg_values[*c - 2];
            let c = *c as i32;
            let sig = x * c;
            println!("Cycle {c}: x = {x}, sig = {sig}");
            sig
        })
        .sum()
}

//Still bug somewhere, couldn't be bothered
fn get_answer_b(input: &str) -> String {
    let instructions = parse_input(input);
    let mut device = Device::new(instructions);
    let screen = device.draw_crt(40, 6);

    screen.iter().fold(String::new(), |acc, line| {
        acc + &(line
            .iter()
            .fold(String::new(), |acc: String, pixel| acc + &pixel.to_string())
            + "\n")
    })
}

fn main() {
    let input_path = "input/input.txt";
    let input = std::fs::read_to_string(input_path).unwrap();

    println!("Answer A: {}", get_answer_a(&input));
    println!("Answer B:\n {}", get_answer_b(&input));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn mini_test() {
        let rom = vec![
            Instruction::Noop,
            Instruction::Addx(3),
            Instruction::Addx(-5),
            Instruction::Noop,
            Instruction::Addx(1),
        ];
        let mut device = Device::new(rom);
        assert_eq!(device.run_cycle().0, 1);
        assert_eq!(device.run_cycle().0, 1);
        assert_eq!(device.run_cycle().0, 1);
        assert_eq!(device.run_cycle().0, 4);
        assert_eq!(device.run_cycle().0, 4);
        assert_eq!(device.run_cycle().0, -1);
        assert_eq!(device.run_cycle().0, -1);
        assert_eq!(device.run_cycle().0, -1);
        assert_eq!(device.run_cycle().0, 0);
    }
    #[test]
    fn test_get_answer_a() {
        let test_input: &str = include_str!("test_input.txt");
        println!("{}", test_input);
        assert_eq!(get_answer_a(test_input), 13140);
    }
}
