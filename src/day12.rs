use std::fmt;

use ahash::AHashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum SpringStatus {
    Working,
    Broken,
    Unknown,
}

struct Row {
    springs: Vec<SpringStatus>,
    blocks: Vec<usize>,
}

impl fmt::Debug for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", row_to_string(&self.springs, &self.blocks))
    }
}

fn row_to_string(springs: &[SpringStatus], blocks: &[usize]) -> String {
    let springs = springs
        .iter()
        .map(|s| match s {
            SpringStatus::Working => '.',
            SpringStatus::Broken => '#',
            SpringStatus::Unknown => '?',
        })
        .collect::<String>();

    let blocks = blocks
        .iter()
        .map(|b| b.to_string())
        .collect::<Vec<_>>()
        .join(",");

    format!("{} {}", springs, blocks)
}

fn parse_row(line: &str) -> Row {
    let (s, b) = line.trim().split_once(' ').unwrap();

    let springs = s
        .bytes()
        .map(|c| match c {
            b'.' => SpringStatus::Working,
            b'#' => SpringStatus::Broken,
            b'?' => SpringStatus::Unknown,
            _ => unreachable!(),
        })
        .collect();

    let blocks = b.split(',').map(|s| s.parse::<usize>().unwrap()).collect();

    Row { springs, blocks }
}

fn parse_input(input: &str) -> Vec<Row> {
    input.lines().map(parse_row).collect()
}

type Cache<'a> = AHashMap<(&'a [SpringStatus], &'a [usize]), usize>;

fn count_arrangements<'a>(row: &'a Row, cache: &mut Cache<'a>) -> usize {
    fn munch_not_working(springs: &[SpringStatus], n: usize) -> Option<&[SpringStatus]> {
        if springs.len() < n {
            return None;
        }
        let (working, rest) = springs.split_at(n);
        if working.iter().any(|s| *s == SpringStatus::Working) {
            return None;
        }
        Some(rest)
    }

    // wrapper that checks the cache and returns early if it's a cache hit.
    // if it's a cache miss, it computes the value and inserts it into the cache
    fn rec<'a>(springs: &'a [SpringStatus], blocks: &'a [usize], cache: &mut Cache<'a>) -> usize {
        if let Some(&n) = cache.get(&(springs, blocks)) {
            return n;
        }

        let n = rec_inner(springs, blocks, cache);
        cache.insert((springs, blocks), n);
        n
    }

    fn rec_inner<'a>(
        mut springs: &'a [SpringStatus],
        blocks: &'a [usize],
        cache: &mut Cache<'a>,
    ) -> usize {
        if springs.len() + 1 < blocks.iter().sum::<usize>() + blocks.len() {
            return 0;
        }

        // Find the first non-working spring or return 1 if there are none.
        let (head, rest) = loop {
            match springs {
                [SpringStatus::Working, rest @ ..] => springs = rest,
                [head, rest @ ..] => break (*head, rest),
                [] => return usize::from(blocks.is_empty()),
            }
        };

        if blocks.is_empty() {
            return springs.iter().all(|s| *s != SpringStatus::Broken) as usize;
        }

        if head == SpringStatus::Broken {
            let n = blocks[0];
            let Some(rest) = munch_not_working(rest, n - 1) else {
                return 0;
            };
            let rest = match rest {
                [] => return usize::from(blocks[1..].is_empty()),
                [SpringStatus::Broken, ..] => return 0,
                [_, rest @ ..] => rest,
            };

            return rec(rest, &blocks[1..], cache);
        }

        // head == SpringStatus::Unknown

        // count possibilities where the spring is working
        let arrangements_if_working = rec(rest, blocks, cache);

        // count possibilities where the spring is broken
        let arrangements_if_broken = if blocks[0] == 1 {
            let rest = match rest {
                [] => return arrangements_if_working + usize::from(blocks[1..].is_empty()),
                [SpringStatus::Broken, ..] => return arrangements_if_working,
                [_, rest @ ..] => rest,
            };
            rec(rest, &blocks[1..], cache)
        } else {
            let Some(rest) = munch_not_working(rest, blocks[0] - 1) else {
                return arrangements_if_working;
            };
            let rest = match rest {
                [] => return arrangements_if_working + usize::from(blocks[1..].is_empty()),
                [SpringStatus::Broken, ..] => return arrangements_if_working,
                [_, rest @ ..] => rest,
            };
            rec(rest, &blocks[1..], cache)
        };

        arrangements_if_working + arrangements_if_broken
    }

    rec(&row.springs, &row.blocks, cache)
}

pub fn part1(input: &str) -> String {
    let rows = parse_input(input);
    let mut cache = AHashMap::new();
    rows.iter()
        .map(|row| count_arrangements(row, &mut cache))
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let mut rows = parse_input(input);
    for row in &mut rows {
        let n = row.springs.len();
        row.springs.push(SpringStatus::Unknown);
        row.springs.extend_from_within(..);
        row.springs.extend_from_within(..);
        row.springs.extend_from_within(..n);
        row.blocks = row.blocks.repeat(5);
    }
    let mut cache = AHashMap::new();
    rows.iter()
        .map(|row| count_arrangements(row, &mut cache))
        .sum::<usize>()
        .to_string()
}
