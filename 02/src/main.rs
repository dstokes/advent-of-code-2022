use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const ROCK: u8 = 1;
const PAPER: u8 = 2;
const SCISSOR: u8 = 3;

const WIN: u8 = 6;
const DRAW: u8 = 3;
const LOSS: u8 = 0;

fn to_moves(codes: (&str, &str)) -> (u8, u8) {
    (to_move(codes.0), to_move(codes.1))
}

fn to_move(code: &str) -> u8 {
    return match code {
        "A" | "X" => ROCK,
        "B" | "Y" => PAPER,
        "C" | "Z" => SCISSOR,
        _ => ROCK
    }
}

fn to_scores(moves: (u8, u8)) -> (u8, u8) {
    if moves.0== moves.1 {
        return (DRAW, DRAW)
    }

    match moves.0 {
        ROCK=>
            match moves.1 {
                SCISSOR=>(WIN, LOSS),
                PAPER=>(LOSS, WIN),
                _=>(LOSS, LOSS)
            }
        PAPER=>
            match moves.1 {
                ROCK=>(WIN, LOSS),
                SCISSOR=>(LOSS, WIN),
                _=>(LOSS, LOSS)
            }
        SCISSOR=>
            match moves.1 {
                PAPER=>(WIN, LOSS),
                ROCK=>(LOSS, WIN),
                _=>(LOSS, LOSS)
            }
        _ => (DRAW,DRAW)
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut score: i16 = 0;

    for line in reader.lines() {
        let moves = to_moves( line.unwrap().split_once(" ").unwrap());
        score += to_scores(moves).1 as i16 + moves.1 as i16;
    }

    println!("Score: {}", score);
    Ok(())
}
