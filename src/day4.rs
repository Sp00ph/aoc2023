use std::collections::BTreeMap;

struct Card {
    id: usize,
    // The input only seems to contain numbers up to 100, so we can use a
    // 128-bit integer as a bitset. This dramatically speeds up the intersection
    // counting compared to using a hashset, reducing the runtime by ~75-80%.
    // On the flipside, the challenge never explicitly states that the numbers
    // are in the range 1-100, so this solution is not guaranteed to work
    // for all inputs.
    winning: u128,
    nums: u128,
}

fn parse_card(line: &str) -> Card {
    let s = line.strip_prefix("Card ").unwrap();
    let (id, s) = s.split_once(':').unwrap();
    let (winning, nums) = s.split_once('|').unwrap();

    fn nums_to_bits(s: &str) -> u128 {
        s.trim()
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .fold(0u128, |acc, n| {
                acc | 1u128.checked_shl(n).expect("Can't handle 3-digit numbers")
            })
    }

    let winning = nums_to_bits(winning);
    let nums = nums_to_bits(nums);

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
            let winning_nums = (card.winning & card.nums).count_ones();
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
        let winning_nums = (card.winning & card.nums).count_ones() as usize;
        for i in 1..=winning_nums {
            queue.get_mut(&(id + i)).unwrap().1 += n;
        }
    }

    total.to_string()
}
