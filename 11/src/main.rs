type UInt = u64;

fn parse_input(input: &str) -> Vec<UInt> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn part1(input: &str) -> UInt {
    let mut ducks = parse_input(input);
    let n = ducks.len();
    let mut num_round = 0;
    let target_round = 10;

    // Phase 1: Left-to-Right
    while num_round < target_round {
        // Stop early if the array is sorted (non-decreasing)
        if ducks.windows(2).all(|w| w[0] <= w[1]) {
            break;
        }

        for i in 0..n - 1 {
            if ducks[i] > ducks[i + 1] {
                ducks[i] -= 1;
                ducks[i + 1] += 1;
            }
        }
        num_round += 1;
    }

    // Phase 2: Right-to-Left Flow
    while num_round < target_round {
        // Stop when all elements are perfectly uniform
        if ducks.iter().all(|&val| val == ducks[0]) {
            break;
        }

        for i in 0..n - 1 {
            if ducks[i] < ducks[i + 1] {
                ducks[i] += 1;
                ducks[i + 1] -= 1;
            }
        }
        num_round += 1;
    }

    ducks
        .iter()
        .enumerate()
        .map(|(i, &val)| val * (i as UInt + 1))
        .sum()
}

fn part2(input: &str) -> UInt {
    let mut ducks = parse_input(input);
    let n = ducks.len();
    let target = ducks.iter().sum::<UInt>() / n as UInt;

    let mut phase1_rounds = 0;

    // Phase 1: Left-to-right
    loop {
        let mut moved = false;
        for i in 0..n - 1 {
            if ducks[i] > ducks[i + 1] {
                ducks[i] -= 1;
                ducks[i + 1] += 1;
                moved = true;
            }
        }
        if !moved {
            break;
        }
        phase1_rounds += 1;
    }

    // Phase 2: Right-to-left
    let mut phase2_rounds = 0;
    let mut current_right_sum = 0;

    for i in (1..n).rev() {
        current_right_sum += ducks[i];
        let cols_counted = (n - i) as UInt;
        let required_right_sum = cols_counted * target;

        if current_right_sum > required_right_sum {
            let surplus = current_right_sum - required_right_sum;
            phase2_rounds = phase2_rounds.max(surplus);
        }
    }

    phase1_rounds + phase2_rounds
}

fn part3(input: &str) -> UInt {
    let ducks = parse_input(input);
    let n = ducks.len();
    let target = ducks.iter().sum::<UInt>() / n as UInt;

    let mut phase2_rounds = 0;
    let mut current_right_sum = 0;

    // Phase 2: Right-to-left only, as the input is
    // already sorted in non-decreasing order
    for i in (1..n).rev() {
        current_right_sum += ducks[i];
        let cols_counted = (n - i) as UInt;
        let required_right_sum = cols_counted * target;

        if current_right_sum > required_right_sum {
            let surplus = current_right_sum - required_right_sum;
            phase2_rounds = phase2_rounds.max(surplus);
        }
    }

    phase2_rounds
}

fn main() {
    let sample_input_01 = helpers::sample_file!("01");
    let sample_answer_01 = part1(&sample_input_01);
    assert_eq!(sample_answer_01, 109);
    let input_01 = helpers::input_file!("01");
    let answer_01 = part1(&input_01);
    println!("Answer for part 1: {answer_01}");

    let sample_input_02_1 = helpers::sample_file!("02_1");
    let sample_answer_02_1 = part2(&sample_input_02_1);
    assert_eq!(sample_answer_02_1, 11);
    let sample_input_02_2 = helpers::sample_file!("02_2");
    let sample_answer_02_2 = part2(&sample_input_02_2);
    assert_eq!(sample_answer_02_2, 1579);
    let input_02 = helpers::input_file!("02");
    let answer_02 = part2(&input_02);
    println!("Answer for part 2: {answer_02}");

    let input_03 = helpers::input_file!("03");
    let answer_03 = part3(&input_03);
    println!("Answer for part 3: {answer_03}");
}
