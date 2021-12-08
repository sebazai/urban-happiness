use std::fs;

fn main() {
    let filename = "input.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines_as_numbers: Vec<i32> = contents.lines().map(|f| f.parse::<i32>().unwrap()).collect();
    let lines_windows_sum: Vec<i32> = lines_as_numbers.windows(3).into_iter().map(|o| o.into_iter().sum()).collect();

    let mut prev = None::<&i32>;
    let increases: Vec<&i32> = lines_windows_sum.iter().filter(|f| match prev {
        Some(ref n) if n < f => {
            prev = Some(f);
            true
        }
        _ => {
            prev = Some(f);
            false
        }
    }).collect();
    println!("Length {}", increases.len())
}