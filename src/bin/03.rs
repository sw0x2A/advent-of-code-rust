#![allow(unexpected_cfgs)]
#![warn(clippy::all, clippy::pedantic)]

use adv_code_2025::start_day;
use anyhow::{Ok, Result};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    #[allow(clippy::items_after_statements)]
    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        let mut total = 0u64;
        for line in reader.lines().map_while(Result::ok) {
            let digits: Vec<u8> = line.bytes().filter(u8::is_ascii_digit).collect();
            let mut max_joltage = 0;
            for i in 0..digits.len() {
                for j in i + 1..digits.len() {
                    let joltage = (digits[i] - b'0') * 10 + (digits[j] - b'0');
                    if joltage > max_joltage {
                        max_joltage = joltage;
                    }
                }
            }
            total += u64::from(max_joltage);
        }
        Ok(total)
    }

    assert_eq!(357, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    #[allow(clippy::items_after_statements)]
    fn part2<R: BufRead>(reader: R) -> Result<u64> {
        let mut total = 0u64;
        for line in reader.lines().map_while(Result::ok) {
            let digits: Vec<u8> = line.bytes().filter(u8::is_ascii_digit).collect();
            let k = 12;
            let mut stack = Vec::with_capacity(k);
            let mut to_remove = digits.len().saturating_sub(k);
            for &d in &digits {
                while !stack.is_empty()
                    && to_remove > 0
                    && stack.last().unwrap() < &d
                    && stack.len() + digits.len() - stack.len() > k
                {
                    stack.pop();
                    to_remove -= 1;
                }
                if stack.len() < k {
                    stack.push(d);
                } else {
                    to_remove -= 1;
                }
            }
            let joltage = stack
                .iter()
                .fold(0u64, |acc, &d| acc * 10 + u64::from(d - b'0'));
            total += joltage;
        }
        Ok(total)
    }

    assert_eq!(3_121_910_778_619, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    //endregion

    Ok(())
}
