use std::fs;

use regex::Regex;

fn main() {
    part_a();
    part_b();
}

fn part_a() {
    let inputs = fs::read_to_string("inputs/day05/input.txt").unwrap();
    let (stacks_input, moves) = inputs.split_once("\n\n").unwrap();
    let stacks_array: Vec<&[u8]> = stacks_input.lines().map(|x| x.as_bytes()).collect();
    let mut x = 1;
    let mut stacks: Vec<Vec<u8>> = Vec::new();
    // parse strings into stacks
    while x < stacks_array.last().unwrap().len() {
        let mut next = Vec::<u8>::new();
        for line in (0..(stacks_array.len() - 1)).rev() {
            if stacks_array[line].get(x).unwrap() != &b' ' {
                next.push(*stacks_array[line].get(x).unwrap());
            }
        }
        stacks.push(next);
        x += 4;
    }
    // make the moves
    let numbers = Regex::new(r"\d+").unwrap();
    for next_move in moves.lines() {
        let move_nrs: Vec<usize> = numbers
            .find_iter(next_move)
            .map(|x| x.as_str().parse().unwrap())
            .collect();
        for _count in 1..=move_nrs[0] {
            let intermediate = stacks[move_nrs[1] - 1].pop().unwrap();
            stacks[move_nrs[2] - 1].push(intermediate);
        }
    }
    let mut result: Vec<u8> = Vec::new();
    for mut stack in stacks {
        result.push(stack.pop().unwrap());
    }
    println!("Part A: {}", std::str::from_utf8(&result).unwrap());
}

fn part_b() {
    let inputs = fs::read_to_string("inputs/day05/input.txt").unwrap();
    let (stacks_input, moves) = inputs.split_once("\n\n").unwrap();
    let stacks_array: Vec<&[u8]> = stacks_input.lines().map(|x| x.as_bytes()).collect();
    let mut x = 1;
    let mut stacks: Vec<Vec<u8>> = Vec::new();
    // parse strings into stacks
    while x < stacks_array.last().unwrap().len() {
        let mut next = Vec::<u8>::new();
        for line in (0..(stacks_array.len() - 1)).rev() {
            if stacks_array[line].get(x).unwrap() != &b' ' {
                next.push(*stacks_array[line].get(x).unwrap());
            }
        }
        stacks.push(next);
        x += 4;
    }
    // make the moves
    let numbers = Regex::new(r"\d+").unwrap();
    for next_move in moves.lines() {
        let move_nrs: Vec<usize> = numbers
            .find_iter(next_move)
            .map(|x| x.as_str().parse().unwrap())
            .collect();
        let mut intermediate: Vec<u8> = Vec::new();
        for _count in 1..=move_nrs[0] {
            intermediate.push(stacks[move_nrs[1] - 1].pop().unwrap());
        }
        for _count in 1..=move_nrs[0] {
            stacks[move_nrs[2] - 1].push(intermediate.pop().unwrap());
        }
    }
    let mut result: Vec<u8> = Vec::new();
    for mut stack in stacks {
        result.push(stack.pop().unwrap());
    }
    println!("Part B: {}", std::str::from_utf8(&result).unwrap());
}
