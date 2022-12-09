use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    part_a();
    part_b();
}

fn part_a() {
    let binding = fs::read_to_string("inputs/day09/input.txt").unwrap();
    let moves = binding.lines();
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    visited_positions.insert((0, 0));
    let directions = HashMap::from([("U", (0, 1)), ("D", (0, -1)), ("L", (-1, 0)), ("R", (1, 0))]);
    let mut h_pos = (0, 0);
    let mut t_pos = (0, 0);
    for m in moves {
        let cmd = m.split_ascii_whitespace().collect::<Vec<_>>();
        let direction = directions.get(cmd[0]).unwrap();
        let no_of_steps: i32 = cmd[1].parse().unwrap();
        for _step in 0..no_of_steps {
            h_pos = (h_pos.0 + direction.0, h_pos.1 + direction.1);
            let diff: (i32, i32) = (h_pos.0 - t_pos.0, h_pos.1 - t_pos.1);
            if diff.0.abs() > 1 || diff.1.abs() > 1 {
                t_pos = (t_pos.0 + diff.0.signum(), t_pos.1 + diff.1.signum());
            }
            visited_positions.insert(t_pos);
        }
    }
    println!("Part A: {}", visited_positions.len());
}

fn part_b() {
    let binding = fs::read_to_string("inputs/day09/input.txt").unwrap();
    let moves = binding.lines();
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    visited_positions.insert((0, 0));
    let directions = HashMap::from([("U", (0, 1)), ("D", (0, -1)), ("L", (-1, 0)), ("R", (1, 0))]);
    let mut rope = vec![(0, 0); 10];
    for m in moves {
        let cmd = m.split_ascii_whitespace().collect::<Vec<_>>();
        let direction = directions.get(cmd[0]).unwrap();
        let no_of_steps: i32 = cmd[1].parse().unwrap();
        for _step in 0..no_of_steps {
            rope[0] = (rope[0].0 + direction.0, rope[0].1 + direction.1);
            for knot in 1..rope.len() {
                let diff: (i32, i32) = (
                    rope[knot - 1].0 - rope[knot].0,
                    rope[knot - 1].1 - rope[knot].1,
                );
                if diff.0.abs() > 1 || diff.1.abs() > 1 {
                    rope[knot] = (
                        rope[knot].0 + diff.0.signum(),
                        rope[knot].1 + diff.1.signum(),
                    );
                }
            }
            visited_positions.insert(rope[9]);
        }
    }
    println!("Part B: {}", visited_positions.len());
}
