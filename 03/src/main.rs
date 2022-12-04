use std::fs;

fn priority(c: char) -> u16 {
    let res = c as u16;
    res - (if res > 96 { 96 } else { 38 })
}

// return chars in both strings
fn matches(a: &str, b: &str) -> Vec<char> {
    let mut result = Vec::new();
    for c in a.chars() {
        if b.find(c) != None {
            result.push(c)
        }
    }
    result
}

fn check_ruck(contents: &str) -> u16 {
    let (first_c, second_c) = contents.split_at(contents.len()/2);
    priority(matches(first_c, second_c)[0])
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    // part 1
    let result: u16 = input.lines()
        .map(check_ruck)
        .sum();
    println!("Part 1 priorities: {}", result); // 8493

    // part 2
    let lines: Vec<&str> = input.lines().collect();
    let mut result: u16 = 0;

    let mut i = 0;
    // there is certainly a better way to chunk these lines
    while i < lines.len() {
        let subset = matches(lines[i], lines[i+1])
            // converting the match vector back into a string
            .iter().cloned().collect::<String>();
        // this index lookup is brittle. should be using a set for matches
        result += priority(matches(&subset, lines[i+2])[0]);
        i += 3
    }
    println!("Part 2 priorities: {}", result); // 2552
}
