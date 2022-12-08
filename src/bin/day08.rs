use std::fs;

fn main() {
    part_a();
    part_b();
}

fn part_a() {
    let binding = fs::read_to_string("inputs/day08/input.txt").unwrap();
    let trees: Vec<&[u8]> = binding.lines().map(|x| x.as_bytes()).collect();
    let mut sum = 0;
    for line in 0..trees.len() {
        for column in 0..trees[line].len() {
            if line == 0
                || column == 0
                || line == trees.len() - 1
                || column == trees[line].len() - 1
                || trees[line][..column]
                    .iter()
                    .all(|other| other < &trees[line][column])
                || trees[line][(column + 1)..]
                    .iter()
                    .all(|other| other < &trees[line][column])
                || trees[..line]
                    .iter()
                    .map(|lines| lines[column])
                    .all(|other| other < trees[line][column])
                || trees[(line + 1)..]
                    .iter()
                    .map(|lines| lines[column])
                    .all(|other| other < trees[line][column])
            {
                sum += 1;
            }
        }
    }
    println!("Part A: {}", sum);
}

fn part_b() {
    let binding = fs::read_to_string("inputs/day08/input.txt").unwrap();
    let trees: Vec<&[u8]> = binding.lines().map(|x| x.as_bytes()).collect();
    let mut max_score = 0;
    for line in 0..trees.len() {
        for column in 0..trees[line].len() {
            let left =
                find_no_of_trees(&mut trees[line][..column].iter().rev(), trees[line][column]);
            let right =
                find_no_of_trees(&mut trees[line][column + 1..].iter(), trees[line][column]);
            let up = find_no_of_trees(
                &mut trees[..line]
                    .iter()
                    .map(|lines| &lines[column])
                    .rev()
                    .into_iter(),
                trees[line][column],
            );
            let down = find_no_of_trees(
                &mut trees[line + 1..].iter().map(|lines| &lines[column]),
                trees[line][column],
            );
            if left * right * up * down > max_score {
                max_score = left * right * up * down;
            }
        }
    }
    println!("Part B: {}", max_score)
}

fn find_no_of_trees(trees_in_line: &mut dyn Iterator<Item = &u8>, tree: u8) -> usize {
    let size = trees_in_line.size_hint().1.unwrap_or_else(|| 0);
    trees_in_line
        .enumerate()
        .find(|other| other.1 >= &tree)
        .map(|x| x.0 + 1)
        .unwrap_or_else(|| size)
}
