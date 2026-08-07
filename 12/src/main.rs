use std::collections::{HashSet, VecDeque};

type Grid = Vec<Vec<u8>>;
type Point = (usize, usize);

fn parse_input(input: &str) -> Grid {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Invalid digit in input") as u8)
                .collect()
        })
        .collect()
}

fn find_path(grid: &Grid, start: Point, destroyed: &HashSet<Point>) -> HashSet<Point> {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut path = HashSet::new();
    let mut queue = VecDeque::new();

    // If the starting point is already destroyed, it can't propagate fire
    if destroyed.contains(&start) {
        return path;
    }

    path.insert(start);
    queue.push_back(start);

    // 4-directional offsets
    let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some((r, c)) = queue.pop_front() {
        let current_val = grid[r][c];

        for &(dr, dc) in &directions {
            let new_r = r as isize + dr;
            let new_c = c as isize + dc;

            // Bounds check using safe casting
            if new_r >= 0 && new_r < rows as isize && new_c >= 0 && new_c < cols as isize {
                let nr = new_r as usize;
                let nc = new_c as usize;
                let next_point = (nr, nc);

                if path.contains(&next_point) { continue; }
                if destroyed.contains(&next_point) { continue; }
                if grid[nr][nc] > current_val { continue; }

                path.insert(next_point);
                queue.push_back(next_point);
            }
        }
    }

    path
}

fn part1(input: &str) -> usize {
    let grid = parse_input(input);
    let empty_destroyed = HashSet::new();
    find_path(&grid, (0, 0), &empty_destroyed).len()
}

fn part2(input: &str) -> usize {
    let grid = parse_input(input);
    let empty_destroyed = HashSet::new();
    let top_left = (0, 0);
    let bottom_right = (grid.len()-1, grid[0].len()-1);

    let path_top_left = find_path(&grid, top_left, &empty_destroyed);
    let path_bottom_right = find_path(&grid, bottom_right, &empty_destroyed);

    path_top_left.union(&path_bottom_right).count()
}

fn part3(input: &str) -> usize {
    let grid = parse_input(input);
    let rows = grid.len();
    let cols = grid[0].len();

    // Pre-extract all coordinates paired with their barrel sizes
    let mut sorted_cells: Vec<(u8, Point)> = Vec::with_capacity(rows * cols);
    for r in 0..rows {
        for c in 0..cols {
            sorted_cells.push((grid[r][c], (r, c)));
        }
    }
    // Sort descending by barrel size (largest to smallest)
    sorted_cells.sort_by(|a, b| b.0.cmp(&a.0));

    let mut path_1 = HashSet::new();
    let mut path_2 = HashSet::new();
    let mut path_3 = HashSet::new();

    // Pass 1: Longest Path
    let mut explored_in_round = HashSet::new();
    for &(_, point) in &sorted_cells {
        if explored_in_round.contains(&point) { continue; }

        let curr_path = find_path(&grid, point, &HashSet::new());
        // Extend our round pruning set
        explored_in_round.extend(curr_path.iter().cloned());

        if curr_path.len() > path_1.len() {
            path_1 = curr_path;
        }
    }

    // Pass 2: Second Longest Path
    let mut explored_in_round = HashSet::new();
    for &(_, point) in &sorted_cells {
        if path_1.contains(&point) { continue; }
        if explored_in_round.contains(&point) { continue; }

        let curr_path = find_path(&grid, point, &path_1);
        explored_in_round.extend(curr_path.iter().cloned());

        if curr_path.len() > path_2.len() {
            path_2 = curr_path;
        }
    }

    // Pass 3: Third Longest Path
    let path_12: HashSet<Point> = path_1.union(&path_2).cloned().collect();
    let mut explored_in_round = HashSet::new();
    for &(_, point) in &sorted_cells {
        if path_12.contains(&point) { continue; }
        if explored_in_round.contains(&point) { continue; }

        let curr_path = find_path(&grid, point, &path_12);
        explored_in_round.extend(curr_path.iter().cloned());

        if curr_path.len() > path_3.len() {
            path_3 = curr_path;
        }
    }

    // Calculate the total unique barrels destroyed across all three paths
    let dynamic_union: HashSet<Point> = path_1
        .union(&path_2).cloned().collect::<HashSet<_>>()
        .union(&path_3).cloned().collect();

    dynamic_union.len()
}

fn main() {
    let sample_input_01 = helpers::sample_file!("01");
    let sample_answer_01 = part1(&sample_input_01);
    assert_eq!(sample_answer_01, 16);
    let input_01 = helpers::input_file!("01");
    let answer_01 = part1(&input_01);
    println!("Answer for part 1: {answer_01}");

    let sample_input_02 = helpers::sample_file!("02");
    let sample_answer_02 = part2(&sample_input_02);
    assert_eq!(sample_answer_02, 58);
    let input_02 = helpers::input_file!("02");
    let answer_02 = part2(&input_02);
    println!("Answer for part 2: {answer_02}");

    let sample_input_03 = helpers::sample_file!("03");
    let sample_answer_03 = part3(&sample_input_03);
    assert_eq!(sample_answer_03, 136);
    let input_03 = helpers::input_file!("03");
    let answer_03 = part3(&input_03);
    println!("Answer for part 3: {answer_03}");
}
