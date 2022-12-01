use std::fs;

fn main() {
    part_a();
    part_b();
}

fn part_a() {
    let input = fs::read_to_string("inputs/day01/input.txt").unwrap();
    let elves = input.split("\n\n");
    let max_elf = elves
        .into_iter()
        .map(|x| x.lines().map(|x| x.parse::<i64>().unwrap()).sum::<i64>())
        .max()
        .unwrap();
    println!("{}", max_elf)
}

fn part_b() {
    let input = fs::read_to_string("inputs/day01/input.txt").unwrap();
    let elves = input.split("\n\n");
    let mut max_elf: Vec<i64> = elves
        .into_iter()
        .map(|x| x.lines().map(|x| x.parse::<i64>().unwrap()).sum::<i64>())
        .collect();
    max_elf.sort();
    max_elf.reverse();
    let top_3_elves = &max_elf[0..3];
    let result: i64 = top_3_elves.iter().sum();
    println!("{}", result)
}
