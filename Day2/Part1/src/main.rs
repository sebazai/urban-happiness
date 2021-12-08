use std::fs;

struct Command<'a> {
    command: &'a str,
    amount: i32,
}

fn main() {
    let filename = "input.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let commands_struct: Vec<Command> = contents.lines().map(|f| {
        let split: Vec<&str> = f.split_whitespace().collect();
        Command {
            command: split[0],
            amount: split[1].parse::<i32>().unwrap(),
        }
    }).collect();

    let mut x = 0;
    let mut y = 0;

    for command in commands_struct.iter() {
        if command.command == "forward" {
            x = x + command.amount
        } else if command.command == "down" {
            y = y + command.amount
        } else if command.command == "up" {
            y = y - command.amount
        }
    }
    println!("Value x: {}, Value y: {}",x ,y);
    println!("Multip value: {}", x*y);

}