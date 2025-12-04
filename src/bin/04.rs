#![allow(unexpected_cfgs)]
#![warn(clippy::all, clippy::pedantic)]

use adv_code_2025::start_day;
use anyhow::{Ok, Result};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    #[allow(clippy::items_after_statements)]
    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let grid = parse_grid(reader);
        let h = grid.len();
        let w = grid[0].len();
        let mut accessible = 0;
        for y in 0..h {
            for x in 0..w {
                if grid[y][x] == b'@' && adjacent_count(&grid, y, x) < 4 {
                    accessible += 1;
                }
            }
        }
        Ok(accessible)
    }

    assert_eq!(13, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    #[allow(clippy::items_after_statements)]
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut grid = parse_grid(reader);
        let h = grid.len();
        let w = grid[0].len();
        let mut total_removed = 0;
        loop {
            let mut to_remove = Vec::new();
            for y in 0..h {
                for x in 0..w {
                    if grid[y][x] == b'@' && adjacent_count(&grid, y, x) < 4 {
                        to_remove.push((y, x));
                    }
                }
            }
            if to_remove.is_empty() {
                break;
            }
            for (y, x) in to_remove {
                grid[y][x] = b'.';
                total_removed += 1;
            }
        }
        Ok(total_removed)
    }

    assert_eq!(43, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    //endregion

    Ok(())
}

fn parse_grid<R: BufRead>(reader: R) -> Vec<Vec<u8>> {
    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.bytes().collect())
        .collect()
}

#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_sign_loss)]
fn adjacent_count(grid: &[Vec<u8>], y: usize, x: usize) -> usize {
    let h = grid.len() as isize;
    let w = grid[0].len() as isize;
    let mut adj = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dy == 0 && dx == 0 {
                continue;
            }
            let ny = y as isize + dy;
            let nx = x as isize + dx;
            if ny >= 0 && ny < h && nx >= 0 && nx < w && grid[ny as usize][nx as usize] == b'@' {
                adj += 1;
            }
        }
    }
    adj
}
