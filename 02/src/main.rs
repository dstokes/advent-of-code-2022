use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const ROCK: u8 = 1;
const PAPER: u8 = 2;
const SCISSOR: u8 = 3;

const WIN: u8 = 6;
const DRAW: u8 = 3;
const LOSS: u8 = 0;

fn to_move(code: &str) -> u8 {
    return match code {
        "A" | "X" => ROCK,
        "B" | "Y" => PAPER,
        "C" | "Z" => SCISSOR,
        _ => ROCK
    }
}

// convert move codes into move ints
fn to_moves(codes: (&str, &str)) -> (u8, u8) {
    (to_move(codes.0), to_move(codes.1))
}

// which shape beats the passed in shape
fn beats(shape: u8) -> u8 {
    match shape {
        ROCK    => PAPER,
        PAPER   => SCISSOR,
        SCISSOR => ROCK,
        _       => 0
    }
}

// which shape loses to the passed in shape
fn loses_to(shape: u8) -> u8 {
    match shape {
        ROCK    => SCISSOR,
        PAPER   => ROCK,
        SCISSOR => PAPER,
        _       => 0
    }
}

// calculate round score
fn to_score(moves: (u8, u8)) -> u8 {
    if moves.0 == moves.1 {
        return DRAW
    }
    if moves.1 == beats(moves.0) {
        return WIN
    }
    LOSS
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut part1_score: i16 = 0;
    let mut part2_score: i16 = 0;

    for line in reader.lines() {
        let ln = line.unwrap();
        let codes = ln.split_once(" ").unwrap();

        // part 1
        let mut moves = to_moves(codes);
        part1_score += to_score(moves) as i16 + moves.1 as i16;

        // part 2
        // change shape based on code
        match codes.1 {
            "X" => moves.1 = loses_to(moves.0),
            "Y" => moves.1 = moves.0,
            "Z" => moves.1 = beats(moves.0),
            _   => ()
        };
        part2_score += to_score(moves) as i16 + moves.1 as i16;
    }

    println!("Part 1 score: {}", part1_score); // 14069
    println!("Part 2 score: {}", part2_score); // 12411
    Ok(())
}
