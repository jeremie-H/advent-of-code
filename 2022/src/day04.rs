use std::error::Error;

/**
 * Part 1
 */
pub fn part1(_input: &str) -> Result<i64, Box<dyn Error>> { Ok(0) }

/**
 * Part 2
 */
pub fn part2(_input: &str) -> Result<i64, Box<dyn Error>> { Ok(0) }

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 0);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        // assert_eq!(part1(include_str!("../inputs/input04.txt")).unwrap(), 0);
        // assert_eq!(part2(include_str!("../inputs/input04.txt")).unwrap(), 0);
    }
}
