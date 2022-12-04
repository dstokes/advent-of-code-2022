use std::fs;
use std::collections::HashSet;

fn to_i(s: &str) -> u32 {
    s.parse().unwrap()
}

fn main() {
    let input =
        fs::read_to_string("input.txt").unwrap();

    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        // [start, end, start, end]
        let chars: Vec<&str> = line.split(&[',','-']).collect();
        let mut range1: HashSet<u32> = HashSet::new();
        let mut range2: HashSet<u32> = HashSet::new();

        // create range [start ... end]
        for i in to_i(chars[0])..=to_i(chars[1]) {
            range1.insert(i);
        }
        // create range [start ... end]
        for i in to_i(chars[2])..=to_i(chars[3]) {
            range2.insert(i);
        }

        // part 1
        if range1.is_subset(&range2) || range2.is_subset(&range1) {
            part1 += 1
        }

        // part 2
        if range1.is_disjoint(&range2) == false {
            part2 += 1;
        }
    }
    println!("Part 1: {}", part1);  // 471
    println!("Part 2: {}", part2); // 888
}
