use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<u32> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

fn part1(input: &str) -> u32 {
    let mut crates = parse_input(input);
    crates.sort_unstable();
    crates.dedup();
    crates.iter().sum()
}

fn part2(input: &str) -> u32 {
    let mut crates = parse_input(input);
    crates.sort_unstable();
    crates.dedup();
    crates.iter().take(20).sum()
}

fn part3(input: &str) -> u32 {
    let crates = parse_input(input);
    let mut counter = HashMap::new();
    for c in crates {
        *counter.entry(c).or_insert(0) += 1;
    }
    *counter.values().max().unwrap()
}

fn main() {
    let sample_input_01 = helpers::sample_file!("01");
    let sample_answer_01 = part1(&sample_input_01);
    assert_eq!(sample_answer_01, 29);
    let input_01 = helpers::input_file!("01");
    let answer_01 = part1(&input_01);
    println!("Answer for part 1: {answer_01}");

    let sample_input_02 = helpers::sample_file!("02");
    let sample_answer_02 = part2(&sample_input_02);
    assert_eq!(sample_answer_02, 781);
    let input_02 = helpers::input_file!("02");
    let answer_02 = part2(&input_02);
    println!("Answer for part 2: {answer_02}");

    let sample_input_03 = helpers::sample_file!("03");
    let sample_answer_03 = part3(&sample_input_03);
    assert_eq!(sample_answer_03, 3);
    let input_03 = helpers::input_file!("03");
    let answer_03 = part3(&input_03);
    println!("Answer for part 3: {answer_03}");
}
