fn parse_input1(input: &str) -> Vec<u64> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

fn part1(input: &str) -> u64 {
    let gears = parse_input1(input);
    let first = gears.first().unwrap();
    let last = gears.iter().last().unwrap();
    2025 * first / last
}

fn part2(input: &str) -> u64 {
    let gears = parse_input1(input);
    let first = gears.first().unwrap();
    let last = gears.iter().last().unwrap();
    (10_000_000_000_000 * last).div_ceil(*first)
}

fn parse_input2(input: &str) -> Vec<(u64, u64)> {
    input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            if let Some((first, second)) = line.split_once('|') {
                (first.parse().unwrap(), second.parse().unwrap())
            } else if index == 0 {
                (1, line.parse().unwrap())
            } else {
                (line.parse().unwrap(), 1)
            }
        })
        .collect()
}

fn part3(input: &str) -> u64 {
    let crates = parse_input2(input);
    crates.iter().fold(100, |acc, (numerator, denominator)| {
        acc * denominator / numerator
    })
}

fn main() {
    let sample_input_01 = helpers::sample_file!("01");
    let sample_answer_01 = part1(&sample_input_01);
    assert_eq!(sample_answer_01, 15888);
    let input_01 = helpers::input_file!("01");
    let answer_01 = part1(&input_01);
    println!("Answer for part 1: {answer_01}");

    let sample_input_02 = helpers::sample_file!("02");
    let sample_answer_02 = part2(&sample_input_02);
    assert_eq!(sample_answer_02, 625_000_000_000);
    let input_02 = helpers::input_file!("02");
    let answer_02 = part2(&input_02);
    println!("Answer for part 2: {answer_02}");

    let sample_input_03 = helpers::sample_file!("03");
    let sample_answer_03 = part3(&sample_input_03);
    assert_eq!(sample_answer_03, 6818);
    let input_03 = helpers::input_file!("03");
    let answer_03 = part3(&input_03);
    println!("Answer for part 3: {answer_03}");
}
