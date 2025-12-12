#![allow(unexpected_cfgs)]
#![warn(clippy::all, clippy::pedantic)]

use adv_code_2025::start_day;
use anyhow::{Ok, Result};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    r: i32,
    c: i32,
}

#[derive(Clone, Debug)]
struct Variation {
    points: Vec<Point>,
    width: usize,
    height: usize,
}

#[derive(Clone, Debug)]
struct Shape {
    variations: Vec<Variation>,
    area: usize,
}

struct Query {
    w: usize,
    h: usize,
    presents: Vec<usize>,
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    #[allow(clippy::items_after_statements)]
    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines: Vec<String> = reader.lines().map_while(Result::ok).collect();
        let mut shapes = HashMap::new();
        let mut queries = Vec::new();

        let mut current_id = None;
        let mut current_rows = Vec::new();

        for line in lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            if trimmed.contains(':') {
                let parts: Vec<&str> = trimmed.split(':').collect();
                let header = parts[0].trim();

                if header.contains('x') {
                    if let Some(id) = current_id {
                        shapes.insert(id, parse_shape(&current_rows));
                        current_id = None;
                        current_rows.clear();
                    }

                    let dim_parts: Vec<&str> = header.split('x').collect();
                    let w: usize = dim_parts[0].parse()?;
                    let h: usize = dim_parts[1].parse()?;

                    let counts_str = parts[1].trim();
                    let counts: Vec<usize> = counts_str
                        .split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect();

                    let mut presents = Vec::new();
                    for (idx, &count) in counts.iter().enumerate() {
                        for _ in 0..count {
                            presents.push(idx);
                        }
                    }
                    queries.push(Query { w, h, presents });
                } else {
                    if let Some(id) = current_id {
                        shapes.insert(id, parse_shape(&current_rows));
                        current_rows.clear();
                    }
                    current_id = Some(header.parse()?);
                }
            } else {
                current_rows.push(trimmed.to_string());
            }
        }
        if let Some(id) = current_id {
            shapes.insert(id, parse_shape(&current_rows));
        }

        let mut solved_count = 0;
        for q in queries {
            let mut presents = q.presents.clone();
            presents.sort_by(|a, b| shapes[b].area.cmp(&shapes[a].area));

            let mut grid = vec![false; q.w * q.h];
            if solve_query(&mut grid, q.w, q.h, &presents, &shapes, q.w * q.h, 0) {
                solved_count += 1;
            }
        }

        Ok(solved_count)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    //endregion

    Ok(())
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::items_after_statements)]
fn parse_shape(lines: &[String]) -> Shape {
    let mut base_points = Vec::new();
    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == '#' {
                base_points.push(Point {
                    r: r as i32,
                    c: c as i32,
                });
            }
        }
    }

    let area = base_points.len();
    let mut variations = Vec::new();
    let mut seen = HashSet::new();

    // Generate all 8 symmetries
    for i in 0..8 {
        let mut points: Vec<Point> = base_points.clone();

        // Flip
        if i >= 4 {
            for p in &mut points {
                p.c = -p.c;
            }
        }
        // Rotate
        for _ in 0..(i % 4) {
            for p in &mut points {
                let tmp = p.r;
                p.r = p.c;
                p.c = -tmp;
            }
        }

        // Normalize
        let min_r = points.iter().map(|p| p.r).min().unwrap_or(0);
        let min_c = points.iter().map(|p| p.c).min().unwrap_or(0);
        let mut normalized: Vec<Point> = points
            .iter()
            .map(|p| Point {
                r: p.r - min_r,
                c: p.c - min_c,
            })
            .collect();
        normalized.sort();

        if seen.insert(normalized.clone()) {
            let max_r = normalized.iter().map(|p| p.r).max().unwrap_or(0);
            let max_c = normalized.iter().map(|p| p.c).max().unwrap_or(0);
            variations.push(Variation {
                points: normalized,
                height: (max_r + 1) as usize,
                width: (max_c + 1) as usize,
            });
        }
    }

    Shape { variations, area }
}

#[allow(clippy::cast_sign_loss)]
fn solve_query(
    grid: &mut Vec<bool>,
    w: usize,
    h: usize,
    presents: &[usize],
    shapes: &HashMap<usize, Shape>,
    empty_cells: usize,
    start_idx: usize,
) -> bool {
    if presents.is_empty() {
        return true;
    }

    let pid = presents[0];
    let shape = &shapes[&pid];
    let mut needed_area = 0;
    for &id in presents {
        needed_area += shapes[&id].area;
    }
    if empty_cells < needed_area {
        return false;
    }

    for i in start_idx..(w * h) {
        let r = i / w;
        let c = i % w;

        if (w * h - i) < needed_area {
            return false;
        }

        for var in &shape.variations {
            if r + var.height > h || c + var.width > w {
                continue;
            }

            let mut fits = true;
            for p in &var.points {
                let idx = (r + p.r as usize) * w + (c + p.c as usize);
                if grid[idx] {
                    fits = false;
                    break;
                }
            }

            if fits {
                for p in &var.points {
                    let idx = (r + p.r as usize) * w + (c + p.c as usize);
                    grid[idx] = true;
                }

                let next_start = if presents.len() > 1 && presents[1] == pid {
                    i
                } else {
                    0
                };

                if solve_query(
                    grid,
                    w,
                    h,
                    &presents[1..],
                    shapes,
                    empty_cells - shape.area,
                    next_start,
                ) {
                    return true;
                }

                for p in &var.points {
                    let idx = (r + p.r as usize) * w + (c + p.c as usize);
                    grid[idx] = false;
                }
            }
        }
    }

    false
}
