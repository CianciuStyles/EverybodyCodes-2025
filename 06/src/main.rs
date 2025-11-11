use std::cmp::min;

fn count_pairs(people: &[char], pairs: &[(char, char)]) -> u32 {
    let mut count = 0;

    for (mentor, novice) in pairs {
        let mut num_mentors = 0;
        for person in people {
            if person == novice {
                count += num_mentors
            } else if person == mentor {
                num_mentors += 1
            }
        }
    }

    count
}

fn part1(input: &str) -> u32 {
    let people: Vec<char> = input.chars().collect();
    count_pairs(&people, &[('A', 'a')])
}

fn part2(input: &str) -> u32 {
    let people: Vec<char> = input.chars().collect();
    count_pairs(&people, &[('A', 'a'), ('B', 'b'), ('C', 'c')])
}

fn search_mentors(people: &[char], start_index: usize, distance_limit: usize, mentor: char) -> u32 {
    let before_lower_bound = start_index.saturating_sub(distance_limit);
    let before = &people[before_lower_bound..start_index];
    let before_count = u32::try_from(before.iter().filter(|&&x| x == mentor).count()).unwrap_or(0);

    let after_upper_bound = min(people.len(), start_index + distance_limit + 1);
    let after = &people[(start_index + 1)..after_upper_bound];
    let after_count = u32::try_from(after.iter().filter(|&&x| x == mentor).count()).unwrap_or(0);

    before_count + after_count
}

fn part3(input: &str, distance_limit: usize, pattern_repeats: usize) -> u32 {
    let people = input.chars().collect::<Vec<char>>().repeat(pattern_repeats);
    let mut count = 0;
    for (index, person) in people.iter().enumerate() {
        match person {
            'a' => count += search_mentors(&people, index, distance_limit, 'A'),
            'b' => count += search_mentors(&people, index, distance_limit, 'B'),
            'c' => count += search_mentors(&people, index, distance_limit, 'C'),
            _ => {}
        }
    }
    count
}

fn main() {
    let sample_input_01 = helpers::sample_file!("01");
    let sample_answer_01 = part1(&sample_input_01);
    assert_eq!(sample_answer_01, 5);
    let input_01 = helpers::input_file!("01");
    let answer_01 = part1(&input_01);
    println!("Answer for part 1: {answer_01}");

    let sample_input_02 = helpers::sample_file!("02");
    let sample_answer_02 = part2(&sample_input_02);
    assert_eq!(sample_answer_02, 11);
    let input_02 = helpers::input_file!("02");
    let answer_02 = part2(&input_02);
    println!("Answer for part 2: {answer_02}");

    let sample_input_03 = helpers::sample_file!("03");
    let sample_answer_03_1 = part3(&sample_input_03, 10, 1);
    assert_eq!(sample_answer_03_1, 34);
    let sample_answer_03_2 = part3(&sample_input_03, 10, 2);
    assert_eq!(sample_answer_03_2, 72);
    let sample_answer_03_3 = part3(&sample_input_03, 1000, 1000);
    assert_eq!(sample_answer_03_3, 3_442_321);
    let input_03 = helpers::input_file!("03");
    let answer_03 = part3(&input_03, 1000, 1000);
    println!("Answer for part 3: {answer_03}");
}
