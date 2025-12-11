#![allow(unexpected_cfgs)]
#![warn(clippy::all, clippy::pedantic)]

use adv_code_2025::start_day;
use anyhow::{Ok, Result};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

const TEST2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    #[allow(clippy::items_after_statements)]
    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let graph = parse_graph(reader)?;
        let mut memo = HashMap::new();
        let answer = count_paths("you", "out", &graph, &mut memo);
        Ok(answer)
    }

    assert_eq!(5, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    #[allow(clippy::items_after_statements)]
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let graph = parse_graph(reader)?;

        // Helper to run path counting with a fresh memo cache every time
        let run_count = |start, end| -> usize {
            let mut memo = HashMap::new();
            count_paths(start, end, &graph, &mut memo)
        };

        // Case A: Path goes svr -> ... -> dac -> ... -> fft -> ... -> out
        // Count = (svr->dac) * (dac->fft) * (fft->out)
        let svr_dac = run_count("svr", "dac");
        let dac_fft = run_count("dac", "fft");
        let fft_out = run_count("fft", "out");
        let path_dac_first = svr_dac * dac_fft * fft_out;

        // Case B: Path goes svr -> ... -> fft -> ... -> dac -> ... -> out
        // Count = (svr->fft) * (fft->dac) * (dac->out)
        let svr_fft = run_count("svr", "fft");
        let fft_dac = run_count("fft", "dac");
        let dac_out = run_count("dac", "out");
        let path_fft_first = svr_fft * fft_dac * dac_out;

        Ok(path_dac_first + path_fft_first)
    }

    assert_eq!(2, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    //endregion

    Ok(())
}

fn parse_graph<R: BufRead>(reader: R) -> Result<HashMap<String, Vec<String>>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        if let Some((source, destinations)) = line.split_once(": ") {
            let dest_nodes: Vec<String> = destinations
                .split_whitespace()
                .map(ToString::to_string)
                .collect();
            graph.insert(source.to_string(), dest_nodes);
        }
    }
    Ok(graph)
}

fn count_paths(
    current_node: &str,
    target_node: &str,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if current_node == target_node {
        return 1;
    }

    if let Some(&count) = memo.get(current_node) {
        return count;
    }

    let Some(neighbors) = graph.get(current_node) else {
        return 0;
    };

    let mut total_paths = 0;
    for neighbor in neighbors {
        total_paths += count_paths(neighbor, target_node, graph, memo);
    }

    memo.insert(current_node.to_string(), total_paths);
    total_paths
}
