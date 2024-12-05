use std::{collections::HashMap, fs::OpenOptions, io::BufRead};

use crate::Part;
use anyhow::{Context, Result};

pub fn process(part: Part, file_path: String) -> Result<()> {
    if part == Part::P1 {
        let (mut left, mut right) = build_lists_from_file(&file_path)?;
        let total = distance(&mut left, &mut right);
        println!("total distance is {total}")
    } else if part == Part::P2 {
        let (mut left, mut right) = build_lists_from_file(&file_path)?;
        let total = similarity(&mut left, &mut right);
        println!("total similarity is {total}")
    }

    Ok(())
}

fn distance(left: &mut [u32], right: &mut [u32]) -> u32 {
    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(left, right)| left.abs_diff(*right))
        .sum()
}

fn similarity(left: &mut [u32], right: &mut [u32]) -> u32 {
    let mut occurrences: HashMap<u32, u32> = HashMap::new();
    right.iter().for_each(|n| {
        if let Some(count) = occurrences.get(n) {
            occurrences.insert(*n, count + 1);
        } else {
            occurrences.insert(*n, 1);
        }
    });

    left.iter()
        .map(|n| {
            if let Some(count) = occurrences.get(n) {
                count * n
            } else {
                0
            }
        })
        .sum()
}

fn build_lists_from_file(file_path: &str) -> Result<(Vec<u32>, Vec<u32>)> {
    let fp = OpenOptions::new().read(true).open(file_path)?;
    let mut reader = std::io::BufReader::new(fp);
    let mut left_side = vec![];
    let mut right_side = vec![];

    loop {
        let mut line = String::new();
        if let Ok(bytes_read) = reader.read_line(&mut line) {
            if bytes_read > 0 {
                let mut line_locations = line.split(" ").filter(|word| !word.is_empty());
                if let Some(num) = line_locations.next() {
                    left_side.push(
                        num.trim()
                            .parse()
                            .context(format!("failet parse num on LHS, input: {}", num))?,
                    )
                } else {
                    panic!("failed to parse line");
                }
                if let Some(num) = line_locations.next() {
                    right_side.push(
                        num.trim()
                            .parse()
                            .context(format!("failet parse num on LHS, input: {}", num))?,
                    )
                } else {
                    panic!("failed to parse line");
                }
            } else {
                return Ok((left_side, right_side));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_lists_from_raw(raw: &str) -> Result<(Vec<u32>, Vec<u32>)> {
        let mut left_side = vec![];
        let mut right_side = vec![];

        raw.lines().for_each(|line| {
            if line.is_empty() {
                return;
            }

            let mut line_locations = line.split(" ").filter(|word| !word.is_empty());
            if let Some(num) = line_locations.next() {
                left_side.push(
                    num.parse()
                        .unwrap_or_else(|_| panic!("failet parse num on LHS, input: {}", num)),
                )
            } else {
                panic!("failed to parse line");
            }
            if let Some(num) = line_locations.next() {
                right_side.push(
                    num.parse()
                        .unwrap_or_else(|_| panic!("failet parse num on LHS, input: {}", num)),
                )
            } else {
                panic!("failed to parse line");
            }
        });

        Ok((left_side, right_side))
    }

    #[test]
    fn test_list_split() {
        let raw_in = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

        assert!(
            build_lists_from_raw(raw_in)
                .map(|(left, right)| {
                    println!("{:?}, {:?}", left, right);
                })
                .is_ok(),
            "building lists failed"
        )
    }

    #[test]
    fn test_simple_distance() {
        let raw_in = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        if let Ok((mut left, mut right)) = build_lists_from_raw(raw_in) {
            assert!(
                distance(&mut left, &mut right) == 11,
                "failed to calculate correct distance"
            )
        } else {
            panic!("failed to build lists from raw for some reason")
        }
    }

    #[test]
    fn test_simple_similarity() {
        let raw_in = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        if let Ok((mut left, mut right)) = build_lists_from_raw(raw_in) {
            assert!(
                similarity(&mut left, &mut right) == 31,
                "failed to calculate correct distance"
            )
        } else {
            panic!("failed to build lists from raw for some reason")
        }
    }
}