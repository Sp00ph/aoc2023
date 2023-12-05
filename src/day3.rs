use std::ops::Range;

#[derive(Debug)]
struct Number {
    value: usize,
    line: usize,
    column: usize,
    length: usize,
}

#[derive(Debug)]
struct Symbol {
    ch: char,
    line: usize,
    column: usize,
}

#[derive(Debug)]
struct Line {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

fn parse_line(line: &str, line_number: usize) -> Line {
    let mut s = line;
    let mut i = 0;
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    while !s.is_empty() {
        let Some(j) = s.find(|ch| ch != '.') else {
            break;
        };
        i += j;
        s = &s[j..];
        let ch = s.chars().next().unwrap();
        if ch.is_numeric() {
            let end = s.find(|ch: char| !ch.is_numeric()).unwrap_or(s.len());
            let number = s[..end].parse().unwrap();
            numbers.push(Number {
                value: number,
                line: line_number,
                column: i,
                length: end,
            });
            s = &s[end..];
            i += end;
        } else {
            symbols.push(Symbol {
                ch,
                line: line_number,
                column: i,
            });
            s = &s[1..];
            i += 1;
        }
    }

    Line { numbers, symbols }
}

fn parse_input(input: &str) -> Vec<Line> {
    input
        .trim()
        .lines()
        .enumerate()
        .map(|(i, line)| parse_line(line, i))
        .collect()
}

fn num_neighbors_symbol(grid: &[Line], number: &Number) -> bool {
    let above = &grid[number.line.saturating_sub(1)];
    let line = &grid[number.line];
    let below = grid.get(number.line + 1);
    let range = (number.column.saturating_sub(1))..(number.column + number.length + 1);

    above.symbols.iter()
        .chain(&line.symbols)
        .chain(below.map(|line| &line.symbols).into_iter().flatten())
        .any(|s| range.contains(&s.column))
}

pub fn part1(input: &str) -> String {
    let grid = parse_input(input);
    let nums = grid.iter().flat_map(|line| &line.numbers);
    nums.filter(|num| num_neighbors_symbol(&grid, num))
        .map(|num| num.value)
        .sum::<usize>()
        .to_string()
}

fn gear_ratio(grid: &[Line], symbol: &Symbol) -> Option<usize> {
    if symbol.ch != '*' {
        return None;
    }
    
    let above = &grid[symbol.line.saturating_sub(1)];
    let line = &grid[symbol.line];
    let below = grid.get(symbol.line + 1);
    let range = (symbol.column.saturating_sub(1))..(symbol.column + 2);

    fn overlaps(lhs: &Range<usize>, rhs: &Range<usize>) -> bool {
        lhs.start < rhs.end && rhs.start < lhs.end
    }

    let mut nums = above
        .numbers
        .iter()
        .chain(&line.numbers)
        .chain(below.map(|line| &line.numbers).into_iter().flatten())
        .filter(|n| overlaps(&range, &(n.column..(n.column + n.length))));

    let lhs = nums.next()?;
    let rhs = nums.next()?;
    if nums.next().is_some() {
        return None;
    }

    Some(lhs.value * rhs.value)
}

pub fn part2(input: &str) -> String {
    let grid = parse_input(input);

    let symbols = grid.iter().flat_map(|line| &line.symbols);
    symbols.filter_map(|symbol| gear_ratio(&grid, symbol))
        .sum::<usize>()
        .to_string()
}
