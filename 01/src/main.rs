struct Instruction {
    direction: char,
    steps: usize,
}

fn parse_input(input: &str) -> (Vec<&str>, Vec<Instruction>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let names = parts[0].split(',').collect();
    let instructions = parts[1]
        .split(',')
        .map(|str| Instruction {
            direction: str.chars().next().unwrap(),
            steps: str[1..].parse().unwrap(),
        })
        .collect();
    (names, instructions)
}

fn part1(input: &str) -> &str {
    let (names, instructions) = parse_input(input);
    let mut current_position = 0usize;
    let n = names.len() - 1;
    for instruction in instructions {
        current_position = match instruction.direction {
            'L' => current_position.saturating_sub(instruction.steps),
            'R' => (current_position + instruction.steps).min(n),
            _ => panic!("Invalid direction"),
        }
    }
    names[current_position]
}

fn part2(input: &str) -> &str {
    let (names, instructions) = parse_input(input);
    let mut current_position = 0usize;
    let n = names.len();
    for instruction in instructions {
        current_position = match instruction.direction {
            'L' => {
                let current_position_i32 = i32::try_from(current_position).unwrap();
                let steps_i32 = i32::try_from(instruction.steps).unwrap();
                let length_i32 = i32::try_from(n).unwrap();
                let new_position_i32 = current_position_i32 - steps_i32;
                new_position_i32.rem_euclid(length_i32) as usize
            }
            'R' => (current_position + instruction.steps).rem_euclid(n),
            _ => panic!("Invalid direction"),
        };
    }
    names[current_position]
}

fn part3(input: &str) -> &str {
    let (mut names, instructions) = parse_input(input);
    let n = names.len();
    for instruction in instructions {
        let swap_position = match instruction.direction {
            'L' => {
                let steps_i32 = 0 - i32::try_from(instruction.steps).unwrap();
                let length_i32 = i32::try_from(n).unwrap();
                steps_i32.rem_euclid(length_i32) as usize
            }
            'R' => instruction.steps.rem_euclid(n),
            _ => panic!("Invalid direction"),
        };
        names.swap(0, swap_position);
    }
    names[0]
}

fn main() {
    let sample_input_01 = helpers::sample_file!("01");
    let sample_answer_01 = part1(&sample_input_01);
    assert_eq!(sample_answer_01, "Fyrryn");
    let input_01 = helpers::input_file!("01");
    let answer_01 = part1(&input_01);
    println!("Answer for part 1: {answer_01}");

    let sample_input_02 = helpers::sample_file!("02");
    let sample_answer_02 = part2(&sample_input_02);
    assert_eq!(sample_answer_02, "Elarzris");
    let input_02 = helpers::input_file!("02");
    let answer_02 = part2(&input_02);
    println!("Answer for part 2: {answer_02}");

    let sample_input_03 = helpers::sample_file!("03");
    let sample_answer_03 = part3(&sample_input_03);
    assert_eq!(sample_answer_03, "Drakzyph");
    let input_03 = helpers::input_file!("03");
    let answer_03 = part3(&input_03);
    println!("Answer for part 3: {answer_03}");
}
