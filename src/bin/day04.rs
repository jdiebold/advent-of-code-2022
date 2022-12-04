use std::fs;

fn main() {
    part_a();
    part_b();
}

fn part_a() {
    let input = fs::read_to_string("inputs/day04/input.txt").unwrap();
    let pairs = input.lines();
    let mut result = 0;
    for pair in pairs {
        let ranges: Vec<Vec<i64>> = pair
            .split(",")
            .map(|range| {
                range
                    .split("-")
                    .map(|digit| digit.parse::<i64>().unwrap())
                    .collect()
            })
            .collect();
        if range_contains((ranges[0][0], ranges[0][1]), (ranges[1][0], ranges[1][1]))
            || range_contains((ranges[1][0], ranges[1][1]), (ranges[0][0], ranges[0][1]))
        {
            result += 1;
        }
    }
    println!("Part A: {}", result);
}

fn range_contains(a: (i64, i64), b: (i64, i64)) -> bool {
    a.0 <= b.0 && a.1 >= b.1
}

fn part_b() {
    let input = fs::read_to_string("inputs/day04/input.txt").unwrap();
    let pairs = input.lines();
    let mut result = 0;
    for pair in pairs {
        let ranges: Vec<Vec<i64>> = pair
            .split(",")
            .map(|range| {
                range
                    .split("-")
                    .map(|digit| digit.parse::<i64>().unwrap())
                    .collect()
            })
            .collect();
        if (ranges[0][1] >= ranges[1][0] && ranges[0][1] <= ranges[1][1])
            || (ranges[1][1] >= ranges[0][0] && ranges[1][1] <= ranges[0][1])
        {
            result += 1;
        }
    }
    println!("Part B: {}", result);
}
