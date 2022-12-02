use std::fs;

fn main() {
    part_a();
    part_b();
}

fn part_a() {
    let inputs: i64 = fs::read_to_string("inputs/day02/input.txt")
        .unwrap()
        .lines()
        .map(|x| calc_score(x.split(" ").map(|x| x.as_bytes()[0]).collect()))
        .sum();
    println!("Part A: {}", inputs);
}

fn calc_score(choices: Vec<u8>) -> i64 {
    let opponent = choices[0];
    let myself = choices[1] - 23;
    let mut score = 0;
    match myself {
        b'A' => {
            if opponent == b'C' {
                score += 6
            }
            score += 1;
        }
        b'B' => {
            if opponent == b'A' {
                score += 6
            }
            score += 2;
        }
        b'C' => {
            if opponent == b'B' {
                score += 6
            }
            score += 3;
        }
        _ => (),
    }
    if opponent == myself {
        score += 3
    }
    score
}

fn part_b() {
    let inputs: i64 = fs::read_to_string("inputs/day02/input.txt")
        .unwrap()
        .lines()
        .map(|x| calc_score_b(x.split(" ").map(|x| x.as_bytes()[0]).collect()))
        .sum();
    println!("Part B: {}", inputs);
}

fn calc_score_b(choices: Vec<u8>) -> i64 {
    let opponent = choices[0] - 64;
    let result = choices[1];
    let mut score: i64 = 0;
    match result {
        b'X' => match opponent {
            1 => score += 3,
            2 => score += 1,
            3 => score += 2,
            _ => (),
        }, // lose
        b'Y' => match opponent {
            1 => score += 1 + 3,
            2 => score += 2 + 3,
            3 => score += 3 + 3,
            _ => (), // draw + 3 points for draw
        },
        b'Z' => match opponent {
            1 => score += 2 + 6,
            2 => score += 3 + 6,
            3 => score += 1 + 6,
            _ => (), // win + 6 points for win
        },
        _ => (),
    }
    score
}
