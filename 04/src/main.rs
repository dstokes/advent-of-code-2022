use std::fs;
use std::collections::HashSet;

fn main() {
    let input =
        fs::read_to_string("input.txt").unwrap();

    let mut part1 = 0;
    let mut part2 = 0;

    for line in input.lines() {
        let bounds: Vec<u8> = line
            // [start, end, start, end]
            .split(&[',','-'])
            // coerce strings to ints
            .map(|x| x.parse().unwrap())
            // convert to Vec
            .collect();

        // create hashsets from ranges [start ... end]
        let range1: HashSet<u8> =
            HashSet::from_iter(bounds[0]..=bounds[1]);
        let range2: HashSet<u8> =
            HashSet::from_iter(bounds[2]..=bounds[3]);

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
