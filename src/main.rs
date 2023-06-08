mod player;
use crate::player::Player;
use serde_json::{Result, Value};
use std::fs::File;

fn main() {
    // import the data/data.json file
    let data = File::open("data/data.json").expect("Unable to open file");
    // parse the data into a vector of players
    let players: serde_json::Value =
        serde_json::from_reader(&data).expect("JSON was not well-formatted");
    println!("{}", players);

}
