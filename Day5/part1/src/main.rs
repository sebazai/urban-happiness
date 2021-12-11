use std::{fs, collections::HashMap, cmp::{min, max}};

type Coordinate = (i32, i32);

fn main() {
    const INPUT: &str = "input.txt";

    let file: String = fs::read_to_string(INPUT).expect("No file found");
    let coordinates: Vec<(Coordinate, Coordinate)> = file.lines().into_iter().map(|line| {
        let split:Vec<&str> = line.split(" -> ").collect();
        let from = split.first().unwrap();
        let to = split.last().unwrap();

        let (x1, y1) = from.split_once(",").unwrap();
        let (x2, y2) = to.split_once(",").unwrap();

        // println!("x1: {}, x2: {}, y1: {}, y2: {}", x1, x2, y1, y2);

        ((x1.parse::<i32>().unwrap(), y1.parse::<i32>().unwrap()), (x2.parse::<i32>().unwrap(), y2.parse::<i32>().unwrap()))
    }).collect();
    
    let mut vents: HashMap<Coordinate, i32> = HashMap::new();

    coordinates.iter().for_each(|&((x1, y1), (x2,y2))| {
        if x1 == x2 {
            for y in min::<&i32>(&y1, &y2).clone()..=max::<&i32>(&y1, &y2).clone() {
                let coordinate: Coordinate = (x1, y);
                vents.insert(coordinate, vents.get(&coordinate).unwrap_or(&0).clone() + 1);
            }
        } else if y1 == y2 {
            for x in min::<&i32>(&x1, &x2).clone()..=max::<&i32>(&x1, &x2).clone() {
                let coordinate: Coordinate = (x, y1);
                vents.insert(coordinate, vents.get(&coordinate).unwrap_or(&0).clone() + 1);
            }
        }
    });

    let mut intersections = 0;
    vents.values().for_each(|vent| if vent >= &2 { intersections += 1 });

    println!("Intersections: {}", intersections);
}
