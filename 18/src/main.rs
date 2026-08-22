use helpers::{split_at_double_newline, split_at_triple_newline};
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

struct Garden {
    pub plant_thicknesses: HashMap<i64, i64>,
    pub free_branches: HashMap<i64, i64>,
    pub connections: HashMap<i64, Vec<(i64, i64)>>,
}

impl FromStr for Garden {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut plant_thicknesses = HashMap::new();
        let mut free_branches = HashMap::new();
        let mut connections: HashMap<i64, Vec<(i64, i64)>> = HashMap::new();

        let plant_re = Regex::new(r"Plant (\d+) with thickness (-?\d+):").unwrap();
        let free_branch_re = Regex::new(r"- free branch with thickness (-?\d+)").unwrap();
        let branch_to_plant_re =
            Regex::new(r"- branch to Plant (\d+) with thickness (-?\d+)").unwrap();

        for block in split_at_double_newline(text) {
            let mut lines = block.lines();
            if let Some(plant_definition) = lines.next() {
                if let Some(caps) = plant_re.captures(plant_definition) {
                    let plant_num = caps[1].parse().map_err(|_| "Invalid plant ID")?;
                    let thickness = caps[2].parse().map_err(|_| "Invalid thickness")?;
                    plant_thicknesses.insert(plant_num, thickness);

                    for connection in lines {
                        if let Some(caps) = branch_to_plant_re.captures(connection) {
                            let conn_plant: i64 = caps[1].parse().unwrap();
                            let conn_thickness: i64 = caps[2].parse().unwrap();
                            connections
                                .entry(plant_num)
                                .or_default()
                                .push((conn_plant, conn_thickness));
                        } else if let Some(caps) = free_branch_re.captures(connection) {
                            let thickness: i64 = caps[1].parse().unwrap();
                            free_branches.insert(plant_num, thickness);
                        }
                    }
                }
            }
        }

        Ok(Garden {
            plant_thicknesses,
            free_branches,
            connections,
        })
    }
}

fn calculate_energy(garden: &Garden, free_branches_vals: &[i64], target: i64) -> i64 {
    fn recur(curr_plant: i64, garden: &Garden, free_branches_vals: &[i64]) -> i64 {
        let mut incoming_energy = 0;

        if let Some(&thickness) = garden.free_branches.get(&curr_plant) {
            let active_val = free_branches_vals
                .get((curr_plant - 1) as usize)
                .copied()
                .unwrap_or(0);
            incoming_energy = thickness * active_val;
        } else if let Some(conns) = garden.connections.get(&curr_plant) {
            for &(child, child_thickness) in conns {
                incoming_energy += recur(child, garden, free_branches_vals) * child_thickness;
            }
        }

        let required = garden
            .plant_thicknesses
            .get(&curr_plant)
            .copied()
            .unwrap_or(0);
        if incoming_energy >= required {
            incoming_energy
        } else {
            0
        }
    }

    recur(target, garden, free_branches_vals)
}

fn parse_test_cases(text: &str) -> Vec<Vec<i64>> {
    text.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|value| value.parse().unwrap())
                .collect()
        })
        .collect()
}

fn part1(input: &str) -> i64 {
    let garden: Garden = input.parse().unwrap();
    let initial_vals = vec![1; garden.free_branches.len()];
    let target_plant = *garden.plant_thicknesses.keys().max().unwrap_or(&0);

    calculate_energy(&garden, &initial_vals, target_plant)
}

fn part2(text: &str) -> i64 {
    let parts: Vec<&str> = split_at_triple_newline(text);
    let garden: Garden = parts[0].parse().unwrap();
    let test_cases = parse_test_cases(parts[1]);
    let target_plant = *garden.plant_thicknesses.keys().max().unwrap_or(&0);

    test_cases
        .iter()
        .map(|test_case| calculate_energy(&garden, test_case, target_plant))
        .sum()
}

fn part3(text: &str) -> i64 {
    let parts: Vec<&str> = split_at_triple_newline(text);
    let garden: Garden = parts[0].parse().unwrap();
    let test_cases = parse_test_cases(parts[1]);
    let target_plant = *garden.plant_thicknesses.keys().max().unwrap_or(&0);

    let mut test_results = Vec::with_capacity(test_cases.len());
    let mut best_config = vec![0; garden.free_branches.len()];
    let mut best_score = 0;

    for test_case in &test_cases {
        let curr_score = calculate_energy(&garden, test_case, target_plant);
        test_results.push(curr_score);

        if curr_score > best_score {
            best_config = test_case.clone();
            best_score = curr_score;
        }
    }

    let mut score_was_improved = true;
    while score_was_improved {
        score_was_improved = false;
        for i in 0..best_config.len() {
            // Try mutating the value inline
            best_config[i] = if best_config[i] == 1 { 0 } else { 1 };

            let curr_score = calculate_energy(&garden, &best_config, target_plant);

            if curr_score > best_score {
                best_score = curr_score;
                score_was_improved = true;
                break; // Keep climbing immediately
            } else {
                // Revert the mutation inline if it didn't help climb the hill
                best_config[i] = if best_config[i] == 1 { 0 } else { 1 };
            }
        }
    }

    test_results
        .iter()
        .filter(|&&score| score > 0)
        .map(|&score| best_score - score)
        .sum()
}

fn main() {
    let sample_input_01 = helpers::sample_file!("01");
    let sample_answer_01 = part1(&sample_input_01);
    assert_eq!(sample_answer_01, 774);
    let input_01 = helpers::input_file!("01");
    let answer_01 = part1(&input_01);
    println!("Answer for part 1: {answer_01}");

    let sample_input_02 = helpers::sample_file!("02");
    let sample_answer_02 = part2(&sample_input_02);
    assert_eq!(sample_answer_02, 324);
    let input_02 = helpers::input_file!("02");
    let answer_02 = part2(&input_02);
    println!("Answer for part 2: {answer_02}");

    let sample_input_03 = helpers::sample_file!("03");
    let sample_answer_03 = part3(&sample_input_03);
    assert_eq!(sample_answer_03, 946);
    let input_03 = helpers::input_file!("03");
    let answer_03 = part3(&input_03);
    println!("Answer for part 3: {answer_03}");
}
