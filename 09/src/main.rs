use itertools::Itertools;

type UInt = u128;
type Dna = [UInt; 4];
type Input = Vec<(usize, Dna)>;

fn base_index(c: char) -> usize {
    match c {
        'A' => 0,
        'T' => 1,
        'C' => 2,
        'G' => 3,
        _ => unreachable!(),
    }
}
fn parse_input(input: &str) -> Input {
    input
        .lines()
        .filter_map(|line| {
            let (seq_id, seq) = line.split_once(":")?;
            let id = seq_id.parse().ok()?;
            let mut dna = [0; 4];
            for (i, c) in seq.chars().enumerate() {
                dna[base_index(c)] |= 1 << i;
            }
            Some((id, dna))
        })
        .collect()
}

fn create_parents_pairs(
    child_index: usize,
    num_sequences: usize,
) -> impl Iterator<Item = (usize, usize)> {
    (0..num_sequences)
        .filter(move |&parent_index| parent_index != child_index)
        .combinations(2)
        .map(|pair| (pair[0], pair[1]))
}

fn check_parents(parent1: &Dna, parent2: &Dna, child: &Dna) -> bool {
    (0..4).all(|i| (child[i] & !(parent1[i] | parent2[i])) == 0)
}

fn similarity(parent1: &Dna, parent2: &Dna, child: &Dna) -> UInt {
    // Validation check
    if !check_parents(parent1, parent2, child) {
        return 0;
    }

    // Calculation
    let (mut sim1, mut sim2) = (0, 0);
    for i in 0..4 {
        sim1 += (child[i] & parent1[i]).count_ones() as UInt;
        sim2 += (child[i] & parent2[i]).count_ones() as UInt;
    }
    sim1 * sim2
}

fn part1(input: &str) -> UInt {
    let sequences = parse_input(input);
    for (c, (_, child_dna)) in sequences.iter().enumerate() {
        for (p1, p2) in create_parents_pairs(c, sequences.len()) {
            let (_, parent1_dna) = &sequences[p1];
            let (_, parent2_dna) = &sequences[p2];

            match similarity(parent1_dna, parent2_dna, child_dna) {
                s if s > 0 => return s,
                _ => continue,
            }
        }
    }
    0
}

fn part2(input: &str) -> UInt {
    let sequences = parse_input(input);
    let mut result = 0;

    for (c, (_, child_dna)) in sequences.iter().enumerate() {
        for (p1, p2) in create_parents_pairs(c, sequences.len()) {
            let (_, parent1_dna) = &sequences[p1];
            let (_, parent2_dna) = &sequences[p2];

            result += similarity(parent1_dna, parent2_dna, child_dna);
        }
    }

    result
}

struct UnionFind {
    parent: Vec<usize>,
    sum: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(sequences: &Input) -> Self {
        let num_sequences = sequences.len();
        let mut parent = vec![0; num_sequences + 1];
        let mut sum = vec![0; num_sequences + 1];
        let mut size = vec![0; num_sequences + 1];

        for (id, _) in sequences {
            parent[*id] = *id;
            sum[*id] = *id;
            size[*id] = 1;
        }

        UnionFind { parent, size, sum }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }
        self.parent[i] = self.find(self.parent[i]);
        self.parent[i]
    }

    fn union(&mut self, i: usize, j: usize) {
        let (root_i, root_j) = (self.find(i), self.find(j));
        if root_i != root_j {
            if self.size[root_i] < self.size[root_j] {
                self.parent[root_i] = root_j;
                self.size[root_j] += self.size[root_i];
                self.sum[root_j] += self.sum[root_i];
            } else {
                self.parent[root_j] = root_i;
                self.size[root_i] += self.size[root_j];
                self.sum[root_i] += self.sum[root_j];
            }
        }
    }
}

fn part3(input: &str) -> usize {
    let sequences = parse_input(input);
    let mut union_find = UnionFind::new(&sequences);

    for (c, (child_id, child_dna)) in sequences.iter().enumerate() {
        for (p1, p2) in create_parents_pairs(c, sequences.len()) {
            let (parent1_id, parent1_dna) = &sequences[p1];
            let (parent2_id, parent2_dna) = &sequences[p2];

            if check_parents(parent1_dna, parent2_dna, child_dna) {
                union_find.union(*child_id, *parent1_id);
                union_find.union(*child_id, *parent2_id);

                break;
            }
        }
    }

    let mut max_size = 0;
    let mut result = 0;
    for (id, _) in &sequences {
        let root = union_find.find(*id);
        if union_find.size[root] > max_size {
            max_size = union_find.size[root];
            result = union_find.sum[root];
        }
    }

    result
}

fn main() {
    let sample_input_01 = helpers::sample_file!("01");
    let sample_answer_01 = part1(&sample_input_01);
    assert_eq!(sample_answer_01, 414);
    let input_01 = helpers::input_file!("01");
    let answer_01 = part1(&input_01);
    println!("Answer for part 1: {answer_01}");

    let sample_input_02 = helpers::sample_file!("02");
    let sample_answer_02 = part2(&sample_input_02);
    assert_eq!(sample_answer_02, 1245);
    let input_02 = helpers::input_file!("02");
    let answer_02 = part2(&input_02);
    println!("Answer for part 2: {answer_02}");

    let sample_input_03_1 = helpers::sample_file!("03_1");
    let sample_answer_03_1 = part3(&sample_input_03_1);
    assert_eq!(sample_answer_03_1, 12);
    let sample_input_03_1 = helpers::sample_file!("03_2");
    let sample_answer_03_1 = part3(&sample_input_03_1);
    assert_eq!(sample_answer_03_1, 36);
    let input_03 = helpers::input_file!("03");
    let answer_03 = part3(&input_03);
    println!("Answer for part 3: {answer_03}");
}
