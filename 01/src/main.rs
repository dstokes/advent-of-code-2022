use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut curr = 0;
    let mut totals = Vec::new();

    for line in reader.lines() {
        let last_input =  line.unwrap();
        if last_input.len() == 0 {
            totals.push(curr);
            curr = 0;
        } else {
            curr += last_input.parse::<i32>().unwrap();
        }
    }
    totals.sort();

    let len = totals.len();
    let sum: i32 = totals[len-3..].iter().sum();

    println!("Max calories: {}", totals[len-1]);
    println!("Max calories from top three: {}", sum);
    Ok(())
}
