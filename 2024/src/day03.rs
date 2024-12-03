use std::error::Error;
use regex::Regex;


/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let r = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    Ok(r.captures_iter(input)
    .map(|captures|captures[1].parse::<i64>().unwrap() * captures[2].parse::<i64>().unwrap())
    .sum())
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let r = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut is_enabled = true;
    Ok(r.captures_iter(input)
    //.inspect(|f|println!("f : {:?}",f))
    .fold(0, |mut total, captures| {
        if let Some(instruction) = captures.get(0) {
            match instruction.as_str() {
                "do()" => { is_enabled = true; }
                "don't()" => { is_enabled = false; }
                mul_instr if mul_instr.starts_with("mul") => {
                    if is_enabled {
                        total += captures[1].parse::<i64>().unwrap() * captures[2].parse::<i64>().unwrap();
                    }
                }
                _ => {}
            }
        }
        total
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const ÉNONCÉ2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ2).unwrap(), 48);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input03.txt")).unwrap(), 178538786);
        assert_eq!(part2(include_str!("../inputs/input03.txt")).unwrap(), 102467299);
    }
}
