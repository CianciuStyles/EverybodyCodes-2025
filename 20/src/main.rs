use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

type Grid = Vec<Vec<char>>;
type Location = (usize, usize);
type Input = (Grid, Location, Location, usize, usize);

fn parse_input(text: &str) -> Input {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut grid = vec![];
    let mut rows = 0;
    let mut cols = 0;

    for (r, line) in text.lines().enumerate() {
        let mut row = vec![];
        for (c, char) in line.chars().enumerate() {
            match char {
                'E' => {
                    end = (r, c);
                }
                'S' => start = (r, c),
                _ => {}
            }
            row.push(char);
            cols = c;
        }
        grid.push(row);
        rows = r;
    }

    (grid, start, end, rows + 1, cols + 1)
}

fn part1(input: &str) -> u32 {
    let (grid, _start, _end, rows, cols) = parse_input(input);
    let trampoline = 'T';
    let mut result = 0;

    for (r, row) in grid.iter().enumerate() {
        for (c, char) in row.iter().enumerate() {
            if char.eq(&trampoline) {
                // Check right
                if c + 1 < cols && grid[r][c + 1].eq(&trampoline) {
                    result += 1;
                }
                // Check down
                if r + 1 < rows && grid[r + 1][c].eq(&trampoline) && (r % 2) != (c % 2) {
                    result += 1;
                }
            }
        }
    }
    result
}

fn part2(input: &str) -> u32 {
    let (grid, start, end, rows, cols) = parse_input(input);
    let mut visited = HashMap::new();
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start)));

    while let Some(Reverse((num_jumps, curr_pos))) = heap.pop() {
        if curr_pos.eq(&end) {
            return num_jumps;
        }

        if visited.get(&curr_pos).unwrap_or(&u32::MAX) <= &num_jumps {
            continue;
        }
        visited.insert(curr_pos, num_jumps);

        let (r, c) = curr_pos;
        for (dr, dc) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (nr, nc) = (r as i32 + dr, c as i32 + dc);

            // Test boundaries
            if nr < 0 || nc < 0 || nr >= rows as i32 || nc >= cols as i32 {
                continue;
            }

            let (nr, nc) = (nr as usize, nc as usize);
            let new_cell = grid[nr][nc];
            // Only consider moves to trampolines
            if new_cell.ne(&'T') && new_cell.ne(&'E') {
                continue;
            }

            // Check if the trampoline is reachable
            if dr == 1 && (r % 2) == (c % 2) {
                // Can't move down from a trampoline if row and col parity are the same
                continue;
            }
            if dr == -1 && (r % 2) != (c % 2) {
                // Can't move up from a trampoline if row and col parity are different
                continue;
            }

            heap.push(Reverse((num_jumps + 1, (nr, nc))))
        }
    }

    panic!("No path found.")
}

fn rotate_grid(grid: &Grid, rows: usize, cols: usize) -> Grid {
    let mut new_grid = grid.clone();
    let n = rows - 1;

    for r in 0..rows {
        for c in r..(cols - r) {
            let j = c - r;
            let a = n - r;
            let i = j / 2;
            let r_new = i;
            let c_new = 2 * a - i - (j % 2);
            new_grid[r_new][c_new] = grid[r][c];
        }
    }

    new_grid
}

fn part3(input: &str) -> u32 {
    let (grid, start, end, rows, cols) = parse_input(input);
    let rotations = vec![
        grid.clone(),
        rotate_grid(&grid, rows, cols),
        rotate_grid(&rotate_grid(&grid, rows, cols), rows, cols),
    ];
    let mut visited = HashMap::new();
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start, 0)));

    while let Some(Reverse((num_jumps, curr_pos, rotation_index))) = heap.pop() {
        if curr_pos.eq(&end) {
            return num_jumps;
        }

        if visited
            .get(&(curr_pos, rotation_index))
            .unwrap_or(&u32::MAX)
            <= &num_jumps
        {
            continue;
        }
        visited.insert((curr_pos, rotation_index), num_jumps);

        let (r, c) = curr_pos;
        let next_grid = &rotations[(rotation_index + 1) % rotations.len()];
        for (dr, dc) in vec![(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (nr, nc) = (r as i32 + dr, c as i32 + dc);

            // Test boundaries
            if nr < 0 || nc < 0 || nr >= rows as i32 || nc >= cols as i32 {
                continue;
            }

            let (nr, nc) = (nr as usize, nc as usize);
            let new_cell = next_grid[nr][nc];
            // Only consider moves to trampolines
            if new_cell.ne(&'T') && new_cell.ne(&'E') {
                continue;
            }

            // Check if the trampoline is reachable
            if dr == 1 && (r % 2) == (c % 2) {
                // Can't move down from a trampoline if row and col parity are the same
                continue;
            }
            if dr == -1 && (r % 2) != (c % 2) {
                // Can't move up from a trampoline if row and col parity are different
                continue;
            }

            heap.push(Reverse((num_jumps + 1, (nr, nc), rotation_index + 1)))
        }
    }

    panic!("No path found.")
}

fn main() {
    let sample_input_01 = helpers::sample_file!("01");
    let sample_answer_01 = part1(&sample_input_01);
    assert_eq!(sample_answer_01, 7);
    let input_01 = helpers::input_file!("01");
    let answer_01 = part1(&input_01);
    println!("Answer for part 1: {answer_01}");

    let sample_input_02 = helpers::sample_file!("02");
    let sample_answer_02 = part2(&sample_input_02);
    assert_eq!(sample_answer_02, 32);
    let input_02 = helpers::input_file!("02");
    let answer_02 = part2(&input_02);
    println!("Answer for part 2: {answer_02}");

    let sample_input_03 = helpers::sample_file!("03");
    let sample_answer_03 = part3(&sample_input_03);
    assert_eq!(sample_answer_03, 23);
    let input_03 = helpers::input_file!("03");
    let answer_03 = part3(&input_03);
    println!("Answer for part 3: {answer_03}");
}
