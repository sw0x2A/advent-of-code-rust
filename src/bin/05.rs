#![allow(unexpected_cfgs)]
#![warn(clippy::all, clippy::pedantic)]

use adv_code_2025::start_day;
use anyhow::{Ok, Result, anyhow};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    #[allow(clippy::items_after_statements)]
    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut lines = reader.lines().map_while(Result::ok);
        let ranges = parse_ranges(&mut lines)?;
        let ids: Vec<u64> = lines
            .map(|line| line.trim().parse::<u64>())
            .filter_map(Result::ok)
            .collect();
        let fresh_count = ids
            .iter()
            .filter(|id| ranges.iter().any(|(start, end)| *id >= start && *id <= end))
            .count();
        Ok(fresh_count)
    }

    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    #[allow(clippy::items_after_statements)]
    fn part2<R: BufRead>(reader: R) -> Result<u64> {
        let mut lines = reader.lines().map_while(Result::ok);
        let mut ranges = parse_ranges(&mut lines)?;
        ranges.sort_unstable();
        let mut merged: Vec<(u64, u64)> = Vec::new();
        for (start, end) in ranges {
            if let Some((_, last_end)) = merged.last_mut() {
                if start <= *last_end + 1 {
                    *last_end = (*last_end).max(end);
                } else {
                    merged.push((start, end));
                }
            } else {
                merged.push((start, end));
            }
        }
        let total = merged.iter().map(|(start, end)| end - start + 1).sum();
        Ok(total)
    }

    assert_eq!(14, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    //endregion

    Ok(())
}

fn parse_ranges(mut lines: impl Iterator<Item = String>) -> Result<Vec<(u64, u64)>> {
    let mut ranges = Vec::new();
    for line in &mut lines {
        if line.trim().is_empty() {
            break;
        }
        let (start, end) = line
            .split_once('-')
            .and_then(|(a, b)| Some((a.trim().parse::<u64>().ok()?, b.trim().parse::<u64>().ok()?)))
            .ok_or_else(|| anyhow!("Invalid range line: {line}"))?;
        ranges.push((start, end));
    }
    Ok(ranges)
}
