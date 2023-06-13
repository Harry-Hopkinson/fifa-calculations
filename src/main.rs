use std::fs::File;
use serde_json;
use std::io;

mod player;
use crate::player::Player;

fn main() {
    let data = File::open("data/data.json").expect("Unable to open file");
    let players: Vec<Player> = serde_json::from_reader(&data).expect("Unable to read file");

    let mut correct: i32 = 0;
    let mut wrong: i32 = 0;
    let mut diff: i32 = 0;
    #[allow(unused_assignments)]
    let mut strict: bool = false;

    let mut input = String::new();
    println!("Strict mode? (y/n)");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    strict = input.trim() == "y";

    for player in &players {
        if player.pos == "CB" || player.pos == "LB" || player.pos == "RB" {
            let average: f32 = (player.defending + player.standing_tackle + player.sliding_tackle + player.heading + player.strength + player.aggression) as f32 / 6.0;
            let average = average.round() as i32;

            if strict {
                if average == player.ovr {
                    correct += 1;
                    println!("{} is correct", player.name);
                } else {
                    wrong += 1;
                    println!("{} is wrong", player.name);
                }
            } else {
                if average <= player.ovr + 3 && average >= player.ovr - 3 {
                    correct += 1;
                    println!("{} is correct", player.name);
                } else {
                    wrong += 1;
                    diff += (average - player.ovr).abs();
                    println!("{}", diff);
                }
            }
            diff = 0;
        } else if player.pos == "CM" || player.pos == "CDM" || player.pos == "CAM" {
            let average: f32 = (player.passing + player.dribbling + player.vision + player.crossing + player.short_passing + player.long_passing) as f32 / 6.0;
            let average = average.round() as i32;

            if strict {
                if average == player.ovr {
                    correct += 1;
                    println!("{} is correct", player.name);
                } else {
                    wrong += 1;
                    println!("{} is wrong", player.name);
                }
            } else {
                if average <= player.ovr + 3 && average >= player.ovr - 3 {
                    correct += 1;
                    println!("{} is correct", player.name);
                } else {
                    wrong += 1;
                    diff += (average - player.ovr).abs();
                    println!("{}", diff);
                }
            }
            diff = 0;
        }
    }

    println!("Correct: {} Wrong: {}", correct, wrong);
}
