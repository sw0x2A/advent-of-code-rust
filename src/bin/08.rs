#![allow(unexpected_cfgs)]
#![warn(clippy::all, clippy::pedantic)]

use adv_code_2025::start_day;
use anyhow::{Ok, Result};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    #[allow(clippy::items_after_statements)]
    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let coords = parse_coords(reader);
        let n = coords.len();
        let mut edges = generate_edges(&coords);
        edges.sort_unstable_by_key(|e| e.0);
        let mut parent: Vec<usize> = (0..n).collect();
        let mut size = vec![1; n];
        let limit = if n <= 20 { 10 } else { 1000 };
        for &(_dist2, i, j) in edges.iter().take(limit) {
            let pi = find(&mut parent, i);
            let pj = find(&mut parent, j);
            if pi != pj {
                parent[pi] = pj;
                size[pj] += size[pi];
                size[pi] = 0;
            }
        }
        let mut sizes: Vec<usize> = size.into_iter().filter(|&s| s > 0).collect();
        sizes.sort_unstable_by(|a, b| b.cmp(a));
        let answer = sizes.iter().take(3).product();
        Ok(answer)
    }

    assert_eq!(40, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::items_after_statements)]
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let coords = parse_coords(reader);
        let n = coords.len();
        if n < 2 {
            return Ok(0);
        }
        let mut edges = generate_edges(&coords);
        edges.sort_unstable_by_key(|e| e.0);
        let mut parent: Vec<usize> = (0..n).collect();
        let mut components = n;
        for &(_dist, u, v) in &edges {
            let root_u = find(&mut parent, u);
            let root_v = find(&mut parent, v);
            if root_u != root_v {
                parent[root_u] = root_v;
                components -= 1;
                if components == 1 {
                    let ans = (coords[u].0 as usize) * (coords[v].0 as usize);
                    return Ok(ans);
                }
            }
        }
        Ok(0)
    }

    assert_eq!(25272, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    //endregion

    Ok(())
}

fn parse_coords<R: BufRead>(reader: R) -> Vec<(i32, i32, i32)> {
    reader
        .lines()
        .map_while(Result::ok)
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let mut parts = l.split(',');
            let x = parts.next().unwrap().trim().parse().unwrap();
            let y = parts.next().unwrap().trim().parse().unwrap();
            let z = parts.next().unwrap().trim().parse().unwrap();
            (x, y, z)
        })
        .collect()
}

fn find(parent: &mut [usize], x: usize) -> usize {
    if parent[x] != x {
        parent[x] = find(parent, parent[x]);
    }
    parent[x]
}

fn generate_edges(coords: &[(i32, i32, i32)]) -> Vec<(i64, usize, usize)> {
    let n = coords.len();
    let mut edges = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        for j in i + 1..n {
            let (x1, y1, z1) = coords[i];
            let (x2, y2, z2) = coords[j];
            let dx = i64::from(x1 - x2);
            let dy = i64::from(y1 - y2);
            let dz = i64::from(z1 - z2);
            let dist2 = dx * dx + dy * dy + dz * dz;
            edges.push((dist2, i, j));
        }
    }
    edges
}
