use ahash::AHashMap;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum SpringStatus {
    Working,
    Broken,
    Unknown,
}

struct Row {
    springs: Vec<SpringStatus>,
    blocks: Vec<usize>,
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
    ///  Tries to remove the first `n` springs from the front of the slice, and returns `None`
    ///  if any of them are working or if the slice is too short. Otherwise,
    ///  returns the rest of the slice.
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

    /// Wrapper that checks the cache and returns early if it's a cache hit.
    /// If it's a cache miss, it computes the value and inserts it into the cache
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
        // If the blocks require more broken springs than there are total springs,
        // there are no arrangements.
        if springs.len() + 1 < blocks.iter().sum::<usize>() + blocks.len() {
            return 0;
        }

        // Find the first non-working spring or return 1 if there are none.
        let (head, rest) = loop {
            match springs {
                [SpringStatus::Working, rest @ ..] => springs = rest,
                [head, rest @ ..] => break (*head, rest),
                // If there are no non-working springs, then there is only
                // an arrangement if there are no blocks.
                [] => return usize::from(blocks.is_empty()),
            }
        };

        // If there are no blocks, then there is only an arrangement if no springs are broken.
        if blocks.is_empty() {
            return springs.iter().all(|s| *s != SpringStatus::Broken) as usize;
        }

        // If the first spring is broken, then we can immediately munch the entire first block.
        if head == SpringStatus::Broken {
            let n = blocks[0];
            let Some(rest) = munch_not_working(rest, n - 1) else {
                return 0;
            };
            let rest = match rest {
                // If there are no remaining springs, then there is only an
                // arrangement if there are also no remaining blocks.
                [] => return usize::from(blocks[1..].is_empty()),
                // If the block of broken springs is followed by a broken spring,
                // then there are no arrangements.
                [SpringStatus::Broken, ..] => return 0,
                // Otherwise, skip the first spring (it cannot be broken) as well as
                // the first block, and count the remaining arrangements.
                [_, rest @ ..] => rest,
            };

            return rec(rest, &blocks[1..], cache);
        }

        // At this point we know that head == SpringStatus::Unknown

        // count possibilities where the spring is working.
        let arrangements_if_working = rec(rest, blocks, cache);

        // count possibilities where the spring is broken.
        // If the first block only has one spring, just check it directly
        // and recurse. Otherwise, munch the whole block and recurse.
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
