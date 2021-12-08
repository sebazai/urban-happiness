use std::fs;

#[derive(Debug, Clone)]
struct BitArrays<'a> {
    generator_bits: Vec<&'a str>,
    scrubber_bits: Vec<&'a str>,
}
fn main() {
    let file = "input.txt";

    let content = fs::read_to_string(file).expect("Something went wrong reading file.");
    let content_lines: Vec<&str> = content.lines().collect();

    // println!("Lines original: {:?}", content_lines);

    // Length of one bit value in input
    let length = content_lines.first().unwrap().chars().count();

    let mut arrays = BitArrays {
        generator_bits: content_lines.clone(),
        scrubber_bits: content_lines.clone(),
    };

    for i in 0..length {
        // println!("i: {}", i);
        let new_content = calculate_bits_on_index_and_return_list(arrays.clone(), i);
        arrays = new_content;
    }

    println!("{:?}", arrays.generator_bits);
    println!("{:?}", arrays.scrubber_bits);

    let decimal_generator = isize::from_str_radix(&arrays.generator_bits[0].to_string(), 2).unwrap();
    let decimal_scrubber = isize::from_str_radix(&arrays.scrubber_bits[0].to_string(), 2).unwrap();
    
    println!("Generator value {}, Scrubber value {:?}", decimal_generator, decimal_scrubber);
    println!("Multiply {}", decimal_generator * decimal_scrubber)
}

fn calculate_bits_in_vec(content: Vec<&str>, bit_value: &str) -> Vec<i32> {
    let amount_of_bits = content.first().unwrap().chars().count();

    let mut bits_vec  = vec![0; amount_of_bits];
    for line in content {
        for (i, char) in line.chars().enumerate() {
            if char.to_string() == bit_value { bits_vec[i] += 1 }
        }
    }
    bits_vec
}

fn calculate_bits_on_index_and_return_list(content: BitArrays, index: usize) -> BitArrays {
    let len_ones = content.generator_bits.len() as i32;
    let len_zeros = content.scrubber_bits.len() as i32;

    let amount_of_ones_generator: Vec<i32> = calculate_bits_in_vec(content.generator_bits.clone(), "1");
    let amount_of_zeros_scrubber: Vec<i32> = calculate_bits_in_vec(content.scrubber_bits.clone(), "0");

    // println!("Amount of ones Generator: Total: {}, {:?}", len_ones, amount_of_ones_generator);
    // println!("Amount of zeros Scrubber: Total: {}, {:?}", len_zeros, amount_of_zeros_scrubber);

    let one_bits_majority_at_index = if len_ones - amount_of_ones_generator[index] <= amount_of_ones_generator[index] {
        "1"
    } else {
        "0"
    };
    let zero_bits_majority_at_index = if len_zeros - amount_of_zeros_scrubber[index] >= amount_of_zeros_scrubber[index] {
        "0"
    } else {
        "1"
    };

    let new_content = BitArrays {
        generator_bits: 
        if content.generator_bits.len() > 1 {
            content.generator_bits.clone().into_iter().filter(|s| s.chars().nth(index).unwrap().to_string() == one_bits_majority_at_index).collect()
        } else {
            content.generator_bits.clone()
        },
        scrubber_bits: 
        if content.scrubber_bits.len() > 1 {
            content.scrubber_bits.clone().into_iter().filter(|s| s.chars().nth(index).unwrap().to_string() == zero_bits_majority_at_index).collect()
        } else {
            content.scrubber_bits.clone()
        },
    };
    // println!("generator array: {:?}", new_content.generator_bits);
    // println!("scrubber array: {:?}", new_content.scrubber_bits);
    new_content
}

