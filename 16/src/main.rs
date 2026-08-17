type UInt = u64;

fn parse_input(text: &str) -> Vec<UInt> {
    text.split(',')
        .filter_map(|num| num.trim().parse::<u64>().ok())
        .collect()
}

fn part1(input: &str) -> UInt {
    let plan = parse_input(input);
    plan.iter().map(|&num| 90 / num).sum()
}

fn find_plan(mut wall: Vec<u64>) -> Vec<u64> {
    let mut plan = Vec::new();

    for i in 0..wall.len() {
        if wall[i] > 0 {
            let curr_step = (i + 1) as u64;
            let count = wall[i];

            // Append the step 'count' times to the plan
            plan.extend(std::iter::repeat(curr_step).take(count as usize));

            // Stride through the remaining vector and subtract the count
            for j in (i..wall.len()).step_by(curr_step as usize) {
                wall[j] = wall[j].saturating_sub(count);
            }
        }
    }
    plan
}

fn part2(input: &str) -> UInt {
    let plan = find_plan(parse_input(input));
    plan.iter().product()
}


fn part3(input: &str) -> u64 {
    let plan = find_plan(parse_input(input));
    let num_blocks = 202520252025000;

    let mut low = 0;
    let mut high = num_blocks;
    let mut result = 0;

    while low <= high {
        let mid = low + (high - low) / 2; // Prevents theoretical overflow compared to (low + high) / 2

        let total_blocks_used: u64 = plan.iter().map(|&num| mid / num).sum();

        if total_blocks_used <= num_blocks {
            result = mid;
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    result
}

fn main() {
    let sample_input_01 = helpers::sample_file!("01");
    let sample_answer_01 = part1(&sample_input_01);
    assert_eq!(sample_answer_01, 193);
    let input_01 = helpers::input_file!("01");
    let answer_01 = part1(&input_01);
    println!("Answer for part 1: {answer_01}");

    let sample_input_02 = helpers::sample_file!("02");
    let sample_answer_02 = part2(&sample_input_02);
    assert_eq!(sample_answer_02, 270);
    let input_02 = helpers::input_file!("02");
    let answer_02 = part2(&input_02);
    println!("Answer for part 2: {answer_02}");

    let sample_input_03 = helpers::sample_file!("03");
    let sample_answer_03 = part3(&sample_input_03);
    assert_eq!(sample_answer_03, 94439495762954);
    let input_03 = helpers::input_file!("03");
    let answer_03 = part3(&input_03);
    println!("Answer for part 3: {answer_03}");
}
