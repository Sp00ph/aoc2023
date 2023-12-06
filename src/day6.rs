#[derive(Debug, Clone, Copy)]
struct Race {
    time: usize,
    record: usize,
}

fn parse_input_part1(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times_line = lines
        .next()
        .and_then(|s| s.strip_prefix("Time:"))
        .unwrap()
        .trim();
    let distances_line = lines
        .next()
        .and_then(|s| s.strip_prefix("Distance:"))
        .unwrap()
        .trim();

    times_line
        .split_whitespace()
        .zip(distances_line.split_whitespace())
        .map(|(time, distance)| Race {
            time: time.parse().unwrap(),
            record: distance.parse().unwrap(),
        })
        .collect()
}

fn ways_to_win(race: Race) -> usize {
    let Race { time: t, record: r } = race;
    // we want to find the max range [a, b] where for each n in [a, b] we have n(t-n)>r
    // then, there are b-a+1 ways to win. And with b:=t-a, we have a(t-a)=ab=b(t-b),
    // so we only need to find a, at which point there are t-2a+1 ways to win

    // approximate the endpoints of [a, b] with the quadratic formula
    // n = (t +- sqrt(t^2 - 4 * r)) / 2

    let Some(radicand) = (t * t).checked_sub(4 * r) else {
        return 0;
    };

    let sqrt = radicand.isqrt();

    // saturate here because we don't care about negative solutions
    // intentionally undershoot the solution so we only need to scan forward
    // (we subtract 2 so the rounded division is always off by at least 1)
    let mut lo = t.saturating_sub(sqrt + 2) / 2;
    assert!(lo * (t - lo) <= r);
    while lo * (t - lo) <= r {
        lo += 1;
    }

    t - 2 * lo + 1
}

pub fn part1(input: &str) -> String {
    let races = parse_input_part1(input);

    races
        .iter()
        .map(|&r| ways_to_win(r))
        .product::<usize>()
        .to_string()
}

fn parse_input_part2(input: &str) -> Race {
    let mut lines = input.lines();
    let time_line = lines.next().and_then(|s| s.strip_prefix("Time:")).unwrap();
    let distance_line = lines
        .next()
        .and_then(|s| s.strip_prefix("Distance:"))
        .unwrap();

    let time = time_line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .fold(0usize, |acc, d| acc * 10 + d as usize);

    let record = distance_line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .fold(0usize, |acc, d| acc * 10 + d as usize);

    Race { time, record }
}

pub fn part2(input: &str) -> String {
    let race = parse_input_part2(input);
    ways_to_win(race).to_string()
}
