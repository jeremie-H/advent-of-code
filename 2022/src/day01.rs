use std::error::Error;

use itertools::Itertools;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    Ok(input.split("\n\n")
    .map(|elve| elve.lines()
        .filter_map(|l| l.parse::<i64>().ok())
        .sum())
    .max().unwrap_or(0))
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    Ok(input.split("\n\n")
    .map(|elve| elve.lines()
        .filter_map(|l| l.parse::<i64>().ok())
        .sum::<i64>())
    .sorted().rev()
    .take(3)
    .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 45000);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input01.txt")).unwrap(), 72511);
        assert_eq!(part2(include_str!("../inputs/input01.txt")).unwrap(), 212117);
    }
}