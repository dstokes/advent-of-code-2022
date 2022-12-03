use std::fs;

fn priority(c: u16) -> u16 {
    c - (if c > 96 { 96 } else { 38 })
}

fn match_sum(a: &str, b: &str) -> u16 {
    let mut total: u16 = 0;
    for c in a.chars() {
        if b.find(c) != None {
            total += priority(c as u16);
            break // break after first match
        }
    }
    total
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut p_total: u16 = 0;

    for line in input.lines() {
        let (first_c, second_c) = line.split_at(line.len()/2);
        p_total += match_sum(first_c, second_c);
    }

    println!("Part 1 priorities: {}", p_total); // 8493
}
