use anyhow::{Context, Result};
use std::{fs, io::BufRead};

use crate::Part;

const MIN_GAP: u32 = 1;
const MAX_GAP: u32 = 3;

pub fn process(part: Part, file_path: String) -> Result<()> {
    let fp = fs::OpenOptions::new().read(true).open(file_path)?;
    let mut reader = std::io::BufReader::new(fp);
    let mut num_good_reports = 0;

    loop {
        let mut buf = String::new();
        if let Ok(read_sz) = reader.read_line(&mut buf) {
            if read_sz > 0 {
                num_good_reports += if is_good_report(
                    &mut buf
                        .split(" ")
                        .filter(|n| !n.is_empty())
                        .map(|n| {
                            n.trim()
                                .parse()
                                .context(format!("failed parsing {n} for line {buf}"))
                                .expect("not a number")
                        })
                        .collect::<Vec<u32>>(),
                    part != Part::P1,
                ) {
                    1
                } else {
                    0
                }
            } else {
                println!("number of good reports {num_good_reports}");
                break;
            }
        } else {
            panic!("failed to read line")
        }
    }

    Ok(())
}

fn is_good_report(report: &mut Vec<u32>, dampen: bool) -> bool {
    if report.is_empty() {
        return true;
    }
    let decreases = report
        .windows(2)
        .filter(|window| window[0] > window[1])
        .count();
    let increases = report
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count();

    if increases == decreases {
        println!("{:?}", report);
        return false;
    }

    let is_decreasing = decreases > increases;
    let is_increasing = increases > decreases;

    if decreases == increases {
        return false;
    }

    // RULES
    // 1. it should either increase all the way or decrease all the way
    // 2. the differences between readings must be within a given window (1-3)
    // 3. if dampening is enabled, we are allowed to check if a level can be removed to make the
    //    report safe.

    let incorrect_direction = report.windows(2).enumerate().find_map(|(index, window)| {
        // ^^ NOTE index is index of first item only
        if (window[0] < window[1] && is_decreasing) || (window[0] > window[1] && is_increasing) {
            return Some(index);
        }
        None
    });
    if let Some(i) = incorrect_direction {
        if dampen {
            let mut lhs = report.clone();
            lhs.remove(i);
            let mut rhs = report.clone();
            rhs.remove(i + 1);
            return is_good_report(&mut lhs, false) || is_good_report(&mut rhs, false);
        }
        return false;
    }

    let incorrect_change = report.windows(2).enumerate().find_map(|(index, window)| {
        let delta = window[0].abs_diff(window[1]);
        if !(MIN_GAP..=MAX_GAP).contains(&delta) {
            // if not in RANGE...
            return Some(index);
        }
        None
    });

    if let Some(i) = incorrect_change {
        if dampen {
            let mut lhs = report.clone();
            lhs.remove(i);
            let mut rhs = report.clone();
            rhs.remove(i + 1);
            return is_good_report(&mut lhs, false) || is_good_report(&mut rhs, false);
        }
        return false;
    }

    true // good report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_input_p1() {
        assert!(
            process(Part::P1, String::from("./tests/day2.txt")).is_ok(),
            "failed to process day2 sample input"
        );
    }

    #[test]
    fn test_simple_input_p2() {
        assert!(
            process(Part::P2, String::from("./tests/day2.txt")).is_ok(),
            "failed to process day2 sample input"
        );
    }

    #[test]
    fn test_day_input_p1() {
        assert!(
            process(Part::P1, String::from("./inputs/day2.txt")).is_ok(),
            "failed to process day2 user input"
        );
    }

    #[test]
    fn test_day_input_p2() {
        assert!(
            process(Part::P2, String::from("./inputs/day2.txt")).is_ok(),
            "failed to process day2 user input"
        );
    }
}
