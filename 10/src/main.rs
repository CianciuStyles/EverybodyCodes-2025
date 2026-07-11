use std::collections::{HashMap, HashSet};

type Pos = (i32, i32);

#[derive(Debug)]
struct Input {
    dragon: Pos,
    sheep: HashSet<Pos>,
    hideouts: HashSet<Pos>,
    rows: i32,
    cols: i32,
}

fn parse_input(text: &str) -> Input {
    let mut dragon = (0, 0);
    let mut sheep = HashSet::new();
    let mut hideouts = HashSet::new();

    let lines: Vec<&str> = text
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();
    let rows = lines.len() as i32;
    let cols = if rows > 0 { lines[0].len() as i32 } else { 0 };

    for (r, row) in lines.iter().enumerate() {
        for (c, cell) in row.chars().enumerate() {
            let pos = (r as i32, c as i32);
            match cell {
                'D' => dragon = pos,
                'S' => {
                    sheep.insert(pos);
                }
                '#' => {
                    hideouts.insert(pos);
                }
                _ => {}
            }
        }
    }

    Input {
        dragon,
        sheep,
        hideouts,
        rows,
        cols,
    }
}

fn dragon_round(dragon_pos: &HashSet<Pos>, rows: i32, cols: i32) -> HashSet<Pos> {
    let deltas = [
        (-2, 1),
        (-1, 2),
        (1, 2),
        (2, 1),
        (2, -1),
        (1, -2),
        (-1, -2),
        (-2, -1),
    ];

    let mut new_moves = HashSet::new();
    for &(r, c) in dragon_pos {
        for &(dr, dc) in &deltas {
            let nr = r + dr;
            let nc = c + dc;
            if nr >= 0 && nr < rows && nc >= 0 && nc < cols {
                new_moves.insert((nr, nc));
            }
        }
    }
    new_moves
}

fn part1(input: &str, num_rounds: i32) -> usize {
    let data = parse_input(input);

    let mut dragon_moves = HashSet::new();
    dragon_moves.insert(data.dragon);
    let mut frontier = dragon_moves.clone();

    for _ in 0..num_rounds {
        frontier = dragon_round(&frontier, data.rows, data.cols);
        dragon_moves.extend(&frontier);
    }

    dragon_moves.intersection(&data.sheep).count()
}

fn sheep_round(sheep_pos: &HashSet<Pos>, rows: i32) -> HashSet<Pos> {
    sheep_pos
        .iter()
        .filter(|&&(r, _)| r < rows)
        .map(|&(r, c)| (r + 1, c))
        .collect()
}

fn part2(input: &str, num_rounds: i32) -> i32 {
    let data = parse_input(input);

    let mut dragon_positions = HashSet::new();
    dragon_positions.insert(data.dragon);
    let mut remaining_sheep = data.sheep.clone();
    let mut sheep_eaten = 0;

    for _ in 0..num_rounds {
        dragon_positions = dragon_round(&dragon_positions, data.rows, data.cols);

        // Check before moving sheep: vulnerable squares = dragon_positions - hideouts
        let vulnerable: HashSet<Pos> = dragon_positions
            .difference(&data.hideouts)
            .cloned()
            .collect();

        let eaten_before: HashSet<Pos> =
            remaining_sheep.intersection(&vulnerable).cloned().collect();
        sheep_eaten += eaten_before.len() as i32;
        for pos in &eaten_before {
            remaining_sheep.remove(pos);
        }

        // Move sheep down
        let moved_sheep = sheep_round(&remaining_sheep, data.rows);

        // Check after moving sheep
        let eaten_after: HashSet<Pos> = moved_sheep.intersection(&vulnerable).cloned().collect();
        sheep_eaten += eaten_after.len() as i32;

        remaining_sheep = moved_sheep.difference(&eaten_after).cloned().collect();
    }

    sheep_eaten
}

