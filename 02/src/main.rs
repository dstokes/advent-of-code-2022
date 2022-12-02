use std::fs::File;
use std::collections::HashMap;
use std::io::{self, prelude::*, BufReader};

const ROCK: u8 = 1;
const PAPER: u8 = 2;
const SCISSOR: u8 = 3;

fn hand_wins(moves: Split<&str>) -> bool {
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
        println(
            hand_wins(
                line.split_whitespace()
            )
        )
    }

    println!("Score: ");
    Ok(())
}
