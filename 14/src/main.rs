use std::collections::{HashMap, HashSet};

type Coord = (usize, usize);
type NeighborsMap = HashMap<Coord, HashSet<Coord>>;

fn parse_input(text: &str) -> (HashSet<Coord>, usize, usize) {
    let rows: Vec<&str> = text.lines().map(|line| line.trim()).filter(|line| !line.is_empty()).collect();
    let r_max = rows.len();
    let c_max = if r_max > 0 { rows[0].len() } else { 0 };

    let mut active_tiles = HashSet::new();
    for (r, row) in rows.iter().enumerate() {
        for (c, cell) in row.chars().enumerate() {
            if cell == '#' {
                active_tiles.insert((r, c));
            }
        }
    }
    (active_tiles, r_max, c_max)
}
fn calculate_neighbours(r_max: usize, c_max: usize) -> NeighborsMap {
    let mut neighbors = HashMap::new();
    let directions = [(-1, -1), (1, 1), (1, -1), (-1, 1)];

    for r in 0..r_max {
        for c in 0..c_max {
            let mut cell_neighbors = HashSet::new();
            for &(dr, dc) in &directions {
                let nr = r as isize + dr;
                let nc = c as isize + dc;
                if nr >= 0 && nr < r_max as isize && nc >= 0 && nc < c_max as isize {
                    cell_neighbors.insert((nr as usize, nc as usize));
                }
            }
            neighbors.insert((r, c), cell_neighbors);
        }
    }
    neighbors
}

fn perform_round(active_tiles: &HashSet<Coord>, neighbors: &NeighborsMap, r_max: usize, c_max: usize) -> HashSet<Coord> {
    let mut new_active_tiles = HashSet::new();

    for r in 0..r_max {
        for c in 0..c_max {
            let curr_tile = (r, c);

            // Get precomputed neighbors for this cell
            if let Some(cell_neighbors) = neighbors.get(&curr_tile) {
                // Count active diagonal neighbors using set intersection count
                let active_neighbors_count = cell_neighbors.iter().filter(|coord| active_tiles.contains(coord)).count();
                let is_currently_active = active_tiles.contains(&curr_tile);

                // Core parity rule: active neighbors count % 2 == current state as 1 or 0
                if active_neighbors_count % 2 == (is_currently_active as usize) {
                    new_active_tiles.insert(curr_tile);
                }
            }
        }
    }
    new_active_tiles
}

fn simulate(mut active_tiles: HashSet<Coord>, r_max: usize, c_max: usize, num_rounds: usize) -> usize {
    let mut result = 0;
    let neighbors = calculate_neighbours(r_max, c_max);

    for _ in 0..num_rounds {
        active_tiles = perform_round(&active_tiles, &neighbors, r_max, c_max);
        result += active_tiles.len();
    }
    result
}

fn part1(input: &str) -> usize {
    let (active_tiles, r_max, c_max) = parse_input(&input);
    simulate(active_tiles, r_max, c_max, 10)
}

fn part2(input: &str) -> usize {
    let (active_tiles, r_max, c_max) = parse_input(&input);
    simulate(active_tiles, r_max, c_max, 2025)
}

fn part3(input: &str) -> usize {
    let (pattern_tiles, pattern_r, pattern_c) = parse_input(&input);

    let r_max = 34;
    let c_max = 34;
    let rr = (r_max - pattern_r) / 2;
    let cc = (c_max - pattern_c) / 2;

    let total_rounds: usize = 1_000_000_000;
    let period = 4095;

    let center_coordinates: HashSet<Coord> = (0..pattern_r)
        .flat_map(|r| (0..pattern_c).map(move |c| (r + rr, c + cc)))
        .collect();

    let target_center_pattern: HashSet<Coord> = pattern_tiles
        .iter()
        .map(|&(r, c)| (r + rr, c + cc))
        .collect();

    let full_cycles = total_rounds / period;
    let remainder_rounds = total_rounds % period;

    let mut period_matches = 0;
    let mut remainder_matches = 0;

    let neighbors = calculate_neighbours(r_max, c_max);
    let mut active_tiles = HashSet::new(); // Grid goes dark at round 0

    for current_round in 1..=period {
        active_tiles = perform_round(&active_tiles, &neighbors, r_max, c_max);

        let current_center: HashSet<Coord> = active_tiles
            .iter()
            .cloned()
            .filter(|coord| center_coordinates.contains(coord))
            .collect();

        if current_center == target_center_pattern {
            let num_active_tiles = active_tiles.len();
            period_matches += num_active_tiles;
            if current_round <= remainder_rounds {
                remainder_matches += num_active_tiles;
            }
        }
    }

    (full_cycles * period_matches) + remainder_matches
}
fn main() {
    let sample_input_01 = helpers::sample_file!("01");
    let sample_answer_01 = part1(&sample_input_01);
    assert_eq!(sample_answer_01, 200);
    let input_01 = helpers::input_file!("01");
    let answer_01 = part1(&input_01);
    println!("Answer for part 1: {answer_01}");

    let input_02 = helpers::input_file!("02");
    let answer_02 = part2(&input_02);
    println!("Answer for part 2: {answer_02}");


    let sample_input_03 = helpers::sample_file!("03");
    let sample_answer_03 = part3(&sample_input_03);
    assert_eq!(sample_answer_03, 278388552);
    let input_03 = helpers::input_file!("03");
    let answer_03 = part3(&input_03);
    println!("Answer for part 3: {answer_03}");
}