fn part3(input: &str) -> u64 {
    let data = parse_input(input);
    let cols = data.cols;
    let rows = data.rows;

    let index = |r: i32, c: i32| -> i32 { r * cols + c };

    let deltas = [
        (-2, 1),
        (-1, 2),
        (1, 2),
        (2, 1),
        (2, -1),
        (1, -2),
        (-1, -2),
        (-2, -1),
    ];
    let mut dragon_moves: Vec<Vec<i32>> = vec![Vec::new(); (rows * cols) as usize];
    for r in 0..rows {
        for c in 0..cols {
            let mut v = Vec::new();
            for &(dr, dc) in &deltas {
                let nr = r + dr;
                let nc = c + dc;
                if nr >= 0 && nr < rows && nc >= 0 && nc < cols {
                    v.push(index(nr, nc));
                }
            }
            dragon_moves[index(r, c) as usize] = v;
        }
    }

    let mut hideout: u128 = 0;
    for &(r, c) in &data.hideouts {
        hideout |= 1u128 << index(r, c);
    }

    let mut sheep: u128 = 0;
    for &(r, c) in &data.sheep {
        sheep |= 1u128 << index(r, c);
    }

    type Cache = HashMap<(i32, u128), u64>;
    let mut d_memo: Cache = HashMap::new();
    let mut s_memo: Cache = HashMap::new();

    fn dragon_turn(
        dragon: i32,
        sheep: u128,
        dragon_moves: &[Vec<i32>],
        hideouts: u128,
        d_memo: &mut Cache,
        s_memo: &mut Cache,
        rows: i32,
        cols: i32,
    ) -> u64 {
        let key = (dragon, sheep);
        if let Some(&v) = d_memo.get(&key) {
            return v;
        }

        let mut num_paths = 0;
        for &next_dragon in &dragon_moves[dragon as usize] {
            let bit = 1u128 << next_dragon;
            let is_hideout = hideouts & bit != 0;
            let next_sheep = if is_hideout { sheep } else { sheep & !bit };

            num_paths += if next_sheep == 0 {
                1
            } else {
                sheep_turn(
                    next_dragon,
                    next_sheep,
                    dragon_moves,
                    hideouts,
                    d_memo,
                    s_memo,
                    rows,
                    cols,
                )
            };
        }

        d_memo.insert(key, num_paths);
        num_paths
    }

    fn sheep_turn(
        dragon: i32,
        sheep: u128,
        dragon_moves: &[Vec<i32>],
        hideouts: u128,
        d_memo: &mut Cache,
        s_memo: &mut Cache,
        rows: i32,
        cols: i32,
    ) -> u64 {
        let key = (dragon, sheep);
        if let Some(&v) = s_memo.get(&key) {
            return v;
        }

        let mut num_paths = 0;
        let mut any_move = false;

        let mut mask = sheep;
        while mask != 0 {
            let bit_index = mask.trailing_zeros() as i32;
            mask &= mask - 1; // remove the lowest bit

            let r = bit_index / cols;
            let new_index = bit_index + cols; // (r+1, c)

            if new_index != dragon || hideouts & (1u128 << new_index) != 0 {
                any_move = true;
                if r + 1 == rows {
                    continue; // sheep managed to escape
                }
                let next_sheep = (sheep & !(1u128 << bit_index)) | (1u128 << new_index);
                num_paths += dragon_turn(
                    dragon,
                    next_sheep,
                    dragon_moves,
                    hideouts,
                    d_memo,
                    s_memo,
                    rows,
                    cols,
                );
            }
        }

        if !any_move {
            num_paths += dragon_turn(
                dragon,
                sheep,
                dragon_moves,
                hideouts,
                d_memo,
                s_memo,
                rows,
                cols,
            );
        }

        s_memo.insert(key, num_paths);
        num_paths
    }

    sheep_turn(
        index(data.dragon.0, data.dragon.1),
        sheep,
        &dragon_moves,
        hideout,
        &mut d_memo,
        &mut s_memo,
        rows,
        cols,
    )
}

fn main() {
    let sample_input_01 = helpers::sample_file!("01");
    let sample_answer_01 = part1(&sample_input_01, 3);
    assert_eq!(sample_answer_01, 27);
    let input_01 = helpers::input_file!("01");
    let answer_01 = part1(&input_01, 4);
    println!("Answer for part 1: {answer_01}");

    let sample_input_02 = helpers::sample_file!("02");
    let sample_answer_02 = part2(&sample_input_02, 3);
    assert_eq!(sample_answer_02, 27);
    let input_02 = helpers::input_file!("02");
    let answer_02 = part2(&input_02, 20);
    println!("Answer for part 2: {answer_02}");

    let sample_input_03_1 = helpers::sample_file!("03_1");
    let sample_answer_03_1 = part3(&sample_input_03_1);
    assert_eq!(sample_answer_03_1, 15);
    let sample_input_03_2 = helpers::sample_file!("03_2");
    let sample_answer_03_2 = part3(&sample_input_03_2);
    assert_eq!(sample_answer_03_2, 8);
    let sample_input_03_3 = helpers::sample_file!("03_3");
    let sample_answer_03_3 = part3(&sample_input_03_3);
    assert_eq!(sample_answer_03_3, 44);
    let sample_input_03_4 = helpers::sample_file!("03_4");
    let sample_answer_03_4 = part3(&sample_input_03_4);
    assert_eq!(sample_answer_03_4, 4406);
    let sample_input_03_5 = helpers::sample_file!("03_5");
    let sample_answer_03_5 = part3(&sample_input_03_5);
    assert_eq!(sample_answer_03_5, 13033988838);
    let input_03 = helpers::input_file!("03");
    let answer_03 = part3(&input_03);
    println!("Answer for part 3: {answer_03}");
}
