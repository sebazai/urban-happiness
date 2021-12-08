use std::fs;

fn main() {
    let filename = "input.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines: Vec<i32> = contents.lines().map(|f| f.parse::<i32>().unwrap()).collect();
    let mut prev = None::<&i32>;
    let increses: Vec<&i32> = lines.iter().filter(|f| match prev {
        Some(ref n) if n < f => {
            prev = Some(f);
            true
        }
        _ => {
            prev = Some(f);
            false
        }
    }).collect();
    println!("Length {}", increses.len())
}