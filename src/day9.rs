use smallvec::SmallVec;

fn parse_input(input: &str) -> Vec<Vec<isize>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

fn extrapolate(seq: &[isize], backward: bool) -> isize {
    if seq.iter().all(|&n| n == 0) {
        return 0;
    }

    // It seems like the sequences are all at most ~20 elements long, so we can use a SmallVec
    // instead of a Vec to avoid heap allocations. This reduces the computation time (runtime excluding
    // parsing) by ~50%, from ~100µs to ~50µs.
    let diffs = seq
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect::<SmallVec<[isize; 25]>>();
    let e = extrapolate(&diffs, backward);
    if backward {
        seq.first().unwrap() - e
    } else {
        seq.last().unwrap() + e
    }
}

pub fn part1(input: &str) -> String {
    let seqs = parse_input(input);
    seqs.iter()
        .map(|seq| extrapolate(seq, false))
        .sum::<isize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let seqs = parse_input(input);
    seqs.iter()
        .map(|seq| extrapolate(seq, true))
        .sum::<isize>()
        .to_string()
}
