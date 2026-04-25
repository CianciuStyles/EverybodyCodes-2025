type Input = Vec<usize>;

fn parse_input(input: &str) -> Input {
    input
        .split(",")
        .map(|digit| digit.parse().unwrap())
        .collect()
}

fn part1(num_nails: usize, input: &str) -> usize {
    let notes = parse_input(input);
    let mid = num_nails / 2;

    notes
        .windows(2)
        .filter(|nails| nails[0].abs_diff(nails[1]) == mid)
        .count()
}

fn part2(num_nails: usize, input: &str) -> usize {
    let notes = parse_input(input);
    let mut links = vec![Vec::new(); num_nails];
    for pair in notes.windows(2) {
        let start = pair[0].min(pair[1]);
        let end = pair[0].max(pair[1]);
        links[start].push(end);
    }

    let mut freq = vec![0usize; num_nails + 2];
    let mut result = 0;
    for (start, ends) in links.into_iter().enumerate() {
        for &end in &ends {
            result += freq[start + 1..end].iter().sum::<usize>();
        }
        for &end in &ends {
            freq[end] += 1;
        }
    }

    result
}

fn part3(num_nails: usize, input: &str) -> usize {
    let notes = parse_input(input);
    let size = num_nails + 2;
    // Use a flat vector to represent the 2D matrix: index = r * size + c
    let mut freq = vec![0usize; size * size];

    // 1. Fill Frequency Matrix
    for pair in notes.windows(2) {
        let (start, end) = (pair[0].min(pair[1]), pair[0].max(pair[1]));
        freq[start * size + end] += 1;
    }

    // 2. Build 2D Prefix Sum Matrix
    let mut p = vec![0usize; size * size];
    for r in 1..=num_nails {
        for c in 1..=num_nails {
            p[r * size + c] = freq[r * size + c] + p[(r - 1) * size + c] + p[r * size + (c - 1)]
                - p[(r - 1) * size + (c - 1)];
        }
    }

    // Helper closure for rectangle sums (Inclusion-Exclusion Principle)
    let get_rect_sum = |x1: usize, y1: usize, x2: usize, y2: usize| -> usize {
        if x1 > x2 || y1 > y2 {
            return 0;
        }
        p[x2 * size + y2] + p[(x1 - 1) * size + (y1 - 1)]
            - p[(x1 - 1) * size + y2]
            - p[x2 * size + (y1 - 1)]
    };

    // 3. Find strike with maximum cuts
    (1..=num_nails)
        .flat_map(|start| (start + 1..=num_nails).map(move |end| (start, end)))
        .map(|(start, end)| {
            // Zone 1: Chords that start BEFORE start and end INSIDE (start+1 to end-1)
            let zone1 = get_rect_sum(1, start + 1, start - 1, end - 1);
            // Zone 2: Chords that start INSIDE (start+1 to end-1) and end AFTER end
            let zone2 = get_rect_sum(start + 1, end + 1, end - 1, num_nails);
            // Zone 3: Direct Hit (The thread lying right under the sword)
            let direct_hit = freq[start * size + end];
            zone1 + zone2 + direct_hit
        })
        .max()
        .unwrap_or(0)
}

fn main() {
    let sample_input_01 = helpers::sample_file!("01");
    let sample_answer_01 = part1(8, &sample_input_01);
    assert_eq!(sample_answer_01, 4);
    let input_01 = helpers::input_file!("01");
    let answer_01 = part1(32, &input_01);
    println!("Answer for part 1: {answer_01}");

    let sample_input_02 = helpers::sample_file!("02");
    let sample_answer_02 = part2(8, &sample_input_02);
    assert_eq!(sample_answer_02, 21);
    let input_02 = helpers::input_file!("02");
    let answer_02 = part2(256, &input_02);
    println!("Answer for part 2: {answer_02}");

    let sample_input_03_1 = helpers::sample_file!("03");
    let sample_answer_03_1 = part3(8, &sample_input_03_1);
    assert_eq!(sample_answer_03_1, 7);
    let input_03 = helpers::input_file!("03");
    let answer_03 = part3(256, &input_03);
    println!("Answer for part 3: {answer_03}");
}
