use std::fs;

struct Command<'a> {
    command: &'a str,
    amount: i64,
}

fn main() {
    let filename = "input.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let commands_struct: Vec<Command> = contents.lines().map(|f| {
        let split: Vec<&str> = f.split_whitespace().collect();
        Command {
            command: split[0],
            amount: split[1].parse::<i64>().unwrap(),
        }
    }).collect();

    let mut horizontal: i64 = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in commands_struct.iter() {
        if command.command == "forward" {
            if aim != 0 {
                depth = depth + (aim*command.amount);
            }
            horizontal = horizontal + command.amount
        } else if command.command == "down" {
            aim = aim + command.amount
        } else if command.command == "up" {
            aim = aim - command.amount
        }
    }
    println!("Value x: {}, Value y: {}",horizontal ,depth);
    println!("Multip value: {}", horizontal*depth);

}
