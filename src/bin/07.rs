#![allow(unexpected_cfgs)]
#![warn(clippy::all, clippy::pedantic)]

use adv_code_2025::start_day;
use anyhow::{Ok, Result};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    #[allow(clippy::items_after_statements)]
    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let grid: Vec<Vec<char>> = reader
            .lines()
            .map_while(Result::ok)
            .map(|line| line.chars().collect())
            .collect();
        let height = grid.len();
        let width = grid[0].len();
        let start_x = grid[0].iter().position(|&c| c == 'S').expect("No S found");
        let mut split_positions = HashSet::new();
        let mut queue = vec![(start_x, 1)];
        while let Some((x, y)) = queue.pop() {
            if y >= height {
                continue;
            }
            match grid[y][x] {
                '^' => {
                    if split_positions.insert((x, y)) {
                        if x > 0 {
                            queue.push((x - 1, y + 1));
                        }
                        if x + 1 < width {
                            queue.push((x + 1, y + 1));
                        }
                    }
                }
                '.' | 'S' => {
                    queue.push((x, y + 1));
                }
                _ => {}
            }
        }
        Ok(split_positions.len())
    }

    assert_eq!(21, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    #[allow(clippy::items_after_statements)]
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let grid: Vec<Vec<char>> = reader
            .lines()
            .map_while(Result::ok)
            .map(|line| line.chars().collect())
            .collect();
        let start_x = grid[0].iter().position(|&c| c == 'S').expect("No S found");
        fn dfs(
            x: usize,
            y: usize,
            grid: &Vec<Vec<char>>,
            memo: &mut HashMap<(usize, usize), usize>,
        ) -> usize {
            if y == grid.len() - 1 {
                return 1;
            }
            if let Some(&cached) = memo.get(&(x, y)) {
                return cached;
            }
            let res = match grid[y][x] {
                '^' => {
                    let mut sum = 0;
                    if x > 0 {
                        sum += dfs(x - 1, y + 1, grid, memo);
                    }
                    if x + 1 < grid[0].len() {
                        sum += dfs(x + 1, y + 1, grid, memo);
                    }
                    sum
                }
                '.' | 'S' => dfs(x, y + 1, grid, memo),
                _ => 0,
            };
            memo.insert((x, y), res);
            res
        }
        let mut memo = HashMap::new();
        let result = dfs(start_x, 1, &grid, &mut memo);
        Ok(result)
    }

    assert_eq!(40, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    //endregion

    Ok(())
}
