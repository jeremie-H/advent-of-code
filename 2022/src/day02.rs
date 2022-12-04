use std::error::Error;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    Ok(input.lines()
    .map(|l| (l.as_bytes()[0],l.as_bytes()[2]))
    .map(|(a,b)| {
        (match b {
            b'X' => 1,
            b'Y' => 2,
            b'Z' => 3,
            _ => 0
        }) + match a {
            b'A' => if b == b'X' { 3 } else if b == b'Y' { 6 } else { 0 },
            b'B' => if b == b'X' { 0 } else if b == b'Y' { 3 } else { 6 },
            b'C' => if b == b'X' { 6 } else if b == b'Y' { 0 } else { 3 },
            _ => 0
        }
    })
    .sum())
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    Ok(input.lines()
    .map(|l| (l.as_bytes()[0],l.as_bytes()[2]))
    .map(|(a,b)| {
       (match b {
            b'Y' => 3,
            b'Z' => 6,
        _ => 0
        }) + match a {
            b'A' => if b == b'X' { 3 } else if b == b'Y' { 1 } else { 2 },
            b'B' => if b == b'X' { 1 } else if b == b'Y' { 2 } else { 3 },
            b'C' => if b == b'X' { 2 } else if b == b'Y' { 3 } else { 1 },
            _ => 0
        }
    })
    .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "A Y
B X
C Z";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 12);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input02.txt")).unwrap(), 14531);
        assert_eq!(part2(include_str!("../inputs/input02.txt")).unwrap(), 11258);
    }
}
