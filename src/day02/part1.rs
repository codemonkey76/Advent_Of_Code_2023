use std::collections::HashMap;
use crate::day02::{Cube, Input};
use crate::day02::input::Game;
use crate::Output;

pub fn solve(input: &Input) -> Output {
    println!("Input: {:?}", input);
    input.iter().map(|_| 0).sum::<u32>().into()
}

// Take input of game, return possible or not possible.
fn is_game_possible(game: Game, rule: HashMap<Cube, u32>) -> bool {
    let mut possible = true;
    for (cube_type, count) in rule {
        if let Some(cubes_in_game) = game.cubes.get(&cube_type) {
            // picked some of these in the game
            if cubes_in_game > *count {
                possible = false;
                break;
            }
        }
    }
    return possible;
}
// create hashmap containing the larget number of each type

