//! Day two of advent of code
//!
//! You get a list of lists of numbers (one list per line)
//! Find out how many lines are either _ascending_ or _descending_.
//! No matter which of the two, the difference of two adjacent elements must be at least one and at most 3.
//!
//! Example:
//! ```text
//! 7 6 4 2 1  // safe
//! 1 2 7 8 9  // unsafe 2->7 +5
//! 9 7 6 2 1  // unsafe 6->2 -4
//! 1 3 2 4 5  // unsafe 1->3 ascending but 3->2 descending
//! 8 6 4 4 1  // unsafe 4->4 +0
//! 1 3 6 7 9  // safe
//! ```
//!
//! (Task 2)
//! The dampener parameter says how many violations are okay for something to be considered safe
//!

use clap::Parser;
use std::path::PathBuf;

type NumType = i32;
type ResultType = usize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: PathBuf,

    #[clap(short, long, default_value = "false")]
    dampen: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Reading file {}.", args.file_name.display());
    println!("Status dampening is {}.", if args.dampen { "on" } else { "off" });

    let content = std::fs::read_to_string(&args.file_name)?;
    let lines = read_lists(content);


    let answer = if args.dampen {
        check_lists_dampended(&lines)
    } else {
        check_lists(&lines)
    };
    println!("{}", answer);

    Ok(())
}

fn read_lists(content: String) -> Vec<Vec<NumType>> {
    content
        .lines()
        .map(|line| {
            line.split(' ')
                .filter_map(|s| s.parse::<NumType>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn check_lists_dampended(lists: &[Vec<NumType>]) -> ResultType {
    lists
        .iter()
        .filter(|list| {
            if !check_list(list.iter().copied()) {
                (0..list.len()).into_iter().filter(|i| {
                    check_list(list.into_iter().enumerate().filter_map(|(index, value)| {
                        if index == *i {
                            None
                        } else {
                            Some(*value)
                        }
                    }))
                }).next().is_some()
            } else {
                true
            }
        })
        .count()
}
fn check_lists(lists: &[Vec<NumType>]) -> ResultType {
    lists.iter().filter(|list| check_list(list.iter().copied())).count() as ResultType
}

fn check_list<I: IntoIterator<Item = NumType>>(list: I) -> bool {
    let mut iter = list.into_iter();

    if let Some(mut last) = iter.next() {
        let mut ascending: bool = true;
        let mut descending: bool = true;

        for num in iter {
            let diff = last - num;
            if 0 == diff || diff.abs() > 3 {
                return false;
            }
            if diff < 0 {
                descending = false;
            }
            if diff > 0 {
                ascending = false;
            }
            if !ascending && !descending {
                return false;
            }
            last = num;
        }
        true
    } else {
        true
    }
}
