use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, Copy)]
struct Location {
    r: usize,
    c: usize,
}
type Grid = Vec<Vec<UInt>>;
type Input = (Grid, Location, Location, usize, usize);
type UInt = u32;

fn parse_input(text: &str) -> Input {
    let mut volcano = Location { r: 0, c: 0 };
    let mut start = Location { r: 0, c: 0 };

    let mut grid = vec![];
    let mut rows = 0;
    let mut cols = 0;

    for (r, line) in text.lines().enumerate() {
        let mut row = vec![];
        for (c, char) in line.chars().enumerate() {
            match char {
                '@' => {
                    volcano = Location { r, c };
                    row.push(0 as UInt);
                }
                'S' => {
                    start = Location { r, c };
                    row.push(0 as UInt);
                }
                _ => row.push(char.to_digit(10).unwrap() as UInt),
            }
            cols = c;
        }
        grid.push(row);
        rows = r;
    }

    (grid, volcano, start, rows + 1, cols + 1)
}

fn in_range(l1: Location, l2: Location, max_dist: UInt) -> bool {
    let dr = l1.r.abs_diff(l2.r) as UInt;
    let dc = l1.c.abs_diff(l2.c) as UInt;
    (dr * dr + dc * dc) <= max_dist
}

fn distance(l1: Location, l2: Location) -> f64 {
    let dr = l1.r as f64 - l2.r as f64;
    let dc = l1.c as f64 - l2.c as f64;
    (dr * dr + dc * dc).sqrt()
}

fn part1(input: &str) -> UInt {
    let (grid, volcano, _start, rows, cols) = parse_input(input);
    let max_dist = (10 as UInt).pow(2);
    let mut result = 0;

    for r in 0..rows {
        for c in 0..cols {
            if in_range(Location { r, c }, volcano, max_dist) {
                result += grid[r][c] as UInt;
            }
        }
    }

    result
}

fn part2(input: &str) -> UInt {
    let (mut grid, volcano, _start, rows, cols) = parse_input(input);
    let mut max_radius = 0;
    let mut max_lava = 0;

    for radius in 1..volcano.r + 1 {
        let max_dist = (radius as UInt).pow(2);
        let mut curr_lava = 0;
        for r in 0..rows {
            for c in 0..cols {
                if in_range(Location { r, c }, volcano, max_dist) {
                    curr_lava += grid[r][c];
                    grid[r][c] = 0;
                }
            }
        }

        if curr_lava > max_lava {
            max_lava = curr_lava;
            max_radius = radius as UInt;
        }
    }

    max_radius * max_lava
}

fn get_ray_crossing(curr: Location, next: Location, volcano: Location) -> i32 {
    if curr.c > volcano.c && next.c > volcano.c {
        if curr.r <= volcano.r && next.r > volcano.r {
            return 1;
        }
        if curr.r > volcano.r && next.r <= volcano.r {
            return -1;
        }
    }
    0
}

fn part3(input: &str) -> UInt {
    let (grid, volcano, start, rows, cols) = parse_input(input);

    // Pre-comput Euclidean distances and ceil burn times
    // for O(1) lookups during Dijsktra
    let mut dist_grid = vec![vec![0f64; cols]; rows];
    let mut burn_time = vec![vec![0; cols]; rows];
    for r in 0..rows {
        for c in 0..cols {
            let d = distance(Location { r, c }, volcano);
            dist_grid[r][c] = d;
            burn_time[r][c] = (d.ceil() as UInt) * 30;
        }
    }

    // Linear search from radius = 1 upward
    let max_radius = rows.max(cols * 2) as UInt;
    for radius in 1..max_radius {
        let time_limit = (radius + 1) * 30;

        let mut pq = BinaryHeap::new();
        let mut visited = HashMap::new();

        pq.push(Reverse((0, start.r, start.c, 0)));

        while let Some(Reverse((time, r, c, winding))) = pq.pop() {
            // Check loop completion: back at the start with 1 full loop
            if r == start.r && c == start.c && (winding == 1 || winding == -1) {
                return (radius * time) as UInt;
            }

            let state_key = (r, c, winding);
            if let Some(&prev_time) = visited.get(&state_key) {
                if prev_time <= time {
                    continue;
                }
            }
            visited.insert(state_key, time);

            for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;

                if nr < 0 || nr == rows as i32 {
                    continue;
                }
                if nc < 0 || nc == cols as i32 {
                    continue;
                }

                let nr = nr as usize;
                let nc = nc as usize;

                // Exclude cells inside radius
                if dist_grid[nr][nc] <= radius as f64 {
                    continue;
                }

                let new_time = time + grid[nr][nc];

                // Branch pruning: drop paths exceeding time_limit
                if new_time >= time_limit {
                    continue;
                }
                if new_time < burn_time[nr][nc] {
                    let dw =
                        get_ray_crossing(Location { r, c }, Location { r: nr, c: nc }, volcano);
                    pq.push(Reverse((new_time, nr, nc, winding + dw)));
                }
            }
        }
    }

    panic!("No solution found!");
}

fn main() {
    let sample_input_01 = helpers::sample_file!("01");
    let sample_answer_01 = part1(&sample_input_01);
    assert_eq!(sample_answer_01, 1573);
    let input_01 = helpers::input_file!("01");
    let answer_01 = part1(&input_01);
    println!("Answer for part 1: {answer_01}");

    let sample_input_02 = helpers::sample_file!("02");
    let sample_answer_02 = part2(&sample_input_02);
    assert_eq!(sample_answer_02, 1090);
    let input_02 = helpers::input_file!("02");
    let answer_02 = part2(&input_02);
    println!("Answer for part 2: {answer_02}");

    let sample_input_03_1 = helpers::sample_file!("03_1");
    let sample_answer_03_1 = part3(&sample_input_03_1);
    assert_eq!(sample_answer_03_1, 592);
    let sample_input_03_2 = helpers::sample_file!("03_2");
    let sample_answer_03_2 = part3(&sample_input_03_2);
    assert_eq!(sample_answer_03_2, 330);
    let sample_input_03_3 = helpers::sample_file!("03_3");
    let sample_answer_03_3 = part3(&sample_input_03_3);
    assert_eq!(sample_answer_03_3, 3180);
    let input_03 = helpers::input_file!("03");
    let answer_03 = part3(&input_03);
    println!("Answer for part 3: {answer_03}");
}
