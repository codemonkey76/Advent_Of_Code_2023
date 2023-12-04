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
        let id: u32;
        let mut draws = vec![];

        if let Some((game, rest)) = game_string.trim().split_once(':') {
            if let Some((_, game_number_string)) = game.trim().split_once(' ') {
                id = game_number_string.parse::<u32>().unwrap();
                for draw in rest.trim().split(';') {
                    let mut cube_draw: HashMap<Cube, u32> = HashMap::new();
                    for cube in draw.trim().split(',') {
                        if let Some((count, color)) = cube.trim().split_once(' ') {
                            cube_draw.insert(Cube::from(color), count.trim().parse::<u32>().unwrap());
                        } else {
                            panic!("Missing count or color");
                        }
                    }
                    draws.push(cube_draw);
                }
                Game {
                    id,
                    draws
                }
            } else {
                panic!("No valid game number");
            }
        } else {
            panic!("No Valid Game");
        }
    }
}