#![allow(unexpected_cfgs)]
#![warn(clippy::all, clippy::pedantic)]

use adv_code_2025::start_day;
use anyhow::{Ok, Result};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::items_after_statements)]
    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let tiles = parse_tiles(reader);
        let mut max_area = 0;

        for i in 0..tiles.len() {
            for j in (i + 1)..tiles.len() {
                let (x1, y1) = tiles[i];
                let (x2, y2) = tiles[j];

                let width = (x1 - x2).unsigned_abs() as usize + 1;
                let height = (y1 - y2).unsigned_abs() as usize + 1;
                let area = width * height;

                if area > max_area {
                    max_area = area;
                }
            }
        }
        Ok(max_area)
    }

    assert_eq!(50, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::items_after_statements)]
    #[allow(clippy::similar_names)]
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let tiles = parse_tiles(reader);
        let mut max_area = 0;

        // Build edges for the polygon
        let mut edges = Vec::new();
        for i in 0..tiles.len() {
            let p1 = tiles[i];
            let p2 = tiles[(i + 1) % tiles.len()]; // Wrap around
            edges.push((p1, p2));
        }

        for i in 0..tiles.len() {
            for j in (i + 1)..tiles.len() {
                let (x1, y1) = tiles[i];
                let (x2, y2) = tiles[j];

                // Calculate dimensions
                let width = (x1 - x2).unsigned_abs() as usize + 1;
                let height = (y1 - y2).unsigned_abs() as usize + 1;
                let area = width * height;

                // Optimization: Don't check complex geometry if area is already smaller than max
                if area <= max_area {
                    continue;
                }

                // Check if this rectangle is valid (entirely inside/on polygon)
                // Rectangle Bounds (inclusive)
                let rx_min = x1.min(x2);
                let rx_max = x1.max(x2);
                let ry_min = y1.min(y2);
                let ry_max = y1.max(y2);

                if is_valid_rectangle(rx_min, rx_max, ry_min, ry_max, &edges) {
                    max_area = area;
                }
            }
        }
        Ok(max_area)
    }

    assert_eq!(24, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    //endregion

    Ok(())
}

fn parse_tiles<R: BufRead>(reader: R) -> Vec<(i64, i64)> {
    reader
        .lines()
        .map_while(Result::ok)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (x_str, y_str) = line.split_once(',').expect("Line must contain a comma");
            (
                x_str.trim().parse().expect("Invalid X coordinate"),
                y_str.trim().parse().expect("Invalid Y coordinate"),
            )
        })
        .collect()
}

type Edge = ((i64, i64), (i64, i64));

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::similar_names)]
fn is_valid_rectangle(
    rx_min: i64,
    rx_max: i64,
    ry_min: i64,
    ry_max: i64,
    edges: &Vec<Edge>,
) -> bool {
    // 1. Check if any polygon edge splits the rectangle
    // A split happens if an edge passes strictly through the interior of the rect.
    for &((px1, py1), (px2, py2)) in edges {
        let ex_min = px1.min(px2);
        let ex_max = px1.max(px2);
        let ey_min = py1.min(py2);
        let ey_max = py1.max(py2);

        let is_vertical = px1 == px2;

        if is_vertical {
            // Vertical edge at x = px1.
            // Splits if x is strictly inside (rx_min, rx_max)
            // AND the y-ranges overlap strictly.
            if px1 > rx_min && px1 < rx_max {
                let overlap_start = ey_min.max(ry_min);
                let overlap_end = ey_max.min(ry_max);
                if overlap_start < overlap_end {
                    return false; // Split detected
                }
            }
        } else {
            // Horizontal edge at y = py1.
            // Splits if y is strictly inside (ry_min, ry_max)
            // AND the x-ranges overlap strictly.
            if py1 > ry_min && py1 < ry_max {
                let overlap_start = ex_min.max(rx_min);
                let overlap_end = ex_max.min(rx_max);
                if overlap_start < overlap_end {
                    return false; // Split detected
                }
            }
        }
    }

    // 2. Check if the center of the rectangle is inside the polygon or on boundary.
    // We use floating point for the center to avoid integer division issues.
    let cx = f64::midpoint(rx_min as f64, rx_max as f64);
    let cy = f64::midpoint(ry_min as f64, ry_max as f64);

    // Check "On Boundary" (distance to any edge is 0)
    // Actually, simple check: is cx, cy exactly on any segment?
    // Since edges are axis aligned, this is easy.
    for &((px1, py1), (px2, py2)) in edges {
        let ex_min = px1.min(px2) as f64;
        let ex_max = px1.max(px2) as f64;
        let ey_min = py1.min(py2) as f64;
        let ey_max = py1.max(py2) as f64;

        let is_vertical = px1 == px2;
        if is_vertical {
            if (px1 as f64 - cx).abs() < 1e-9 && cy >= ey_min && cy <= ey_max {
                return true; // On boundary
            }
        } else if (py1 as f64 - cy).abs() < 1e-9 && cx >= ex_min && cx <= ex_max {
            return true; // On boundary
        }
    }

    // Ray Casting (Ray to x = +infinity)
    let mut intersections = 0;
    for &((px1, py1), (px2, py2)) in edges {
        let is_vertical = px1 == px2;
        if is_vertical {
            let vx = px1 as f64;
            let vy_min = py1.min(py2) as f64;
            let vy_max = py1.max(py2) as f64;

            // Check if ray crosses this vertical segment
            // Ray is at y = cy, going x > cx.
            // Condition: Edge must be to the right (vx > cx)
            // AND cy must be within vertical range [vy_min, vy_max)
            // (Standard Raycast uses half-open intervals to handle vertices)
            if vx > cx && cy >= vy_min && cy < vy_max {
                intersections += 1;
            }
        }
    }

    // Odd intersections = Inside
    intersections % 2 == 1
}
