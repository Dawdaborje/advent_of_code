use std::fs;

fn main() {
    let input = fs::read_to_string("/home/borje/Documents/CODING/Personal/personal/advent_of_code/2025/day_2_gift_shop/src/puzzle_input.txt").unwrap();

    let mut pos = 50;
    let mut total_hits: i32 = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let dir = line.chars().next().unwrap();
        let dist: i32 = line[1..].parse().unwrap();

        match dir {
            'R' => {
                let hits = (pos + dist) / 100;
                total_hits += hits;
                pos = (pos + dist).rem_euclid(100);
            }
            'L' => {
                let first_click = if pos == 0 { 100 } else { pos };
                let hits = if dist < first_click {
                    0
                } else {
                    1 + (dist - first_click) / 100
                };
                total_hits += hits;
                pos = (pos - dist).rem_euclid(100)
            }
            _ => panic!("invalid input"),
        }
    }

    println!("{}", total_hits);
}
