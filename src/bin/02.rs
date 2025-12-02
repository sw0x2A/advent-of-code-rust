#![allow(unexpected_cfgs)]
#![warn(clippy::all, clippy::pedantic)]

use adv_code_2025::start_day;
use anyhow::{Ok, Result};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    #[allow(clippy::items_after_statements)]
    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut total = 0;
        for line in reader.lines().map_while(Result::ok) {
            for range in line.split(',') {
                let (start, end) = {
                    let mut parts = range.split('-');
                    (
                        parts.next().unwrap().parse::<usize>().unwrap(),
                        parts.next().unwrap().parse::<usize>().unwrap(),
                    )
                };
                for id in start..=end {
                    let s = id.to_string();
                    let len = s.len();
                    if len % 2 == 0 {
                        let (first, second) = s.split_at(len / 2);
                        if first == second {
                            total += id;
                        }
                    }
                }
            }
        }
        Ok(total)
    }

    assert_eq!(1_227_775_554, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    #[allow(clippy::items_after_statements)]
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut total = 0;
        for line in reader.lines().map_while(Result::ok) {
            for range in line.split(',') {
                let (start, end) = {
                    let mut parts = range.split('-');
                    (
                        parts.next().unwrap().parse::<usize>().unwrap(),
                        parts.next().unwrap().parse::<usize>().unwrap(),
                    )
                };
                for id in start..=end {
                    let s = id.to_string();
                    let len = s.len();
                    for sub_len in 1..=(len / 2) {
                        if len % sub_len != 0 {
                            continue;
                        }
                        let sub = &s[..sub_len];
                        let repeat_count = len / sub_len;
                        if repeat_count < 2 {
                            continue;
                        }
                        if sub.repeat(repeat_count) == s {
                            total += id;
                            break;
                        }
                    }
                }
            }
        }
        Ok(total)
    }

    assert_eq!(4_174_379_265, part2(BufReader::new(TEST.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    //endregion

    Ok(())
}
