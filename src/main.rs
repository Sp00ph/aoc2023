#![feature(isqrt)]

use std::time::{Duration, Instant};

use anyhow::Context;
use clap::Parser;
use seq_macro::seq;

seq!(N in 1..=25 {
    mod day~N;
});

seq!(N in 1..=25 {
    #[used]
    static FNS: [[fn(&str) -> String; 2]; 25] = [
        #(
            [day~N::part1, day~N::part2],
        )*
    ];
});

#[derive(Parser)]
enum Args {
    #[clap(alias = "rp")]
    RunPart {
        day: usize,
        part: usize,
        #[arg(short, long)]
        input: Option<String>,
        #[arg(short = 't', long)]
        show_time: bool,
    },
    #[clap(alias = "rd")]
    RunDay {
        day: usize,
        #[clap(short, long)]
        input: Option<String>,
        #[arg(short = 't', long)]
        show_time: bool,
        #[arg(short = 'T', long)]
        show_total_time: bool,
    },
    #[clap(alias = "ra")]
    RunAll {
        #[arg(short = 't', long)]
        show_time: bool,
        #[arg(short = 'T', long)]
        show_total_time: bool,
    },
}

fn run_part(
    day: usize,
    part: usize,
    input: Option<String>,
    show_time: bool,
    acc: Option<&mut Duration>,
) -> anyhow::Result<()> {
    let input = match input {
        Some(input) => input,
        None => std::fs::read_to_string(format!("input/day{}.txt", day))
            .context("Input for this day isn't available.")?,
    };
    let fns = &FNS[day - 1];
    let now = Instant::now();
    let output = fns[part - 1](&input);
    let elapsed = now.elapsed();
    println!("===== Day {} Part {} =====", day, part);
    println!("{}", output);
    if show_time {
        println!("Finished in: {:.3?}", elapsed);
    }
    if let Some(acc) = acc {
        *acc += elapsed;
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args {
        Args::RunPart {
            day,
            part,
            input,
            show_time,
        } => run_part(day, part, input, show_time, None),
        Args::RunDay {
            day,
            input,
            show_time,
            show_total_time,
        } => {
            let mut acc = show_total_time.then(|| Duration::ZERO);

            run_part(day, 1, input.clone(), show_time, acc.as_mut())?;
            run_part(day, 2, input, show_time, acc.as_mut())?;

            if let Some(acc) = acc {
                println!("Total time: {:.3?}", acc);
            }
            Ok(())
        }
        Args::RunAll {
            show_time,
            show_total_time,
        } => {
            let mut acc = show_total_time.then(|| Duration::ZERO);
            for day in 1..=25 {
                run_part(day, 1, None, show_time, acc.as_mut())?;
                run_part(day, 2, None, show_time, acc.as_mut())?;
            }

            if let Some(acc) = acc {
                println!("Total time: {:.3?}", acc);
            }
            Ok(())
        }
    }
}
