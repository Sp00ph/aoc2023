
pub fn part1(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|line| -> u32 {
            let first_digit = line
                .chars()
                .find_map(|c| c.to_digit(10))
                .expect("First digit not found");
            let last_digit = line
                .chars()
                .rev()
                .find_map(|c| c.to_digit(10))
                .expect("Last digit not found");
            first_digit * 10 + last_digit
        })
        .sum::<u32>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let nums = [
        ('0', "zero"),
        ('1', "one"),
        ('2', "two"),
        ('3', "three"),
        ('4', "four"),
        ('5', "five"),
        ('6', "six"),
        ('7', "seven"),
        ('8', "eight"),
        ('9', "nine"),
    ];
    input
        .trim()
        .lines()
        .map(|line| {
            let digit = |s: &str| {
                nums.iter()
                    .position(|&(num, word)| s.starts_with(num) || s.starts_with(word))
            };

            let first_digit = line
                .char_indices()
                .find_map(|(i, _)| digit(&line[i..]))
                .expect("First digit not found") as u32;
            let last_digit = line
                .char_indices()
                .rev()
                .find_map(|(i, _)| digit(&line[i..]))
                .expect("Last digit not found") as u32;

            first_digit * 10 + last_digit
        })
        .sum::<u32>()
        .to_string()
}
