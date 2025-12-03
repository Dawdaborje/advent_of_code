use std::fs;

fn main() {
    let input = fs::read_to_string("/home/borje/Documents/CODING/Personal/personal/advent_of_code/2025/day_1_secret_entrance/src/puzzle_input.txt").unwrap();

    let mut pos = 50;
    let mut count = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let dir = line.chars().next().unwrap();
        let dist: i32 = line[1..].parse().unwrap();

        match dir {
            'R' => pos = (pos + dist) % 100,
            'L' => pos = (pos - dist) % (100),
            _ => panic!("invalid input"),
        }

        if pos == 0 {
            count += 1;
        }
    }

    println!("{}", count);
}
