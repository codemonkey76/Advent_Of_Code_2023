use crate::day03::{FoundNumber, Input};
use crate::Output;

pub fn solve(input: &Input) -> Output {
    let found_numbers = find_part_numbers(input);

    let filtered: Vec<_> = found_numbers
        .iter()
        .filter(|found| is_valid_part(found, input))
        .map(|found| found.number.parse::<u32>().unwrap())
        .collect();

    filtered.iter().sum::<u32>().into()
}


pub fn check_string_for_symbol(row: &[char], col: usize, len: usize) -> bool {
    let mut valid = false;
    let start_col = if col > 0 { col - 1 } else { col };
    let end_col = if len+col+1 > row.len() { len+col } else { len + col + 1 };

    if let Some(slice) = row.get(start_col..end_col) {
        println!("{}", slice.iter().collect::<String>());
        for char in slice {
            if !char.is_ascii_digit() && *char != '.' {
                valid = true;
            }
        }
    } else {
        panic!("Something went wrong attempting to slice string");
    }
    valid
}

pub fn is_valid_part(found_number: &FoundNumber, input: &Input) -> bool {

    let check_row = |row : Option<&Vec<char>>| {
        row.map_or(false, |r| check_string_for_symbol(r, found_number.position.col, found_number.number.len()))
    };

    let has_row_above = found_number.position.row > 0;
    let has_current_row = input.get(found_number.position.row).is_some();
    let has_next_row = input.get(found_number.position.row+1).is_some();

    let result = (has_row_above && check_row(input.get(found_number.position.row-1)))
        || (has_current_row && check_row(input.get(found_number.position.row)))
        || (has_next_row && check_row(input.get(found_number.position.row+1)));

    result

}



pub fn find_part_numbers(input: &Input) -> Vec<FoundNumber> {
    let mut result: Vec<FoundNumber> = vec![];

    for row in 0..input[0].len() {
        let mut number = String::new();
        let mut found_at = 0;

        for column in 0..input.len() {
            if input[row][column].is_ascii_digit() {
                if number.is_empty() {
                    found_at = column;
                }
                number.push(input[row][column]);
            } else if ! number.is_empty() {
                result.push(FoundNumber::new(number.clone(), found_at, row));
                number.clear();
                found_at = 0;

            }
        }
        if ! number.is_empty() {
            result.push(FoundNumber::new(number.clone(), found_at, row));
            number.clear();
        }
    }

    result
}
