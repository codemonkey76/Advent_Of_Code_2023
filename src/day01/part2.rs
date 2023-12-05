use std::collections::HashMap;
use crate::day01::Input;
use crate::Output;

pub fn solve(input: &Input) -> Output {
    input.iter().copied().map(convert_to_digits).sum::<u32>().into()
}

fn convert_to_digits(input: &str) -> u32 {
    let word_to_digit_map = create_word_to_digit_map();

    let mut result_string = String::new();
    let mut current_word = String::new();
    let mut i = 0;
    let mut j = 0;

    while i < input.len() {
        let mut current_char = input.chars().nth(i).unwrap();

        if current_char.to_string().parse::<u32>().ok().is_some() {
            result_string.push(current_char);
            i+=1;
            continue;
        }

        while (i+j) < input.len() {
            current_char = input.chars().nth(i+j).unwrap();
            current_word.push(current_char);

            if let Some(&digit) = word_to_digit_map.get(current_word.as_str()) {
                result_string.push(digit);
                current_word.clear();
                i += j-1;
                break;
            }

            j += 1;
        }
        current_word.clear();
        j = 0;
        i += 1;
    }


    if let (Some(first), Some(last)) = (result_string.chars().next(), result_string.chars().next_back()) {
        let result = format!("{}{}", first, last).parse::<u32>().unwrap();
        return result;
    }
    0

}

fn create_word_to_digit_map() -> HashMap<&'static str, char> {
    [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9')
    ]
        .iter()
        .cloned()
        .collect()
}