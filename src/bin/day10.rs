use std::fs;

fn main() {
    part_a();
    part_b();
}

fn part_a() {
    let input = fs::read_to_string("inputs/day10/input.txt").unwrap();
    let mut cmds = input.lines().flat_map(|line| {
        if line != "cmd" {
            line.split_ascii_whitespace().collect::<Vec<_>>()
        } else {
            vec![line]
        }
    });
    let mut x = 1;
    let mut cycle = 1;
    let mut sum: i64 = 0;
    while let Some(cmd) = cmds.next() {
        if cycle == 20 || (cycle - 20) % 40 == 0 {
            sum += cycle * x;
            println!("adding after {}: {}", cycle, cycle * x);
        }
        cycle += 1;
        x += cmd.parse().unwrap_or(0);
    }
    println!("Part A: {}", sum)
}

fn part_b() {
    let input = fs::read_to_string("inputs/day10/input.txt").unwrap();
    let mut cmds = input.lines().flat_map(|line| {
        if line != "cmd" {
            line.split_ascii_whitespace().collect::<Vec<_>>()
        } else {
            vec![line]
        }
    });
    let mut x: i32 = 1;
    let mut cycle: i32 = 0;
    while let Some(cmd) = cmds.next() {
        if (cycle % 40 - x).abs() <= 1 {
            print!("#");
        } else {
            print!(".");
        }
        cycle += 1;
        if cycle % 40 == 0 {
            print!("\n");
        }
        x += cmd.parse().unwrap_or(0);
    }
}
