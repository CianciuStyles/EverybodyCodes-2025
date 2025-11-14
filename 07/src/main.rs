use std::collections::{HashMap, HashSet, VecDeque};

type Input<'a> = (Vec<&'a str>, Rules);
type Rules = HashMap<char, Name>;
type Name = Vec<char>;

fn parse_input(input: &str) -> Input<'_> {
    let (names_str, rules_str) = input.split_once("\n\n").unwrap();

    let names = names_str.trim().split(',').collect();
    let rules = rules_str
        .lines()
        .map(|line| {
            let (left, right) = line.trim().split_once(" > ").unwrap();
            let key = left.chars().next().unwrap();
            let value = right
                .split(',')
                .map(|c| c.chars().next().unwrap())
                .collect();
            (key, value)
        })
        .collect();

    (names, rules)
}

fn verify_name(name: &str, rules: &Rules) -> bool {
    !name
        .chars()
        .zip(name.chars().skip(1))
        .any(|(left, right)| !rules.get(&left).unwrap().contains(&right))
}

fn part1(input: &str) -> &str {
    let (names, rules) = parse_input(input);

    names
        .iter()
        .find(|&&name| verify_name(name, &rules))
        .unwrap()
}

fn part2(input: &str) -> usize {
    let (names, rules) = parse_input(input);
    names
        .iter()
        .enumerate()
        .filter(|(_, name)| verify_name(name, &rules))
        .map(|(index, _)| index + 1)
        .sum()
}

fn generate_names(prefix: &str, rules: &Rules) -> Vec<Name> {
    let mut names = vec![];

    if !verify_name(prefix, rules) {
        return names;
    }

    let valid_names_len_range = 7..=11;
    let mut queue: VecDeque<Name> = VecDeque::new();
    queue.push_back(prefix.chars().collect());
    while let Some(name) = queue.pop_front() {
        if let Some(suffixes) = rules.get(name.last().unwrap()) {
            for suffix in suffixes {
                let mut new_name = name.clone();
                new_name.push(*suffix);
                let new_name_len = name.len() + 1;

                if valid_names_len_range.contains(&new_name_len) {
                    names.push(new_name.clone());
                }
                if new_name_len <= 11 {
                    queue.push_back(new_name.clone());
                }
            }
        }
    }

    names
}

fn part3(input: &str) -> usize {
    let (prefixes, rules) = parse_input(input);
    prefixes
        .iter()
        .flat_map(|prefix| generate_names(prefix, &rules))
        .collect::<HashSet<Name>>()
        .len()
}

fn main() {
    let sample_input_01 = helpers::sample_file!("01");
    let sample_answer_01 = part1(&sample_input_01);
    assert_eq!(sample_answer_01, "Oroneth");
    let input_01 = helpers::input_file!("01");
    let answer_01 = part1(&input_01);
    println!("Answer for part 1: {answer_01}");

    let sample_input_02 = helpers::sample_file!("02");
    let sample_answer_02 = part2(&sample_input_02);
    assert_eq!(sample_answer_02, 23);
    let input_02 = helpers::input_file!("02");
    let answer_02 = part2(&input_02);
    println!("Answer for part 2: {answer_02}");

    let sample_input_03_1 = helpers::sample_file!("03_1");
    let sample_answer_03_1 = part3(&sample_input_03_1);
    assert_eq!(sample_answer_03_1, 25);
    let sample_input_03_2 = helpers::sample_file!("03_2");
    let sample_answer_03_2 = part3(&sample_input_03_2);
    assert_eq!(sample_answer_03_2, 1154);
    let input_03 = helpers::input_file!("03");
    let answer_03 = part3(&input_03);
    println!("Answer for part 3: {answer_03}");
}
