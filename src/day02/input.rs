use std::collections::HashMap;
use crate::day02::{Cube, Input};

const INPUT: &str = include_str!("../../input/02/input.txt");

pub fn read() -> Input {
    INPUT
        .trim()
        .split('\n')
        .map(Game::parse)
        .collect()
}

#[derive(Debug, Clone)]
pub struct Game {
    pub id: u32,
    pub draws: Vec<HashMap<Cube, u32>>,
}


impl Game {
    pub fn parse(game_string: &str) -> Self {
        let (game, rest) = game_string
            .trim()
            .split_once(':')
            .expect("No Valid Game");

        let (_, game_number_string) = game
            .trim()
            .split_once(' ')
            .expect("No Valid Game Number");

        let id = game_number_string
            .parse::<u32>()
            .expect("Invalid game number");

        let draws = rest
            .trim()
            .split(';')
            .map(|draw| {
                draw
                    .trim()
                    .split(',')
                    .map(|cube| {
                        let (count, color) = cube
                            .trim()
                            .split_once(' ')
                            .expect("Missing count or color");
                        (Cube::from(color), count.parse::<u32>().expect("Invalid count")
                    )
            }).collect::<HashMap<Cube, u32>>()
        }).collect();

        Game { id, draws }

    }
}