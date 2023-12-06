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

fn ways_to_win(race: Race) -> impl Iterator<Item = usize> {
    (0..race.time).filter(move |t| t.saturating_mul(race.time - t) > race.record)
}

pub fn part1(input: &str) -> String {
    let races = parse_input_part1(input);

    races
        .iter()
        .map(|&r| ways_to_win(r).count())
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
    
    ways_to_win(race).count().to_string()
}
