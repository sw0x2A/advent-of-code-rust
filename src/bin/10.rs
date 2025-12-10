#![allow(unexpected_cfgs)]
#![warn(clippy::all, clippy::pedantic)]

use adv_code_2025::start_day;
use anyhow::{Context, Result};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

#[derive(Debug)]
struct Machine {
    lights_target: u128,
    joltage_target: Vec<f64>,
    buttons: Vec<Vec<usize>>,
    num_slots: usize,
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    #[allow(clippy::items_after_statements)]
    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut total = 0;
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            let machine = parse_machine(&line)?;
            total += solve_part1_machine(&machine);
        }
        Ok(total)
    }

    assert_eq!(7, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    #[allow(clippy::items_after_statements)]
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut total = 0;
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            let machine = parse_machine(&line)?;
            if let Some(presses) = solve_part2_machine(&machine) {
                total += presses;
            }
        }
        Ok(total)
    }

    assert_eq!(33, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    //endregion

    Ok(())
}

fn parse_machine(line: &str) -> Result<Machine> {
    let open_bracket = line.find('[').context("Missing [")?;
    let close_bracket = line.find(']').context("Missing ]")?;
    let lights_str = &line[open_bracket + 1..close_bracket];

    let mut lights_target: u128 = 0;
    for (i, c) in lights_str.chars().enumerate() {
        if c == '#' {
            lights_target |= 1 << i;
        }
    }

    let open_brace = line.find('{').context("Missing {")?;
    let close_brace = line.find('}').context("Missing }")?;
    let req_str = &line[open_brace + 1..close_brace];

    let joltage_target: Vec<f64> = req_str
        .split(',')
        .map(|s| s.trim().parse::<f64>())
        .collect::<Result<_, _>>()?;

    let num_slots = joltage_target.len();

    let diagram_end = close_bracket + 1;
    let buttons_part = &line[diagram_end..open_brace];

    let mut buttons = Vec::new();
    let mut in_paren = false;
    let mut buffer = String::new();

    for c in buttons_part.chars() {
        if c == '(' {
            in_paren = true;
            buffer.clear();
        } else if c == ')' {
            if in_paren {
                let mut indices = Vec::new();
                if !buffer.trim().is_empty() {
                    for num_str in buffer.split(',') {
                        let idx: usize = num_str.trim().parse()?;
                        indices.push(idx);
                    }
                }
                buttons.push(indices);
                in_paren = false;
            }
        } else if in_paren {
            buffer.push(c);
        }
    }

    Ok(Machine {
        lights_target,
        joltage_target,
        buttons,
        num_slots,
    })
}

fn solve_part1_machine(machine: &Machine) -> usize {
    let button_masks: Vec<u128> = machine
        .buttons
        .iter()
        .map(|indices| {
            let mut mask = 0;
            for &idx in indices {
                mask |= 1 << idx;
            }
            mask
        })
        .collect();

    for k in 0..=button_masks.len() {
        if check_xor_combination(&button_masks, k, 0, 0, machine.lights_target) {
            return k;
        }
    }
    0
}

