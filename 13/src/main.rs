fn part1(input: &str) -> u32 {
    let numbers: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut dial = vec![1];

    for num in numbers.iter().step_by(2) {
        dial.push(*num);
    }

    let end_index = if numbers.len() % 2 == 0 { 0 } else { 1 };
    for num in numbers.iter().rev().skip(end_index).step_by(2) {
        dial.push(*num);
    }

    dial[2025 % dial.len()]
}

fn part2(input: &str) -> u32 {
    let mut cw_elements = Vec::new();
    let mut ccw_elements = Vec::new();
    let mut is_clockwise = true;

    for line in input.lines() {
        if let Some((start_str, end_str)) = line.split_once('-') {
            let start: u32 = start_str.parse().unwrap();
            let end: u32 = end_str.parse().unwrap();
            let range = start..=end;

            if is_clockwise {
                cw_elements.extend(range);
            } else {
                ccw_elements.extend(range);
            }
            is_clockwise = !is_clockwise;
        }
    }

    let mut dial = vec![1];
    dial.extend(cw_elements);
    dial.extend(ccw_elements.into_iter().rev());

    dial[20252025 % dial.len()]
}

fn part3(input: &str) -> u64 {
    let mut cw_elements = Vec::new();
    let mut ccw_elements = Vec::new();
    let mut is_clockwise = true;
    let mut num_elements = 1u64;

    for line in input.lines() {
        if let Some((start_str, end_str)) = line.split_once('-') {
            let start: u64 = start_str.parse().unwrap();
            let end: u64 = end_str.parse().unwrap();
            let interval_len = end - start + 1;
            num_elements += interval_len;

            if is_clockwise {
                cw_elements.push((interval_len, start, end))
            } else {
                ccw_elements.push((interval_len, end, start))
            }
            is_clockwise = !is_clockwise;
        }
    }

    let mut dial = vec![(1, 1, 1)];
    dial.extend(cw_elements);
    dial.extend(ccw_elements.iter().rev());

    let mut index = 202520252025 % num_elements;

    for (interval_len, start, end) in dial {
        if index < interval_len {
            // We are in the right interval to expand
            return if start <= end {
                start + index
            } else {
                start - index
            }
        } else {
            index -= interval_len
        }
    }

    unreachable!()
}

fn main() {
    let sample_input_01 = helpers::sample_file!("01");
    let sample_answer_01 = part1(&sample_input_01);
    assert_eq!(sample_answer_01, 67);
    let input_01 = helpers::input_file!("01");
    let answer_01 = part1(&input_01);
    println!("Answer for part 1: {answer_01}");

    let sample_input_02 = helpers::sample_file!("02");
    let sample_answer_02 = part2(&sample_input_02);
    assert_eq!(sample_answer_02, 30);
    let input_02 = helpers::input_file!("02");
    let answer_02 = part2(&input_02);
    println!("Answer for part 2: {answer_02}");

    let input_03 = helpers::input_file!("03");
    let answer_03 = part3(&input_03);
    println!("Answer for part 3: {answer_03}");
}
