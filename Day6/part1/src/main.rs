use std::fs;

fn main() {
    println!("Hello, world!");
    const INPUT: &str = "input.txt";
    let content = fs::read_to_string(INPUT).expect("No content");

    let mut days_left  =[0_u32; 9];
    content.split(",").for_each(|s| days_left[s.parse::<usize>().unwrap()] += 1);
    println!("{:?}", days_left);

    for d in 1..=80 {
        let new_borns = (d + 8) % 9;
        let parents = (d + 6) % 9;
        // println!("{} {}", new_borns, parents);
        // println!("{:?}", days_left);

        days_left[parents as usize] += days_left[new_borns as usize];
    }
    let fishes: u32 = days_left.iter().sum();
    println!("{:?}", fishes)
}


