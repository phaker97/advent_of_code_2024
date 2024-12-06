//!
//! Usage `day1 --input-file <file> [--calc-diff]`
//!
//! If the `--calc-diff` flag is given, then the difference score will be calculated (task 1)
//! If not, then the similarity score will be calculated (task 2)
//!
//! The input file needs to consist of two columns of numbers separated by three spaces.
//! This is the format of the file in the advent of code.
//!
use std::fs;
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input_file: PathBuf,

    #[clap(long, short, action)]
    calc_diff: bool,
}


// Change these types according to the numbers in the input
type NumType = i32;
type ResultType = u32;

fn main() {
    let args = Args::parse();

    println!("Reading from {:#?}", &args.input_file.display());
    println!("Calculating {}", if args.calc_diff { "diff" } else { "similarity" });

    match fs::read_to_string(&args.input_file) {
        Ok(content) => {
            let (left_list, right_list) = create_lists(content);

            if args.calc_diff {
                println!("{}", calc_diff_score(&left_list, &right_list));
            } else {
                println!("{}", calc_sim_score(&left_list, &right_list));
            }

        }
        Err(e) => {
            eprintln!("Error reading file {}: {}", &args.input_file.display(), e);
        }
    }
}

/// inserts a value into a sorted vec at a correct place
fn insert<T: Ord>(vec: &mut Vec<T>, elem: T) {
    let pos = vec.binary_search(&elem).unwrap_or_else(|e| e);
    vec.insert(pos, elem);
}

/// Creates a list of two columns from a string
fn create_lists(content : String) -> (Vec<NumType>, Vec<NumType>) {
    let mut left_list: Vec<NumType> = Vec::new();
    let mut right_list: Vec<NumType> = Vec::new();

    for line in content.lines() {
        if line.trim().is_empty() {continue}
        let (l, r) = line.split_once("   ").unwrap();

        for (num, vec) in [(l, &mut left_list), (r, &mut right_list)] {
            let elem = num.parse().unwrap();
            insert(vec, elem);
        }
    }

    (left_list, right_list)
}

/// Calculates the difference according to this rule:
/// Always look at pairs (first left + first right, second left + second right etc.)
/// Calculate the absolute difference.
/// Summ the differences over all elements
///
/// If both slices are sorted, then this does exactly what task 1 of day 1 wants
fn calc_diff_score(left_list: &[NumType], right_list: &[NumType]) -> ResultType {
    left_list.iter().zip(right_list.iter()).map(|(left, right)| (left - right).abs() as ResultType).sum()
}

/// Calculates the similarity score in this way:
/// Multiply the elements from the left slice with how many times they appear in the right slice.
/// The slices need to be sorted, as this does binary search to find the first and the last element.
/// The difference in indices will be the count.
fn calc_sim_score(left_list: &[NumType], right_list: &[NumType]) -> ResultType {
    let mut result: ResultType = 0;
    for left in left_list {
        let start = right_list.partition_point(|x| x < left);
        let end = right_list.partition_point(|x| x <= left);
        let count = end - start;

        result += (count as ResultType) * (*left as ResultType);
    }

    result
}
