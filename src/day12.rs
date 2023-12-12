
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

    let springs: Vec<_> = s
        .bytes()
        .map(|c| match c {
            b'.' => SpringStatus::Working,
            b'#' => SpringStatus::Broken,
            b'?' => SpringStatus::Unknown,
            _ => unreachable!(),
        })
        .collect();

    let blocks: Vec<_> = b.split(',').map(|s| s.parse::<usize>().unwrap()).collect();

    // These are the biggest lengths that our hashing scheme can handle. It seems that
    // the input doesn't include any larger values, but this is not guaranteed by
    // the problem statement. In the worst case we'd need to switch these to usizes
    // and just use a hashmap.
    assert!(springs.len() <= 24);
    assert!(blocks.len() <= 6);

    Row { springs, blocks }
}

fn parse_input(input: &str) -> Vec<Row> {
    input.lines().map(parse_row).collect()
}

// With our hashing scheme, cache keys are always < 2^12. At that size, an array
// is slightly faster than a hashmap on my machine.
type CacheKey = u16;
type Cache = [usize; 1 << 12];

fn count_arrangements(row: &Row, cache: &mut Cache) -> usize {
    
    fn cache_key(springs: &[SpringStatus], blocks: &[usize]) -> CacheKey {
        (springs.len() as u16) << 5 | blocks.len() as u16
    }

    fn get_cache(cache: &Cache, key: CacheKey) -> Option<usize> {
        match cache[key as usize] {
            usize::MAX => None,
            count => Some(count),
        }
    }

    fn set_cache(cache: &mut Cache, key: CacheKey, count: usize) -> usize {
        cache[key as usize] = count;
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

    fn rec(mut springs: &[SpringStatus], blocks: &[usize], cache: &mut Cache) -> usize {
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

        let key = cache_key(springs, blocks);

        if let Some(count) = get_cache(cache, key) {
            return count;
        }

        // Easy case: if there are not enough springs to cover the blocks, then there are no arrangements.
        if springs.len() < blocks.iter().sum::<usize>() + blocks.len() - 1 {
            return set_cache(cache, key, 0);
        }

        // If the first spring is unknown, then we can either assume it is working or broken, so we
        // try both cases.
        if springs[0] == SpringStatus::Unknown {
            let count_if_working = rec(&springs[1..], blocks, cache);

            let count_if_broken = match munch_not_working(springs, blocks[0]) {
                Some(munched) => rec(munched.get(1..).unwrap_or_default(), &blocks[1..], cache),
                None => 0,
            };

            return set_cache(cache, key, count_if_working + count_if_broken);
        }

        // Now it must be that springs[0] == SpringStatus::Broken.

        let ret = match munch_not_working(springs, blocks[0]) {
            Some(munched) => rec(munched.get(1..).unwrap_or_default(), &blocks[1..], cache),
            None => 0,
        };
        set_cache(cache, key, ret)
    }

    rec(&row.springs, &row.blocks, cache)
}

pub fn part1(input: &str) -> String {
    let rows = parse_input(input);
    let mut cache = [usize::MAX; 1 << 12];
    rows.iter()
        .map(|row| {
            cache.fill(usize::MAX);
            count_arrangements(row, &mut cache)
        })
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
    let mut cache = [usize::MAX; 1 << 12];
    rows.iter()
        .map(|row| {
            cache.fill(usize::MAX);
            count_arrangements(row, &mut cache)
        })
        .sum::<usize>()
        .to_string()
}
