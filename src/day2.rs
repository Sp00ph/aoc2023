struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

struct Game {
    num: usize,
    rounds: Vec<Round>,
}

fn parse_round(s: &str) -> Round {
    let (mut red, mut green, mut blue) = (0, 0, 0);

    for part in s.split(", ") {
        let (num, color) = part.split_once(" ").unwrap();
        let num: usize = num.parse().unwrap();

        match color {
            "red" => red += num,
            "green" => green += num,
            "blue" => blue += num,
            _ => unreachable!(),
        }
    }

    Round { red, green, blue }
}

fn parse_game(line: &str) -> Game {
    let s = line.strip_prefix("Game ").unwrap();
    let (num, s) = s.split_once(": ").unwrap();
    let rounds = s.split("; ").map(parse_round).collect();

    Game {
        num: num.parse().unwrap(),
        rounds,
    }
}

fn is_game_possible(game: &Game, red: usize, green: usize, blue: usize) -> bool {
    game.rounds
        .iter()
        .all(|r| r.red <= red && r.green <= green && r.blue <= blue)
}

fn parse_games(input: &str) -> Vec<Game> {
    input.trim().lines().map(parse_game).collect()
}

pub fn part1(input: &str) -> String {
    let games = parse_games(input);
    games
        .iter()
        .filter(|g| is_game_possible(g, 12, 13, 14))
        .map(|g| g.num)
        .sum::<usize>()
        .to_string()
}

fn min_power(game: &Game) -> usize {
    let (red, green, blue) = game.rounds.iter().fold((0, 0, 0), |(red, green, blue), r| {
        (red.max(r.red), green.max(r.green), blue.max(r.blue))
    });

    red * green * blue
}

pub fn part2(input: &str) -> String {
    let games = parse_games(input);
    games.iter().map(min_power).sum::<usize>().to_string()
}
