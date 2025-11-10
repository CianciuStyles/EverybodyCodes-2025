type Sword = (u64, Vec<u64>);
type Input = Vec<Sword>;
type Level = (Option<u64>, Option<u64>, Option<u64>);
type Quality = (u64, Vec<u32>);

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (id, numbers) = line.split_once(':').unwrap();
            (
                id.parse().unwrap(),
                numbers.split(',').map(|x| x.parse().unwrap()).collect(),
            )
        })
        .collect()
}

fn calculate_quality(numbers: &[u64]) -> Quality {
    let mut fishbone: Vec<Level> = Vec::new();

    for number in numbers {
        let mut new_level_required = true;
        for level in &mut fishbone {
            let mid = &level.1.unwrap();
            if number < mid && level.0.is_none() {
                level.0 = Some(*number);
                new_level_required = false;
                break;
            } else if number > mid && level.2.is_none() {
                level.2 = Some(*number);
                new_level_required = false;
                break;
            }
        }
        if new_level_required {
            fishbone.push((None, Some(*number), None));
        }
    }

    (
        fishbone
            .iter()
            .map(|x| x.1.unwrap().to_string())
            .collect::<String>()
            .parse()
            .unwrap(),
        fishbone
            .iter()
            .map(|level| {
                format!(
                    "{}{}{}",
                    level.0.map(|x| x.to_string()).unwrap_or_default(),
                    level.1.map(|x| x.to_string()).unwrap_or_default(),
                    level.2.map(|x| x.to_string()).unwrap_or_default(),
                )
                .parse()
                .unwrap()
            })
            .collect(),
    )
}

fn part1(input: &str) -> u64 {
    let (_, numbers) = parse_input(input).into_iter().next().unwrap();
    calculate_quality(&numbers).0
}

fn part2(input: &str) -> u64 {
    let qualities: Vec<u64> = parse_input(input)
        .iter()
        .map(|(_, numbers)| calculate_quality(numbers).0)
        .collect();
    qualities.iter().max().unwrap() - qualities.iter().min().unwrap()
}

fn part3(input: &str) -> u64 {
    let mut qualities = parse_input(input);
    qualities.sort_by(|sword_1, sword_2| {
        let quality1 = calculate_quality(&sword_1.1);
        let quality2 = calculate_quality(&sword_2.1);

        match quality1.0.cmp(&quality2.0) {
            std::cmp::Ordering::Equal => match quality1.1.cmp(&quality2.1) {
                std::cmp::Ordering::Equal => sword_1.0.cmp(&sword_2.0),
                cmp => cmp,
            },
            cmp => cmp,
        }
    });
    qualities.reverse();

    qualities
        .iter()
        .enumerate()
        .map(|(index, (id, _))| (index + 1) as u64 * id)
        .sum()
}

fn main() {
    let sample_input_01 = helpers::sample_file!("01");
    let sample_answer_01 = part1(&sample_input_01);
    assert_eq!(sample_answer_01, 581_078);
    let input_01 = helpers::input_file!("01");
    let answer_01 = part1(&input_01);
    println!("Answer for part 1: {answer_01}");

    let sample_input_02 = helpers::sample_file!("02");
    let sample_answer_02 = part2(&sample_input_02);
    assert_eq!(sample_answer_02, 77_053);
    let input_02 = helpers::input_file!("02");
    let answer_02 = part2(&input_02);
    println!("Answer for part 2: {answer_02}");

    let sample_input_03 = helpers::sample_file!("03");
    let sample_answer_03 = part3(&sample_input_03);
    assert_eq!(sample_answer_03, 260);
    let input_03 = helpers::input_file!("03");
    let answer_03 = part3(&input_03);
    println!("Answer for part 3: {answer_03}");
}
