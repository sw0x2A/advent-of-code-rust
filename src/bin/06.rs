#![allow(unexpected_cfgs)]
#![warn(clippy::all, clippy::pedantic)]

use adv_code_2025::start_day;
use anyhow::{Context, Result};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

#[allow(clippy::too_many_lines)]
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    #[allow(clippy::items_after_statements)]
    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines: Vec<String> = reader.lines().map_while(Result::ok).collect();
        if lines.is_empty() {
            return Ok(0);
        }
        let height = lines.len();
        let width = lines.iter().map(String::len).max().unwrap_or(0);
        let grid: Vec<Vec<char>> = lines
            .into_iter()
            .map(|mut l| {
                l.extend(std::iter::repeat_n(' ', width - l.len()));
                l.chars().collect::<Vec<_>>()
            })
            .collect();
        let mut columns: Vec<Vec<char>> = vec![vec![' '; height]; width];
        for (y, row) in grid.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                columns[x][y] = c;
            }
        }
        let mut problems: Vec<Vec<Vec<char>>> = Vec::new();
        let mut current: Vec<Vec<char>> = Vec::new();
        let mut in_space = false;
        for col in columns {
            if col.iter().all(|&c| c == ' ') {
                if !in_space {
                    if !current.is_empty() {
                        problems.push(current);
                        current = Vec::new();
                    }
                    in_space = true;
                }
            } else {
                current.push(col);
                in_space = false;
            }
        }
        if !current.is_empty() {
            problems.push(current);
        }
        let mut total = 0;
        for problem in problems {
            let h = problem[0].len();
            let mut rows: Vec<String> = vec![String::new(); h];
            for col in &problem {
                for y in 0..h {
                    rows[y].push(col[y]);
                }
            }
            let op_row = rows.last().unwrap().trim();
            let op = op_row
                .chars()
                .find(|&c| c == '+' || c == '*')
                .ok_or_else(|| anyhow::anyhow!("No operator found in problem"))?;
            let numbers: Vec<usize> = rows[..h - 1]
                .iter()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(str::parse::<usize>)
                .collect::<Result<Vec<_>, _>>()?;
            if numbers.is_empty() {
                return Err(anyhow::anyhow!("No numbers found in problem"));
            }
            let result = match op {
                '+' => numbers.iter().copied().reduce(|a, b| a + b).unwrap(),
                '*' => numbers.iter().copied().reduce(|a, b| a * b).unwrap(),
                _ => unreachable!(),
            };
            total += result;
        }
        Ok(total)
    }

    assert_eq!(4_277_556, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    #[allow(clippy::items_after_statements)]
    #[allow(clippy::needless_range_loop)]
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines: Vec<String> = reader.lines().map_while(Result::ok).collect();
        if lines.is_empty() {
            return Ok(0);
        }

        let height = lines.len();
        let width = lines.iter().map(String::len).max().unwrap_or(0);

        let grid: Vec<Vec<char>> = lines
            .iter()
            .map(|l| {
                let mut chars: Vec<char> = l.chars().collect();
                chars.resize(width, ' ');
                chars
            })
            .collect();

        let mut separator_indices = vec![];
        for x in 0..width {
            let is_empty = (0..height).all(|y| grid[y][x].is_whitespace());
            if is_empty {
                separator_indices.push(x);
            }
        }

        let mut blocks = Vec::new();
        let mut in_block = false;
        let mut start_col = 0;

        for x in 0..width {
            let is_separator = separator_indices.contains(&x);
            if !in_block && !is_separator {
                in_block = true;
                start_col = x;
            } else if in_block && is_separator {
                in_block = false;
                blocks.push(start_col..x);
            }
        }
        if in_block {
            blocks.push(start_col..width);
        }

        let mut grand_total: usize = 0;

        for range in blocks {
            let mut operator = ' ';
            for x in range.clone() {
                let c = grid[height - 1][x];
                if !c.is_whitespace() {
                    operator = c;
                    break;
                }
            }
            if operator == ' ' {
                continue;
            }
            let mut numbers = Vec::new();
            for x in range.clone() {
                let mut num_str = String::new();
                for y in 0..height - 1 {
                    let c = grid[y][x];
                    if !c.is_whitespace() {
                        num_str.push(c);
                    }
                }
                if !num_str.is_empty() {
                    let num = num_str.parse::<usize>().with_context(|| {
                        format!("Failed to parse number string '{num_str}' in col {x}")
                    })?;
                    numbers.push(num);
                }
            }
            match operator {
                '+' => {
                    grand_total += numbers.iter().sum::<usize>();
                }
                '*' => {
                    if !numbers.is_empty() {
                        let product = numbers.iter().product::<usize>();
                        grand_total += product;
                    }
                }
                _ => {
                    eprintln!("Unknown operator found: {operator}");
                }
            }
        }
        Ok(grand_total)
    }

    assert_eq!(3_263_827, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    //endregion

    Ok(())
}
