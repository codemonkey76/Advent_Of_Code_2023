use crate::day03::{Input, Position};
use crate::Output;

pub fn solve(input: &Input) -> Output {
    // let s = "123....34...5666....";
    // let pos = 8;
    // let num = get_number(pos, &s.chars().collect::<Vec<char>>());

    let numbers = get_surrounding_numbers(Position::new(1, 3), input);

    println!("Numbers: {:?}", numbers);

    input.iter().map(|_| 0).sum::<u32>().into()
}

fn get_numbers(row: &[char], col: usize) -> Vec<u32> {
    let starting_pos = if col > 0 { col - 1 } else { col };
    let ending_pos = if col < row.len() { col + 1 } else { col };

    for pos in starting_pos..ending_pos {
        if row[pos].is_ascii_digit() {
            // get full_number
        }
    }

    println!("Checking row: {}", row[starting_pos..=ending_pos].iter().collect::<String>());
    vec![]
}

fn get_surrounding_numbers(pos_of_star: Position, input: &Input) -> Vec<u32> {
    let mut numbers = vec![];

    if pos_of_star.row > 0 {
        println!("Checking row above");
        if let Some(above) = input.get(pos_of_star.row - 1) {
            numbers.append(&mut get_numbers(above, pos_of_star.col));
        }
    }

    println!("Checking current row");
    if let Some(current) = input.get(pos_of_star.row) {
        numbers.append(&mut get_numbers(current, pos_of_star.col));
    }

    if pos_of_star.row != input.len() {
        println!("Checking row below");
        if let Some(next) = input.get(pos_of_star.row+1) {
            numbers.append(&mut get_numbers(next, pos_of_star.col));
        }
    }

    numbers
}

// Function to take a column position of a number and a string slice
fn get_number(col: usize, row: &[char]) -> u32 {
    let mut start_pos = 0;
    let mut end_pos= row.len();

    for pos in (0..=col).rev() {
        if !row[pos].is_ascii_digit() {
            start_pos = pos+1;
            break;
        }
    }

    for pos in col..row.len() {
        if !row[pos].is_ascii_digit() {
            end_pos = pos;
            break;
        }
    }

    row[start_pos..end_pos]
        .iter()
        .collect::<String>()
        .parse::<u32>()
        .expect("Unable to parse number")
}