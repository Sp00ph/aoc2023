use ahash::AHashMap;
use enum_map::{enum_map, Enum, EnumMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Less,
    Greater,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rule<'a> {
    category: Category,
    op: Op,
    value: usize,
    goto: &'a str,
}

impl Rule<'_> {
    fn matches(self, part: &Part) -> bool {
        match self.op {
            Op::Less => part[self.category] < self.value,
            Op::Greater => part[self.category] > self.value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
    fallback: &'a str,
}

fn parse_workflow(line: &str) -> Workflow<'_> {
    let (name, rest) = line.split_once('{').unwrap();
    let mut rules = rest.strip_suffix('}').unwrap().split(',');
    let fallback = rules.next_back().unwrap();
    let rules = rules
        .map(|rule| {
            let (category, rest) = rule.split_at(1);
            let (op, rest) = rest.split_at(1);
            let (value, goto) = rest.split_once(':').unwrap();
            let value: usize = value.parse().unwrap();
            let category = match category {
                "x" => Category::X,
                "m" => Category::M,
                "a" => Category::A,
                "s" => Category::S,
                _ => unreachable!("invalid category"),
            };
            let op = match op {
                "<" => Op::Less,
                ">" => Op::Greater,
                _ => unreachable!("invalid operator"),
            };
            Rule {
                category,
                op,
                value,
                goto,
            }
        })
        .collect();

    Workflow {
        name,
        rules,
        fallback,
    }
}

type Part = EnumMap<Category, usize>;

fn parse_part(line: &str) -> Part {
    let line = line.strip_prefix("{x=").unwrap();
    let (x, rest) = line.split_once(",m=").unwrap();
    let (m, rest) = rest.split_once(",a=").unwrap();
    let (a, rest) = rest.split_once(",s=").unwrap();
    let s = rest.strip_suffix('}').unwrap();
    enum_map! {
        Category::X => x.parse().unwrap(),
        Category::M => m.parse().unwrap(),
        Category::A => a.parse().unwrap(),
        Category::S => s.parse().unwrap(),
    }
}

type WorkflowMap<'a> = AHashMap<&'a str, Workflow<'a>>;

fn parse_input(input: &str) -> (WorkflowMap<'_>, Vec<Part>) {
    let mut lines = input.lines();
    let workflows = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(parse_workflow)
        .map(|workflow| (workflow.name, workflow))
        .collect();
    let parts = lines.map(parse_part).collect();
    (workflows, parts)
}

pub fn part1(input: &str) -> String {
    let (workflows, parts) = parse_input(input);

    let mut total = 0;
    'outer: for part in parts {
        let mut workflow = &workflows["in"];
        loop {
            // Find the first rule that matches the part, or go to the fallback.
            let next = workflow
                .rules
                .iter()
                .find(|rule| rule.matches(&part))
                .map(|rule| rule.goto)
                .unwrap_or(workflow.fallback);

            match next {
                "A" => {
                    total += part[Category::X]
                        + part[Category::M]
                        + part[Category::A]
                        + part[Category::S];
                    continue 'outer;
                }
                "R" => continue 'outer,
                next => workflow = &workflows[next],
            }
        }
    }

    total.to_string()
}

type Ranges = EnumMap<Category, (usize, usize)>;

/// Tries to split the ranges into two parts, one that fits the rule,
/// and one that doesn't. If either of the parts is empty, it returns None.
fn split_ranges(ranges: Ranges, rule: Rule) -> Option<(Ranges, Ranges)> {
    let (min, max) = ranges[rule.category];
    if rule.op == Op::Greater && max > rule.value {
        // Part of the ranges that fits the rule.
        let mut inside = ranges;
        inside[rule.category].0 = rule.value + 1;
        // Part of the ranges that doesn't fit the rule.
        let mut outside = ranges;
        outside[rule.category].1 = rule.value;

        Some((inside, outside))
    } else if rule.op == Op::Less && min < rule.value {
        let mut inside = ranges;
        inside[rule.category].1 = rule.value - 1;

        let mut outside = ranges;
        outside[rule.category].0 = rule.value;

        Some((inside, outside))
    } else {
        // No overlap between the ranges and the rule, so return None.
        None
    }
}

/// Counts the number of values in the ranges.
/// e.g. for the full range, this would be 4000**4.
fn ranges_size(ranges: &Ranges) -> usize {
    ranges.values().map(|&(min, max)| max + 1 - min).product()
}

pub fn part2(input: &str) -> String {
    // Recursively calculate the number of valid parts for the workflow `node`,
    // This can be done using a simple DFS, because the input is just
    // a tree of rules. The `ranges` parameter is used to constrain
    // the valid values for each category in lower levels of the tree.
    fn rec(workflows: &WorkflowMap, node: &str, mut ranges: Ranges) -> usize {
        let mut total = 0;
        let w = &workflows[node];

        for &rule in &w.rules {
            // Only process the rules that actually overlap the range.
            if let Some((inside, outside)) = split_ranges(ranges, rule) {
                // The current rule already processes all of `inside`,
                // so the next rules should only process `outside` to prevent
                // duplicates.
                ranges = outside;
                // If the rule goes to "A", accept the entire range.
                // If it goes to "R", reject the entire range.
                // Otherwise, recurse into the next workflow.
                if rule.goto == "A" {
                    total += ranges_size(&inside);
                } else if rule.goto != "R" {
                    total += rec(workflows, rule.goto, inside);
                }
            }
        }
        // At this point, what's left in `ranges` will all
        // be sent to the fallback, so we can handle it as
        // a sort of unconditional rule.
        if w.fallback == "A" {
            total += ranges_size(&ranges);
        } else if w.fallback != "R" {
            total += rec(workflows, w.fallback, ranges);
        }

        total
    }

    let (workflows, _) = parse_input(input);

    rec(
        &workflows,
        "in",
        enum_map! {
            Category::X => (1, 4000),
            Category::M => (1, 4000),
            Category::A => (1, 4000),
            Category::S => (1, 4000),
        },
    )
    .to_string()
}
