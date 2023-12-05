use std::collections::BTreeMap;

use ahash::AHashSet;

struct Card {
    id: usize,
    winning: AHashSet<usize>,
    nums: AHashSet<usize>,
}

fn parse_card(line: &str) -> Card {
    let s = line.strip_prefix("Card ").unwrap();
    let (id, s) = s.split_once(':').unwrap();
    let (winning, nums) = s.split_once('|').unwrap();
    let winning = winning
        .trim()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    let nums = nums
        .trim()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    Card {
        id: id.trim().parse().unwrap(),
        winning,
        nums,
    }
}

fn parse_input(input: &str) -> Vec<Card> {
    input.lines().map(parse_card).collect()
}

pub fn part1(input: &str) -> String {
    let cards = parse_input(input);

    cards
        .iter()
        .map(|card| {
            let winning_nums = card.winning.intersection(&card.nums).count();
            if winning_nums == 0 {
                0
            } else {
                1 << (winning_nums - 1)
            }
        })
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let cards = parse_input(input);
    let mut queue = BTreeMap::from_iter(cards.into_iter().map(|card| (card.id, (card, 1usize))));

    let mut total = 0;

    while let Some((id, (card, n))) = queue.pop_first() {
        total += n;
        let winning_nums = card.winning.intersection(&card.nums).count();
        for i in 1..=winning_nums {
            queue.get_mut(&(id + i)).unwrap().1 += n;
        }
    }

    total.to_string()
}
