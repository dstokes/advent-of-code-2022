use std::fs;
use std::collections::HashSet;

fn to_i(s: &str) -> u32 {
    s.parse().unwrap()
}

fn main() {
    let input =
        fs::read_to_string("input.txt").unwrap();

    // part 1
    let mut matches = 0;
    for line in input.lines() {
        // [start, end, start, end]
        let chars: Vec<&str> = line.split(&[',','-']).collect();
        let mut range1: HashSet<u32> = HashSet::new();
        let mut range2: HashSet<u32> = HashSet::new();

        // [start ... end]
        for i in to_i(chars[0])..=to_i(chars[1]) {
            range1.insert(i);
        }
        // [start ... end]
        for i in to_i(chars[2])..=to_i(chars[3]) {
            range2.insert(i);
        }

        if range1.is_subset(&range2) || range2.is_subset(&range1) {
            matches += 1
        }
    }
    println!("Part 1: {}", matches) // 471
}
