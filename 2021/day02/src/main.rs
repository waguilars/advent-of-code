use std::fs;

fn main() {
    println!("Hello, world!");
    let filename = "input";

    let content = fs::read_to_string(filename).expect("Cannot read file");

    let commands: Vec<Command> = content
        .split("\n")
        .filter(|value| !value.is_empty())
        .map(|value| parse_command(value))
        .collect();

    let mut horizontal: u32 = 0;
    let mut depth: u32 = 0;
    let mut aim: u32 = 0;

    for command in commands {
        match command.instruction.as_ref() {
            "forward" => {
                horizontal += command.steps;
                depth += aim * command.steps;
            },
            "down" => aim +=  command.steps,
            "up" => aim -= command.steps,
            _ => {}
        }
    }

    let result = horizontal * depth;
    println!("result -> {}", result);
}

#[derive(Debug)]
struct Command<'a> {
    instruction: &'a str,
    steps: u32,
}

fn parse_command(string: &str) -> Command {
    let vec = string.split_whitespace().collect::<Vec<&str>>();

    Command {
        instruction: vec[0],
        steps: vec[1].parse().unwrap(),
    }
}
