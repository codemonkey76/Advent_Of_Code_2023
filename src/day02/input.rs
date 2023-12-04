use std::collections::HashMap;
use regex::Regex;
use crate::day02::{Cube, Input};

const INPUT: &str = include_str!("../../input/02/input.txt");

pub fn read() -> Input {
    INPUT
        .trim()
        .split("\n")
        .map(parse_game)
        .collect()
}

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub cubes: HashMap<Cube, u32>
}

fn parse_game(game_string: &str) -> Game {
    if let Some((game, rest)) = game_string.split_once(':') {
        if let Some((_, game_number_string)) = game.split_once(' ') {
            let game_id = game_number_string.parse::<u32>().unwrap();
            let mut cubes = HashMap::new();
            for part in  rest.trim().split(';') {
                for draw in part.trim().split(',') {
                    if let Some((number, color)) = draw.trim().split_once(' ') {
                        cubes.insert(Cube::from(color), number.parse::<u32>().unwrap());
                    }
                }
            }

            Game {
                id: game_id,
                cubes
            }
        } else {
            panic!("No no game id found");
        }
    } else {
        panic!("Invalid Game String");
    }
}