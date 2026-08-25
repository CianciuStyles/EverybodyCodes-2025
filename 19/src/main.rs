use std::collections::{BTreeMap, HashMap};

type Passages = BTreeMap<i32, Vec<(i32, i32)>>;

fn parse_input(text: &str) -> Passages {
    let mut passages: Passages = BTreeMap::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Parse "x,y,length" into integers cleanly
        let coords: Vec<i32> = trimmed
            .split(',')
            .map(|num| num.parse::<i32>().expect("Invalid number in input"))
            .collect();

        if coords.len() == 3 {
            let (x, y, length) = (coords[0], coords[1], coords[2]);
            passages.entry(x).or_default().push((y, y + length));
        }
    }

    passages
}

fn find_path(passages: &Passages) -> i32 {
    // Collect sorted unique X coordinates directly from our sorted BTreeMap keys
    let unique_xs: Vec<&i32> = passages.keys().collect();
    if unique_xs.is_empty() {
        return 0;
    }

    // Map tracking reachable states { y_coordinate: num_flaps }
    let mut reachable: HashMap<i32, i32> = HashMap::new();
    reachable.insert(0, 0);

    // Prepend 0 to our x coordinates iteration to represent the starting x1 step
    let mut x1 = 0;

    for &&x2 in &unique_xs {
        let mut next_reachable: HashMap<i32, i32> = HashMap::new();
        let dx = x2 - x1;

        // If a specific x2 column isn't in our map data, skip it safely
        if let Some(intervals) = passages.get(&x2) {
            for &(start, end) in intervals {
                for y2 in start..end {
                    let required_y1_parity = (dx + y2).rem_euclid(2);
                    let mut best_total_flaps = i32::MAX;

                    for (&y1, &num_flaps) in &reachable {
                        if y1.rem_euclid(2) != required_y1_parity {
                            continue;
                        }
                        if (y2 - y1).abs() > dx {
                            continue;
                        }

                        let total_flaps = num_flaps + ((dx + (y2 - y1)) / 2);
                        if total_flaps < best_total_flaps {
                            best_total_flaps = total_flaps;
                        }
                    }

                    if best_total_flaps != i32::MAX {
                        // Keep the sequential overwrite pattern matching the target puzzle framework
                        next_reachable.insert(y2, best_total_flaps);
                    }
                }
            }
        }

        reachable = next_reachable;
        x1 = x2; // Step forward x1 to become the current column marker
    }

    reachable.values().copied().min().unwrap_or(0)
}

fn part1(input: &str) -> i32 {
    let passages = parse_input(input);
    find_path(&passages)
}

fn part2(input: &str) -> i32 {
    let passages = parse_input(input);
    find_path(&passages)
}

fn part3(input: &str) -> i32 {
    let passages = parse_input(input);
    find_path(&passages)
}

fn main() {
    let sample_input_01 = helpers::sample_file!("01");
    let sample_answer_01 = part1(&sample_input_01);
    assert_eq!(sample_answer_01, 24);
    let input_01 = helpers::input_file!("01");
    let answer_01 = part1(&input_01);
    println!("Answer for part 1: {answer_01}");

    let sample_input_02 = helpers::sample_file!("02");
    let sample_answer_02 = part2(&sample_input_02);
    assert_eq!(sample_answer_02, 22);
    let input_02 = helpers::input_file!("02");
    let answer_02 = part2(&input_02);
    println!("Answer for part 2: {answer_02}");

    let input_03 = helpers::input_file!("03");
    let answer_03 = part3(&input_03);
    println!("Answer for part 3: {answer_03}");
}
