use std::fs;

#[derive(Debug)]
struct IntIntervalModel {
    start: u64,
    end: u64,
}

fn main() {
    let input = fs::read_to_string("src/puzzle_input.txt").unwrap();
    let mut intervals: Vec<IntIntervalModel> = vec![];
    let mut counter = 0;

    for line in input.lines() {
        if line.contains("-") {
            let line_split: Vec<&str> = line.split("-").collect();

            let int_model = IntIntervalModel {
                start: line_split[0].parse().unwrap(),
                end: line_split[1].parse().unwrap(),
            };

            intervals.push(int_model);
        } else {
            if !line.is_empty() {
                for int_val in &intervals {
                    let target_int: u64 = line.parse().unwrap();
                    if int_val.start <= target_int && target_int <= int_val.end {
                        counter += 1;
                        break;
                    }
                }
            }
        }
    }

    println!("{:?}", counter)
}