fn check_xor_combination(
    buttons: &[u128],
    k: usize,
    start_idx: usize,
    current_acc: u128,
    target: u128,
) -> bool {
    if k == 0 {
        return current_acc == target;
    }

    for i in start_idx..buttons.len() {
        if buttons.len() - i < k {
            break;
        }
        if check_xor_combination(buttons, k - 1, i + 1, current_acc ^ buttons[i], target) {
            return true;
        }
    }
    false
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::needless_range_loop)]
fn solve_part2_machine(machine: &Machine) -> Option<usize> {
    let rows = machine.num_slots;
    let cols = machine.buttons.len();

    let mut matrix = vec![vec![0.0; cols + 1]; rows];
    for (c, btn_indices) in machine.buttons.iter().enumerate() {
        for &r in btn_indices {
            if r < rows {
                matrix[r][c] = 1.0;
            }
        }
    }
    for r in 0..rows {
        matrix[r][cols] = machine.joltage_target[r];
    }

    let mut bounds = vec![usize::MAX; cols];
    for c in 0..cols {
        let mut limit = usize::MAX;
        let mut affects_any = false;
        for r in 0..rows {
            if matrix[r][c] > 0.5 {
                affects_any = true;
                let allowed = (machine.joltage_target[r] / matrix[r][c]).floor();
                let allowed = if allowed < 0.0 { 0 } else { allowed as usize };
                if allowed < limit {
                    limit = allowed;
                }
            }
        }
        bounds[c] = if affects_any { limit } else { 0 };
    }

    let mut pivot_row = 0;
    let mut pivot_cols = Vec::new();

    for c in 0..cols {
        if pivot_row >= rows {
            break;
        }

        let mut best_r = pivot_row;
        let mut found = false;
        for r in pivot_row..rows {
            if matrix[r][c].abs() > 1e-9 {
                best_r = r;
                found = true;
                break;
            }
        }

        if !found {
            continue;
        }

        matrix.swap(pivot_row, best_r);

        let div = matrix[pivot_row][c];
        for j in c..=cols {
            matrix[pivot_row][j] /= div;
        }

        for r in 0..rows {
            if r != pivot_row {
                let factor = matrix[r][c];
                if factor.abs() > 1e-9 {
                    for j in c..=cols {
                        matrix[r][j] -= factor * matrix[pivot_row][j];
                    }
                }
            }
        }

        pivot_cols.push((pivot_row, c));
        pivot_row += 1;
    }

    for r in pivot_row..rows {
        if matrix[r][cols].abs() > 1e-9 {
            return None;
        }
    }

    let pivot_col_indices: Vec<usize> = pivot_cols.iter().map(|&(_, c)| c).collect();
    let free_cols: Vec<usize> = (0..cols)
        .filter(|c| !pivot_col_indices.contains(c))
        .collect();

    let mut best_total = None;
    let mut current_free = vec![0; free_cols.len()];

    solve_recursive(
        0,
        &free_cols,
        &bounds,
        &pivot_cols,
        &matrix,
        &mut current_free,
        cols,
        &mut best_total,
    );

    best_total
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::too_many_arguments)]
fn solve_recursive(
    idx: usize,
    free_cols: &[usize],
    bounds: &[usize],
    pivot_cols: &[(usize, usize)],
    matrix: &Vec<Vec<f64>>,
    current_free: &mut Vec<usize>,
    num_vars: usize,
    best_total: &mut Option<usize>,
) {
    if idx == free_cols.len() {
        let mut x = vec![0.0; num_vars];
        let mut current_sum = 0;

        for (i, &c) in free_cols.iter().enumerate() {
            let val = current_free[i];
            x[c] = val as f64;
            current_sum += val;
        }

        if let Some(bt) = *best_total
            && current_sum >= bt
        {
            return;
        }

        for &(r, c) in pivot_cols.iter().rev() {
            let mut val = matrix[r][matrix[0].len() - 1];
            for j in (c + 1)..num_vars {
                if matrix[r][j].abs() > 1e-9 {
                    val -= matrix[r][j] * x[j];
                }
            }

            if val < -1e-9 {
                return;
            }
            let rounded = val.round();
            if (val - rounded).abs() > 1e-9 {
                return;
            }

            let int_val = rounded as usize;
            if int_val > bounds[c] {
                return;
            }

            x[c] = rounded;
            current_sum += int_val;

            if let Some(bt) = *best_total
                && current_sum >= bt
            {
                return;
            }
        }

        if best_total.is_none_or(|bt| current_sum < bt) {
            *best_total = Some(current_sum);
        }
        return;
    }

    let c = free_cols[idx];
    for val in 0..=bounds[c] {
        current_free[idx] = val;
        solve_recursive(
            idx + 1,
            free_cols,
            bounds,
            pivot_cols,
            matrix,
            current_free,
            num_vars,
            best_total,
        );
    }
}
