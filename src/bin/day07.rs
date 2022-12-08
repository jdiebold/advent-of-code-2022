use std::{collections::HashMap, fs};

fn main() {
    part_a();
    part_b();
}

fn part_a() {
    let input = fs::read_to_string("inputs/day07/input.txt").unwrap();
    let mut directories = HashMap::new();
    let mut pos = Vec::new();
    for s in input.split("$ cd ").skip(1) {
        if s != "..\n" {
            let mut l = s.lines();
            pos.push(l.next().unwrap());
            directories.insert(pos.concat(), l.skip(1).collect::<Vec<_>>());
        } else {
            pos.pop();
        }
    }
    let mut sum: i64 = 0;
    calc_dir_size(&mut sum, "/", &directories);
    println!("Part A: {}", sum);
}

fn calc_dir_size(sum: &mut i64, dir_name: &str, directories: &HashMap<String, Vec<&str>>) -> i64 {
    let mut dir_size = 0;
    for obj in directories.get(dir_name).unwrap() {
        if obj.starts_with("dir") {
            dir_size += calc_dir_size(
                sum,
                &(dir_name.to_owned() + obj.split_ascii_whitespace().last().unwrap()),
                &directories,
            );
        } else {
            dir_size += obj
                .split_ascii_whitespace()
                .next()
                .unwrap()
                .parse::<i64>()
                .unwrap();
        }
    }
    if dir_size <= 100000 {
        *sum += dir_size;
    }
    dir_size
}

fn part_b() {
    let input = fs::read_to_string("inputs/day07/input.txt").unwrap();
    let mut directories = HashMap::new();
    let mut sizes = Vec::new();
    let mut pos = Vec::new();
    for s in input.split("$ cd ").skip(1) {
        if s != "..\n" {
            let mut l = s.lines();
            pos.push(l.next().unwrap());
            directories.insert(pos.concat(), l.skip(1).collect::<Vec<_>>());
        } else {
            pos.pop();
        }
    }
    let rootsize = calc_dir_size_b(&mut sizes, "/", &directories);
    let needed_size = 30000000 - (70000000 - rootsize);
    let result = sizes.iter().filter(|x| x >= &&needed_size).min().unwrap();
    println!("Part B: {:?}", result);
}

fn calc_dir_size_b(
    sizes: &mut Vec<i64>,
    dir_name: &str,
    directories: &HashMap<String, Vec<&str>>,
) -> i64 {
    let mut dir_size = 0;
    for obj in directories.get(dir_name).unwrap() {
        if obj.starts_with("dir") {
            dir_size += calc_dir_size_b(
                sizes,
                &(dir_name.to_owned() + obj.split_ascii_whitespace().last().unwrap()),
                &directories,
            );
        } else {
            dir_size += obj
                .split_ascii_whitespace()
                .next()
                .unwrap()
                .parse::<i64>()
                .unwrap();
        }
    }
    sizes.push(dir_size);
    dir_size
}
