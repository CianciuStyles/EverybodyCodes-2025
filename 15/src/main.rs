use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::cmp::Ordering;

type Direction = (i64, i64);
type Graph = HashMap<Point, HashSet<(Point, i64)>>;
type Instruction = (char, i64);
type Point = (i64, i64);

// Custom wrapper to reverse BinaryHeap ordering into a Min-Heap for Dijkstra
#[derive(Eq, PartialEq)]
struct State {
    cost: i64,
    position: Point,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse order for min-heap
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const UP: Direction = (-1, 0);
const DOWN: Direction = (1, 0);
const LEFT: Direction = (0, -1);
const RIGHT: Direction = (0, 1);

fn parse_input(text: &str) -> Vec<Instruction> {
    text.split(',')
        .map(|s| {
            let s = s.trim();
            let turn = s.chars().next().unwrap();
            let steps = s[1..].parse().unwrap();
            (turn, steps)
        })
        .collect()
}

fn get_next_direction(curr: Direction, turn: char) -> Direction {
    if turn == 'L' {
        match curr {
            UP => LEFT,
            LEFT => DOWN,
            DOWN => RIGHT,
            RIGHT => UP,
            _ => unreachable!(),
        }
    } else {
        match curr {
            UP => RIGHT,
            RIGHT => DOWN,
            DOWN => LEFT,
            LEFT => UP,
            _ => unreachable!(),
        }
    }
}

fn part1(input: &str) -> u32 {
    let instructions = parse_input(&input);
    let start: Point = (0, 0);
    let mut curr_pos = start;
    let mut direction = UP;
    let mut walls = HashSet::new();

    for (turn, steps) in instructions {
        direction = get_next_direction(direction, turn);
        for _ in 0..steps {
            curr_pos = (curr_pos.0 + direction.0, curr_pos.1 + direction.1);
            walls.insert(curr_pos);
        }
    }

    walls.remove(&curr_pos);
    let end = curr_pos;

    let mut frontier = VecDeque::new();
    frontier.push_back((start, 0));

    let mut visited = HashSet::new();
    visited.insert(start);

    while let Some(((r, c), num_steps)) = frontier.pop_front() {
        if (r, c) == end {
            return num_steps;
        }

        for (dr, dc) in &[UP, DOWN, LEFT, RIGHT] {
            let next_p = (r + dr, c + dc);
            if !walls.contains(&next_p) && visited.insert(next_p) {
                frontier.push_back((next_p, num_steps + 1));
            }
        }
    }

    unreachable!()
}

fn blocked(v1: Point, v2: Point, horizontal_walls: &HashMap<i64, Vec<(i64, i64)>>, vertical_walls: &HashMap<i64, Vec<(i64, i64)>>) -> bool {
    // Self-sorting point extraction
    let (left, right) = if v1 < v2 { (v1, v2) } else { (v2, v1) };
    let (left_r, left_c) = left;
    let (right_r, right_c) = right;

    if left_r == right_r { // Horizontal Path
        // 1. Perpendicular Check
        for (&vc, vwalls) in vertical_walls {
            if left_c < vc && vc < right_c {
                for &(vr1, vr2) in vwalls {
                    if vr1 <= left_r && left_r <= vr2 {
                        return true;
                    }
                }
            }
        }
        // 2. Collinear Check
        if let Some(hwalls) = horizontal_walls.get(&left_r) {
            for &(hr1, hr2) in hwalls {
                if left_c < hr2 && right_c > hr1 {
                    return true;
                }
            }
        }
    } else if left_c == right_c { // Vertical Path
        // 1. Perpendicular Check
        for (&hr, hwalls) in horizontal_walls {
            if left_r < hr && hr < right_r {
                for &(hc1, hc2) in hwalls {
                    if hc1 <= left_c && left_c <= hc2 {
                        return true;
                    }
                }
            }
        }
        // 2. Collinear Check
        if let Some(vwalls) = vertical_walls.get(&left_c) {
            for &(vr1, vr2) in vwalls {
                if left_r < vr2 && right_r > vr1 {
                    return true;
                }
            }
        }
    }
    false
}

fn find_shortest_path(graph: &HashMap<Point, HashSet<(Point, i64)>>, start: Point, end: Point) -> i64 {
    let mut queue = BinaryHeap::new();
    let mut distances = HashMap::new();

    distances.insert(start, 0);
    queue.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = queue.pop() {
        if position == end {
            return cost;
        }

        if let Some(&best) = distances.get(&position) {
            if cost > best {
                continue;
            }
        }

        if let Some(neighbours) = graph.get(&position) {
            for &(neighbour, weight) in neighbours {
                let next_dist = cost + weight;
                let current_best = distances.get(&neighbour).copied().unwrap_or(i64::MAX);

                if next_dist < current_best {
                    distances.insert(neighbour, next_dist);
                    queue.push(State { cost: next_dist, position: neighbour });
                }
            }
        }
    }

    unreachable!()
}

fn part2(input: &str) -> i64 {
    let instructions = parse_input(&input);
    let start: Point = (0, 0);
    let mut curr_pos = start;
    let mut direction = UP;

    let mut walls = HashSet::new();
    let mut corners = HashSet::new();
    corners.insert(start);

    for (turn, steps) in instructions {
        direction = get_next_direction(direction, turn);
        let corner_1 = curr_pos;
        let corner_2 = (curr_pos.0 + direction.0 * steps, curr_pos.1 + direction.1 * steps);

        walls.insert((std::cmp::min(corner_1, corner_2), std::cmp::max(corner_1, corner_2)));
        corners.insert(corner_1);
        corners.insert(corner_2);
        curr_pos = corner_2;
    }

    let end = curr_pos;

    // Generate corner offset vertices safely
    let mut raw_vertices = HashSet::new();
    for &(r, c) in &corners {
        for &(dr, dc) in &[(-1, -1), (1, 1), (1, -1), (-1, 1)] {
            raw_vertices.insert((r + dr, c + dc));
        }
    }
    let mut vertices: Vec<Point> = raw_vertices.into_iter().collect();
    vertices.sort();

    // Map profiles for O(1) group filtering
    let mut horizontal_walls: HashMap<i64, Vec<(i64, i64)>> = HashMap::new();
    let mut vertical_walls: HashMap<i64, Vec<(i64, i64)>> = HashMap::new();

    for &((r1, c1), (r2, c2)) in &walls {
        if r1 == r2 {
            horizontal_walls.entry(r1).or_default().push((c1, c2));
        }
        if c1 == c2 {
            vertical_walls.entry(c1).or_default().push((r1, r2));
        }
    }

    let mut graph: Graph = HashMap::new();

    // Visibility pipeline loop
    for i in 0..vertices.len() {
        for j in (i + 1)..vertices.len() {
            let v1 = vertices[i];
            let v2 = vertices[j];

            // Case 1: Shared axis straight shot
            if v1.0 == v2.0 || v1.1 == v2.1 {
                if !blocked(v1, v2, &horizontal_walls, &vertical_walls) {
                    let dist = (v2.0 - v1.0).abs() + (v2.1 - v1.1).abs();
                    graph.entry(v1).or_default().insert((v2, dist));
                    graph.entry(v2).or_default().insert((v1, dist));
                }
            }
            // Case 2: Check L-shaped open turning pivots
            else {
                let p_a = (v1.0, v2.1);
                if !blocked(v1, p_a, &horizontal_walls, &vertical_walls) && !blocked(p_a, v2, &horizontal_walls, &vertical_walls) {
                    let dist = (v2.0 - v1.0).abs() + (v2.1 - v1.1).abs();
                    graph.entry(v1).or_default().insert((v2, dist));
                    graph.entry(v2).or_default().insert((v1, dist));
                }

                let p_b = (v2.0, v1.1);
                if !blocked(v1, p_b, &horizontal_walls, &vertical_walls) && !blocked(p_b, v2, &horizontal_walls, &vertical_walls) {
                    let dist = (v2.0 - v1.0).abs() + (v2.1 - v1.1).abs();
                    graph.entry(v1).or_default().insert((v2, dist));
                    graph.entry(v2).or_default().insert((v1, dist));
                }
            }
        }
    }

    // Connect starting point
    for &v in &vertices {
        if (v.0 - start.0).abs() <= 1 && (v.1 - start.1).abs() <= 1 {
            let dist = (v.0 - start.0).abs() + (v.1 - start.1).abs();
            graph.entry(start).or_default().insert((v, dist));
            graph.entry(v).or_default().insert((start, dist));
        }
    }

    // Connect end point
    for &v in &vertices {
        if (v.0 - end.0).abs() <= 1 && (v.1 - end.1).abs() <= 1 {
            let dist = (v.0 - end.0).abs() + (v.1 - end.1).abs();
            graph.entry(end).or_default().insert((v, dist));
            graph.entry(v).or_default().insert((end, dist));
        }
    }

    find_shortest_path(&graph, start, end)
}

fn main() {
    let sample_input_01 = helpers::sample_file!("01");
    let sample_answer_01 = part1(&sample_input_01);
    assert_eq!(sample_answer_01, 16);
    let input_01 = helpers::input_file!("01");
    let answer_01 = part1(&input_01);
    println!("Answer for part 1: {answer_01}");

    let input_02 = helpers::input_file!("02");
    let answer_02 = part1(&input_02);
    println!("Answer for part 2: {answer_02}");

    let input_03 = helpers::input_file!("03");
    let answer_03 = part2(&input_03);
    println!("Answer for part 3: {answer_03}");
}
