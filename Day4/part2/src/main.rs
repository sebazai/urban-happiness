use std::{fs, cmp::Ordering};

#[derive(Debug, Clone)]
pub struct Bingo {
    data: Vec<Vec<i32>>,
    // marked: Box<[[bool; 5]; 5]>,
    marked: Vec<Vec<bool>>,
    solved_at_index: i32,
}
fn main() {
    const INPUT: &str = "input.txt";
    let content = fs::read_to_string(INPUT).expect("Cant read file");
    let bingo_numbers: Vec<i32> = content.lines().next().unwrap().split(",").map(|s| s.parse::<i32>().unwrap()).collect();

    let bingo_boards: Vec<&str> = content.lines().skip(2).collect();
    let mut constructed_bingo_boards: Vec<Bingo> = vec![];
    
    // println!("{:?}", bingo_numbers);

    // Parse input of boards
    let mut board: Vec<Vec<i32>> = vec![];
    let mut i = 0;
    for line in bingo_boards.into_iter() {
        if i == 5 {
            i = 0;
            // println!("{:?}", board);
            constructed_bingo_boards.push(Bingo {
                data: board,
                marked: vec![vec![false; 5]; 5],
                solved_at_index: 0,
            });
            board = vec![];
        } else {
            let trimmed = line.trim();
            let splitted: Vec<i32> = trimmed.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
            board.push(splitted);
            i += 1;
        }
    }

    // println!("{:?}", constructed_bingo_boards.len());
    let mut solved_bingo_boards: Vec<Bingo> = vec![];

    // Solve each board
    for board in constructed_bingo_boards.into_iter() {
        solved_bingo_boards.push(solve_bingo_board(board, &bingo_numbers));
    }

    // Find the winner
    // let mut asd = solved_bingo_boards.clone().into_iter().map(|f| f.solved_at_index).collect::<Vec<i32>>();
    // asd.sort_unstable(); 
    // println!("{:?}", asd);
    let winner = solved_bingo_boards.into_iter()
        .max_by(|a,b| a.solved_at_index.partial_cmp(&b.solved_at_index).unwrap_or(Ordering::Greater))
            .expect("No winner found.");
    
    // println!("{:?}", winner);
    let sum = calculate_sum_from_unmarked(&winner);
    let multiplier = bingo_numbers.into_iter().nth(winner.solved_at_index.try_into().unwrap()).expect("No value on index");
    // println!("{} {}", multiplier, sum);
    println!("{}",  multiplier * sum);
}

fn calculate_sum_from_unmarked(winner_board: &Bingo) -> i32 {
    let mut sum = 0;
    for i in 0..5 {
        for j in 0..5 {
            if !winner_board.marked[i][j] {
                sum += winner_board.data[i][j];
            }
        }
    }
    sum
}

fn solve_bingo_board(mut board: Bingo, numbers: &Vec<i32>) -> Bingo {
    for (i, number) in numbers.into_iter().enumerate() {
        let check = check_if_number_in_board(&board.data, &number);
        if check != (6, 6) {
            board.marked[check.0][check.1] = true;
            let complete = check_if_board_complete(&board);
            if complete {
                board.solved_at_index = i as i32;
                return board;
            }
        }
    }
    // Not solved ever with given input
    return board;
}

fn check_if_number_in_board(board: &Vec<Vec<i32>>, number: &i32) -> (usize, usize) {
    for i in 0..5 {
        for j in 0..5 {
            if board[i][j] == number.to_owned() {
                return (i, j)
            }
        }
    }
    (6, 6)
}

fn check_if_board_complete(board: &Bingo) -> bool {
    for row in &board.marked {
        let row_check = row.into_iter().all(|f| f.to_owned() == true);
        // If row bingo
        if row_check {
            return row_check
        }
    }
    for i in 0..5 {
        let mut col = true;
        for j in 0..5 {
            if !board.marked[j][i] {
                col = false;
            }
        }
        // Check if column bingo
        if col {
            return col;
        }
    }
    false
}