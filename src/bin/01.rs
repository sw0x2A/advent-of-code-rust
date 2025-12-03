#![allow(unexpected_cfgs)]
#![warn(clippy::all, clippy::pedantic)]

use adv_code_2025::start_day;
use anyhow::{Ok, Result, bail};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    #[allow(clippy::items_after_statements)]
    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut dial = 50;
        let mut count = 0;
        for line in reader.lines().map_while(Result::ok) {
            let (dir, dist) = line.split_at(1);
            let dist: i32 = dist.parse()?;
            match dir {
                "L" => dial = (dial + 100 - dist) % 100,
                "R" => dial = (dial + dist) % 100,
                _ => bail!("Invalid direction"),
            }
            if dial == 0 {
                count += 1;
            }
        }
        Ok(count)
    }

    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    #[allow(clippy::items_after_statements)]
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut dial = 50;
        let mut count = 0;
        for line in reader.lines().map_while(Result::ok) {
            let (dir, dist) = line.split_at(1);
            let dist: i32 = dist.parse()?;
            match dir {
                "L" => {
                    for _ in 0..dist {
                        dial = (dial + 99) % 100;
                        if dial == 0 {
                            count += 1;
                        }
                    }
                }
                "R" => {
                    for _ in 0..dist {
                        dial = (dial + 1) % 100;
                        if dial == 0 {
                            count += 1;
                        }
                    }
                }
                _ => bail!("Invalid direction"),
            }
        }
        Ok(count)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    //endregion

    Ok(())
}
