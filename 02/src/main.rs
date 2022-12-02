use std::fs::File;
use std::collections::HashMap;
use std::str::Split;
use std::io::{self, prelude::*, BufReader};

const ROCK: u8 = 1;
const PAPER: u8 = 2;
const SCISSOR: u8 = 3;

fn str_to_move(

fn hand_wins(moves: (&str, &str)) -> bool {
    let (hand_one, hand_two) = moves;
    match hand_one {
        ROCK=>hand_two==SCISSOR,
        PAPER=>hand_two==ROCK,
        SCISSOR=>hand_two==PAPER,
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", hand_wins(line.unwrap().split_once(" ").unwrap()))
    }

    println!("Score: ");
    Ok(())
}
