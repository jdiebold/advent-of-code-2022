use std::fs;

fn main() {
    part_a();
    part_b();
}

fn part_a() {
    let rucksacks = fs::read_to_string("inputs/day03/input.txt").unwrap();
    let results: i64 = rucksacks
        .lines()
        .map(|rucksack| TryInto::<i64>::try_into(find_item_in_both(rucksack)).unwrap())
        .sum();
    println!("Part A: {:?}", results);
}

fn find_item_in_both(rucksack: &str) -> u8 {
    let len = rucksack.len();
    let compartments = rucksack.split_at(len / 2);
    for item in compartments.0.as_bytes() {
        if compartments.1.as_bytes().contains(item) {
            if item.is_ascii_uppercase() {
                return item - 38;
            } else {
                return item - 96;
            }
        }
    }
    0
}

fn part_b() {
    let input = fs::read_to_string("inputs/day03/input.txt").unwrap();
    let elves: Vec<&str> = input.lines().collect();
    let mut result: i64 = 0;
    for group in elves.chunks(3) {
        let item = group[0]
            .as_bytes()
            .into_iter()
            .filter(|x| group[1].as_bytes().contains(x))
            .filter(|x| group[2].as_bytes().contains(x))
            .collect::<Vec<_>>()[0];
        if item.is_ascii_uppercase() {
            result += TryInto::<i64>::try_into(*item).unwrap() - 38;
        } else {
            result += TryInto::<i64>::try_into(*item).unwrap() - 96;
        }
    }
    println!("Part B: {}", result)
}
