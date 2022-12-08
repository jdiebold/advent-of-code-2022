use std::fs;

fn main() {
    part_a();
    part_b();
}

fn part_a() {
    let input = fs::read_to_string("inputs/day06/input.txt").unwrap();
    let lines = input.lines();
    for signal in lines {
        for i in 4..signal.as_bytes().len() {
            let s = &signal[i - 4..i];
            if unique(s) {
                println!("Part A: {}", i);
                break;
            };
        }
    }
}

fn unique(s: &str) -> bool {
    s.chars()
        .enumerate()
        .find_map(|(i, c)| {
            s.chars()
                .enumerate()
                .skip(i + 1)
                .find(|(_, other)| c == *other)
        })
        .is_none()
}

fn part_b() {
    let input = fs::read_to_string("inputs/day06/input.txt").unwrap();
    let lines = input.lines();
    for signal in lines {
        for i in 14..signal.as_bytes().len() {
            let s = &signal[i - 14..i];
            if unique(s) {
                println!("Part B: {}", i);
                break;
            };
        }
    }
}
