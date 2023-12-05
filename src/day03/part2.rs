use crate::day03::{Input, Position};
use crate::Output;

pub fn solve(input: &Input) -> Output {
    let possible_gears = find_possible_gears(input);
    let gears = possible_gears.filter(is_gear);

    input.iter().map(|_| 0).sum::<u32>().into()
}

pub type FoundGears = Vec<Position>;

pub fn check_string_for_number(row: &[char], col: usize, len: usize) -> bool {
    let start_col = if col > 0 { col - 1 } else { col };
    let end_col = if len+col+1 > row.len() { len + col } else { len+col+1 };

    if let Some(slice) = row.get(start_col..end_col) {
        for char in slice {
            if char.is_ascii_digit()
        }
    }
    false

}
fn is_gear(possible_gear: Position, input: &Input) -> bool {
    
    let mut count = 0;
    let has_row_above = possible_gear.row > 0;
    let has_current_row = input.get(possible_gear.row).is_some();
    let has_next_row = input.get(possible_gear.row+1).is_some();

    if has_row_above {
        if let Some(slice) = input[possible_gear.row - 1][row.get(
    }

    if has_current_row {

    }

    if has_next_row {

    }


    count == 2
}

fn find_possible_gears(input: &Input) -> FoundGears {
    let mut result: FoundGears = vec![];

    for row in 0..input[0].len() {
        for column in 0..input.len() {
            if input[row][column] == '*' {
                result.push(Position::new(column, row));
            }
        }
    }

    result
}