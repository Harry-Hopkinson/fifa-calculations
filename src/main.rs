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
    let int_conversion_error: String = "Cannot convert to integer".to_string();
    #[allow(unused_assignments)]
    let mut strict: bool = false;

    let mut input = String::new();
    println!("Strict mode? (y/n)");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    strict = input.trim() == "y";

    for player in &players {
        if player.POS == "CB" || player.POS == "LB" || player.POS == "RB" {
            let average: f32 = (player.Defending.parse::<i32>().expect(&int_conversion_error) + player.StandingTackle.parse::<i32>().expect(&int_conversion_error) + player.SlidingTackle.parse::<i32>().expect(&int_conversion_error) + player.Heading.parse::<i32>().expect(&int_conversion_error) + player.Strength.parse::<i32>().expect(&int_conversion_error) + player.Aggression.parse::<i32>().expect(&int_conversion_error)) as f32 / 6.0;
            let average = average.round() as i32;

            if strict {
                if average == player.OVR.parse::<i32>().expect(&int_conversion_error) {
                    correct += 1;
                    println!("{} is correct", player.Name);
                } else {
                    wrong += 1;
                    println!("{} is wrong", player.Name);
                }
            } else {
                if average <= (player.OVR.parse::<i32>().expect(&int_conversion_error) + 3) && average >= (player.OVR.parse::<i32>().expect(&int_conversion_error)) - 3 {
                    correct += 1;
                    println!("{} is correct", player.Name);
                } else {
                    wrong += 1;
                    diff += (average - player.OVR.parse::<i32>().expect(&int_conversion_error)).abs();
                    println!("{}", diff);
                }
            }
            diff = 0;
        } else if player.POS == "CM" || player.POS == "CDM" || player.POS == "CAM" {
            let average: f32 = (player.Passing.parse::<i32>().expect(&int_conversion_error) + player.Dribbling.parse::<i32>().expect(&int_conversion_error) + player.Vision.parse::<i32>().expect(&int_conversion_error) + player.Vision.parse::<i32>().expect(&int_conversion_error) + player.Crossing.parse::<i32>().expect(&int_conversion_error) + player.ShortPassing.parse::<i32>().expect(&int_conversion_error) + player.LongPassing.parse::<i32>().expect(&int_conversion_error)) as f32 / 6.0;
            let average = average.round() as i32;

            if strict {
                if average == player.OVR.parse::<i32>().expect(&int_conversion_error) {
                    correct += 1;
                    println!("{} is correct", player.Name);
                } else {
                    wrong += 1;
                    println!("{} is wrong", player.Name);
                }
            } else {
                if average <= (player.OVR.parse::<i32>().expect(&int_conversion_error)) + 3 && average >= (player.OVR.parse::<i32>().expect(&int_conversion_error)) - 3 {
                    correct += 1;
                    println!("{} is correct", player.Name);
                } else {
                    wrong += 1;
                    diff += (average - player.OVR.parse::<i32>().expect(&int_conversion_error)).abs();
                    println!("{}", diff);
                }
            }
            diff = 0;
        }
    }

    println!("Correct: {} Wrong: {}", correct, wrong);
}
