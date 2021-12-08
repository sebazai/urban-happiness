use std::fs;

fn main() {
    let file = "input.txt";

    let content = fs::read_to_string(file).expect("Something went wrong reading file.");
    let content_lines: Vec<&str> = content.lines().collect();

    let amount_of_rows = content_lines.len();
    let amount_of_bits = content_lines.first().unwrap().chars().count();

    let mut amount_of_one_bits = vec![0; amount_of_bits];
    
    for line in content_lines {
        for (index, char) in line.chars().enumerate() {
            if char.to_string() == "1" { amount_of_one_bits[index] += 1 }
        }
    }

    println!("Amount of one bits {:?}", amount_of_one_bits);
    let bits: Vec<i32> = amount_of_one_bits.iter().map(|f| {
        if f > &(amount_of_rows / 2) {
            1
        } else {
            0
        }
    }).collect();

    let bit_vec_as_string = bits.clone().into_iter().map(|i| i.to_string()).collect::<String>();
    let bit_vec_as_reverse_string = bits.into_iter().map(|i| {
        if i == 0 {
            "1"
        } else {
            "0"
        }
    }).collect::<String>();
    let decimal = isize::from_str_radix(&bit_vec_as_string.to_string(), 2).unwrap();
    let reverse_decimal = isize::from_str_radix(&bit_vec_as_reverse_string, 2).unwrap();
    
    println!("Decimal value {}, reverse value {:?}", decimal,reverse_decimal);
    println!("Multip {}", decimal * &reverse_decimal)
}
