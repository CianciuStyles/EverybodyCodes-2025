use regex::Regex;

type Complex = (i64, i64);

fn parse_input(input: &str) -> Complex {
    let regex = Regex::new(r"^\w+=\[(-?\d+),(-?\d+)]$").unwrap();
    let captures = regex.captures(input).unwrap();

    let x = captures[1].parse().unwrap();
    let y = captures[2].parse().unwrap();
    (x, y)
}

fn add(a: Complex, b: Complex) -> Complex {
    (a.0 + b.0, a.1 + b.1)
}

fn mul(a: Complex, b: Complex) -> Complex {
    (a.0 * b.0 - a.1 * b.1, a.0 * b.1 + a.1 * b.0)
}

fn div(a: Complex, b: Complex) -> Complex {
    (a.0 / b.0, a.1 / b.1)
}

fn part1(input: &str) -> String {
    let mut r = (0, 0);
    let a = parse_input(input);

    for _ in 0..3 {
        r = mul(r, r);
        r = div(r, (10, 10));
        r = add(r, a);
    }

    format!("[{},{}]", r.0, r.1)
}

fn check_point(p: Complex) -> bool {
    let mut r = (0, 0);

    for _ in 0..100 {
        r = mul(r, r);
        r = div(r, (100_000, 100_000));
        r = add(r, p);

        if r.0.abs() > 1_000_000 || r.1.abs() > 1_000_000 {
            return false;
        }
    }
    true
}

fn count_engraved_points(top_left: Complex, bottom_right: Complex, step: usize) -> usize {
    (top_left.0..=bottom_right.0)
        .step_by(step)
        .map(|x| {
            (top_left.1..=bottom_right.1)
                .step_by(step)
                .map(|y| (x, y))
                .filter(|&p| check_point(p))
                .count()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let a = parse_input(input);
    let b = add(a, (1000, 1000));

    count_engraved_points(a, b, 10)
}

fn part3(input: &str) -> usize {
    let a = parse_input(input);
    let b = add(a, (1000, 1000));

    count_engraved_points(a, b, 1)
}

fn main() {
    let sample_input_01 = helpers::sample_file!("01");
    let sample_answer_01 = part1(&sample_input_01);
    assert_eq!(sample_answer_01, "[357,862]");
    let input_01 = helpers::input_file!("01");
    let answer_01 = part1(&input_01);
    println!("Answer for part 1: {answer_01}");

    let sample_input_02 = helpers::sample_file!("02");
    let sample_answer_02 = part2(&sample_input_02);
    assert_eq!(sample_answer_02, 4076);
    let input_02 = helpers::input_file!("02");
    let answer_02 = part2(&input_02);
    println!("Answer for part 2: {answer_02}");

    let sample_input_03 = helpers::sample_file!("03");
    let sample_answer_03 = part3(&sample_input_03);
    assert_eq!(sample_answer_03, 406_954);
    let input_03 = helpers::input_file!("03");
    let answer_03 = part3(&input_03);
    println!("Answer for part 3: {answer_03}");
}
