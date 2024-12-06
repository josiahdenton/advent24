use std::fs;

use crate::Part;
use anyhow::Result;
use regex::Regex;

pub fn process(part: Part, file_path: &str) -> Result<()> {
    let content = fs::read_to_string(file_path)?;
    let memory = content.lines().collect::<Vec<&str>>().join("");

    let total = scan_and_mul(&memory, part);
    println!("total is {total}");

    Ok(())
}

fn scan_and_mul(memory: &str, part: Part) -> u32 {
    let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();

    let mut total = 0;
    let mut offset = 0;
    while let Some(cap) = re_mul.captures_at(memory, offset) {
        let cur_mul = cap.get(0).unwrap();
        let m1 = cap.get(1).map_or("", |m| m.as_str());
        let m2 = cap.get(2).map_or("", |m| m.as_str());
        if part == Part::P2
            && dont_re
                .shortest_match_at(memory, offset)
                .unwrap_or_default()
                <= cur_mul.end()
            && dont_re
                .shortest_match_at(memory, offset)
                .unwrap_or_default()
                != 0
        {
            offset = do_re.shortest_match_at(memory, offset).unwrap_or_default();
            if offset == 0 {
                break;
            }
            continue;
        }

        let m1 = m1
            .parse::<u32>()
            .unwrap_or_else(|e| panic!("failed to parse {:?} reason {:?}", m1, e));
        let m2 = m2
            .parse::<u32>()
            .unwrap_or_else(|e| panic!("failed to parse {:?} reason {:?}", m2, e));

        total += m1 * m2;
        offset = cur_mul.end();
        if offset > memory.len() {
            break;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_single_match() {
        let good_instr = "xmul(2,4)%&";
        let bad_instr = "%&mul[3,7]";
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        if let Some(cap) = re.captures(good_instr) {
            let m1 = cap.get(1).map_or("", |m| m.as_str());
            let m2 = cap.get(2).map_or("", |m| m.as_str());
            println!("{}, {}", m1, m2);
        }

        if let Some(cap) = re.captures(bad_instr) {
            let m1 = cap.get(1).map_or("", |m| m.as_str());
            let m2 = cap.get(2).map_or("", |m| m.as_str());
            println!("{}, {}", m1, m2);
        } else {
            println!("no match");
        }
    }

    #[test]
    fn test_multi_match() {
        let multi = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        let mut offset = 0;
        while let Some(cap) = re.captures_at(multi, offset) {
            let m1 = cap.get(1).map_or("", |m| m.as_str());
            let m2 = cap.get(2).map_or("", |m| m.as_str());
            println!("{}, {}", m1, m2);
            offset = cap.get(0).unwrap().end();
            if offset > multi.len() {
                break;
            }
        }
    }

    #[test]
    fn test_p1_day_input() {
        assert!(process(Part::P1, "./inputs/day3.txt").is_ok());
    }

    #[test]
    fn test_p2_day_input() {
        assert!(process(Part::P2, "./inputs/day3.txt").is_ok());
    }
}
