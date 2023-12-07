
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct CardIdx(u8);

impl CardIdx {
    fn from_byte_part1(b: u8) -> Self {
        match b {
            b'2'..=b'9' => Self(b - b'2'),
            b'T' => Self(8),
            b'J' => Self(9),
            b'Q' => Self(10),
            b'K' => Self(11),
            b'A' => Self(12),
            _ => panic!("invalid card byte: {b}"),
        }
    }

    fn from_byte_part2(b: u8) -> Self {
        match b {
            b'J' => Self(0),
            b'2'..=b'9' => Self(b - b'1'),
            b'T' => Self(9),
            b'Q' => Self(10),
            b'K' => Self(11),
            b'A' => Self(12),
            _ => panic!("invalid card byte: {b}"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct Hand {
    typ: HandType,
    cards: [CardIdx; 5],
}

impl Hand {
    fn new(cards: [CardIdx; 5], part2: bool) -> Self {
        let typ = if !part2 {
            Self::determine_type_part1(cards)
        } else {
            Self::determine_type_part2(cards)
        };
        Self { cards, typ }
    }

    fn determine_type_part1(cards: [CardIdx; 5]) -> HandType {
        let mut count = [0u8; 13];
        for &CardIdx(idx) in &cards {
            count[idx as usize] += 1;
        }

        count.sort_unstable_by(|a, b| b.cmp(a));

        match count {
            [5, ..] => HandType::FiveOfAKind,
            [4, ..] => HandType::FourOfAKind,
            [3, 2, ..] => HandType::FullHouse,
            [3, ..] => HandType::ThreeOfAKind,
            [2, 2, ..] => HandType::TwoPair,
            [2, ..] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }

    fn determine_type_part2(cards: [CardIdx; 5]) -> HandType {
        let mut count = [0u8; 13];
        for &CardIdx(idx) in &cards {
            count[idx as usize] += 1;
        }
        let jokers = count[0] as usize;
        if jokers == 5 {
            return HandType::FiveOfAKind;
        }
        let rest = &mut count[1..];
        rest.sort_unstable_by(|a, b| b.cmp(a));

        match rest {
            [5, ..] => HandType::FiveOfAKind,
            [4, ..] => [HandType::FourOfAKind, HandType::FiveOfAKind][jokers],
            [3, 2, ..] => HandType::FullHouse,
            [3, ..] => [
                HandType::ThreeOfAKind,
                HandType::FourOfAKind,
                HandType::FiveOfAKind,
            ][jokers],
            [2, 2, ..] => [HandType::TwoPair, HandType::FullHouse][jokers],
            [2, ..] => [
                HandType::OnePair,
                HandType::ThreeOfAKind,
                HandType::FourOfAKind,
                HandType::FiveOfAKind,
            ][jokers],
            _ => [
                HandType::HighCard,
                HandType::OnePair,
                HandType::ThreeOfAKind,
                HandType::FourOfAKind,
                HandType::FiveOfAKind,
            ][jokers],
        }
    }
}

fn parse_line(line: &str, part2: bool) -> (Hand, usize) {
    let (hand, bid) = line.trim().split_once(' ').unwrap();
    let hand: [u8; 5] = hand.as_bytes().try_into().unwrap();
    let card_fn = if part2 {
        CardIdx::from_byte_part2
    } else {
        CardIdx::from_byte_part1
    };
    let cards = hand.map(card_fn);
    (Hand::new(cards, part2), bid.parse().unwrap())
}

fn parse_input(input: &str, part2: bool) -> Vec<(Hand, usize)> {
    input.trim().lines().map(|l| parse_line(l, part2)).collect()
}

pub fn part1(input: &str) -> String {
    let mut hands = parse_input(input, false);
    hands.sort_unstable();
    hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let mut hands = parse_input(input, true);
    hands.sort_unstable();
    hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum::<usize>()
        .to_string()
}
