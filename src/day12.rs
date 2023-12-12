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
    fn get_cache(cache: &Cache, springs: &[SpringStatus], blocks: &[usize]) -> Option<usize> {
        cache.get(&(springs, blocks)).copied()
    }

    fn set_cache<'a>(
        cache: &mut Cache<'a>,
        springs: &'a [SpringStatus],
        blocks: &'a [usize],
        count: usize,
    ) -> usize {
        cache.insert((springs, blocks), count);
        count
    }

    fn munch_not_working(mut springs: &[SpringStatus], n: usize) -> Option<&[SpringStatus]> {
        for _ in 0..n {
            if let [SpringStatus::Unknown | SpringStatus::Broken, rest @ ..] = springs {
                springs = rest;
            } else {
                return None;
            }
        }

        if springs.first() == Some(&SpringStatus::Broken) {
            None
        } else {
            Some(springs)
        }
    }

    fn rec<'a>(
        mut springs: &'a [SpringStatus],
        blocks: &'a [usize],
        cache: &mut Cache<'a>,
        indent: usize,
    ) -> usize {
        // strip leading working springs.
        while let [SpringStatus::Working, rest @ ..] = springs {
            springs = rest;
        }
        
        // If there are no springs, then there is only an arrangement if there are no blocks.
        if springs.is_empty() {
            return usize::from(blocks.is_empty());
        }
        
        // If there are no blocks, then there is only an arrangement if there are no broken springs.
        if blocks.is_empty() {
            return usize::from(springs.iter().all(|s| *s != SpringStatus::Broken));
        }
        
        if let Some(count) = get_cache(cache, springs, blocks) {
            return count;
        }

        // Easy case: if there are not enough springs to cover the blocks, then there are no arrangements.
        if springs.len() < blocks.iter().sum::<usize>() + blocks.len() - 1 {
            return set_cache(cache, springs, blocks, 0);
        }
    
        // If the first spring is unknown, then we can either assume it is working or broken, so we
        // try both cases.
        if springs[0] == SpringStatus::Unknown {
            let count_if_working = rec(&springs[1..], blocks, cache, indent + 1);

            let count_if_broken = match munch_not_working(springs, blocks[0]) {
                Some(munched) => rec(
                    munched.get(1..).unwrap_or_default(),
                    &blocks[1..],
                    cache,
                    indent + 1,
                ),
                None => 0,
            };

            return set_cache(cache, springs, blocks, count_if_working + count_if_broken);
        }

        // Now it must be that springs[0] == SpringStatus::Broken.

        let ret = match munch_not_working(springs, blocks[0]) {
            Some(munched) => {
                rec(
                    munched.get(1..).unwrap_or_default(),
                    &blocks[1..],
                    cache,
                    indent + 1,
                )
            }
            None =>0,
        };
        set_cache(cache, springs, blocks, ret)
    }

    rec(&row.springs, &row.blocks, cache, 0)
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
