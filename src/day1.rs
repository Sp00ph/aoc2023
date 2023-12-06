pub fn part1(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|line| -> u32 {
            let first_digit = line
                .bytes()
                .find(|c| c.is_ascii_digit())
                .map(|c| (c - b'0') as u32)
                .expect("First digit not found");
            let last_digit = line
                .bytes()
                .rev()
                .find(|c| c.is_ascii_digit())
                .map(|c| (c - b'0') as u32)
                .expect("Last digit not found");
            first_digit * 10 + last_digit
        })
        .sum::<u32>()
        .to_string()
}

fn first_num(b: &[u8]) -> u32 {
    let mut it = b.iter();
    while let Some(&b) = it.next() {
        let s = it.as_slice();
        match b {
            b'1'..=b'9' => return (b - b'0') as u32,
            b'o' if s.starts_with(b"ne") => return 1,
            b't' => {
                if s.starts_with(b"wo") {
                    return 2;
                } else if s.starts_with(b"hree") {
                    return 3;
                }
            }
            b'f' => {
                if s.starts_with(b"our") {
                    return 4;
                } else if s.starts_with(b"ive") {
                    return 5;
                }
            }
            b's' => {
                if s.starts_with(b"ix") {
                    return 6;
                } else if s.starts_with(b"even") {
                    return 7;
                }
            }
            b'e' if s.starts_with(b"ight") => {
                return 8;
            }
            b'n' if s.starts_with(b"ine") => {
                return 9;
            }
            _ => {}
        }
    }
    panic!("First digit not found");
}

fn last_num(b: &[u8]) -> u32 {
    let mut it = b.iter();
    while let Some(&b) = it.next_back() {
        let s = it.as_slice();
        match b {
            b'1'..=b'9' => return (b - b'0') as u32,
            b'e' => {
                if s.ends_with(b"on") {
                    return 1;
                } else if s.ends_with(b"thre") {
                    return 3;
                } else if s.ends_with(b"fiv") {
                    return 5;
                } else if s.ends_with(b"nin") {
                    return 9;
                }
            }
            b'o' if s.ends_with(b"tw") => {
                return 2;
            }
            b'r' if s.ends_with(b"fou") => {
                return 4;
            }
            b'x' if s.ends_with(b"six") => {
                return 6;
            }
            b'n' if s.ends_with(b"seve") => {
                return 7;
            }
            b't' if s.ends_with(b"eight") => {
                return 8;
            }
            _ => {}
        }
    }
    panic!("First digit not found");
}

pub fn part2(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|line| {
            let first_digit = first_num(line.as_bytes());
            let last_digit = last_num(line.as_bytes());
            first_digit * 10 + last_digit
        })
        .sum::<u32>()
        .to_string()
}
