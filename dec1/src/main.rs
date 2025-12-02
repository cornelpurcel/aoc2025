use core::panic;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::ops::Div;

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            Direction::Left => "left",
            Direction::Right => "right",
        };
        write!(f, "{text}")
    }
}

#[derive(Debug, Clone)]
struct ParseError;

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cannot parse the shit you put in!")
    }
}

impl Direction {
    fn from_char(character: char) -> Result<Self, ParseError> {
        match character {
            'R' => Ok(Direction::Right),
            'L' => Ok(Direction::Left),
            _ => Err(ParseError {}),
        }
    }
}
#[derive(Debug)]
struct Command {
    direction: Direction,
    amount: u16,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let direction = &self.direction;
        let amount = self.amount;
        write!(f, "turning {direction} {amount} times")
    }
}

struct SafeState {
    position: u8,
}

impl SafeState {
    fn new() -> Self {
        SafeState { position: 50 }
    }
    fn execute_command(&mut self, command: Command) -> i32 {
        let new_raw_position: i32 = match command.direction {
            Direction::Left => self.position as i32 - command.amount as i32,
            Direction::Right => self.position as i32 + command.amount as i32,
        };
        let left_pass = if command.direction == Direction::Left
            && command.amount > (self.position as u16)
            && self.position != 0
        {
            1
        } else {
            0
        };
        let zero_passes = new_raw_position.div(100).abs();
        let new_position = new_raw_position.rem_euclid(100);
        let arrived_at_zero = if new_raw_position == 0 { 1 } else { 0 };
        println!(
            "Turning {} from {} and got to {} with {} passes",
            command,
            self.position,
            new_position,
            left_pass + zero_passes + arrived_at_zero
        );

        self.position = new_position as u8;
        if left_pass + zero_passes + arrived_at_zero < 0 {
            dbg!(command);
            dbg!(left_pass);
            dbg!(zero_passes);
            dbg!(new_position);
            panic!("wtf passes can't be zero???")
        }
        left_pass + zero_passes + arrived_at_zero
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("reading the input from {file_path}");

    let commands = parse_file(file_path)?;

    let size = commands.len();

    let mut zeroes: i32 = 0;
    let mut state = SafeState::new();
    for command in commands {
        let res = state.execute_command(command);
        zeroes += res;
    }

    println!("analyzed {size} commands and found {zeroes} zeroes");
    Ok(())
}

fn parse_file(file_path: &String) -> Result<Vec<Command>, Box<dyn Error>> {
    let mut result: Vec<Command> = Vec::new();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    for line in contents.split_whitespace() {
        let direction = Direction::from_char(line.chars().next().unwrap())?;
        let amount: u16 = line[1..].parse()?;
        let new_command = Command { direction, amount };
        result.push(new_command);
    }

    Ok(result)
}
