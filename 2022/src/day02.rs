use std::error::Error;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    Ok(input.lines()
    .filter_map(|l| l.split_once(' '))
    .map(|(a,b)| {
        (match b {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0
        }) + match a {
            "A" => if b == "X" { 3 } else if b == "Y" { 6 } else { 0 },
            "B" => if b == "X" { 0 } else if b == "Y" { 3 } else { 6 },
            "C" => if b == "X" { 6 } else if b == "Y" { 0 } else { 3 },
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
    .filter_map(|l| l.split_once(' '))
    .map(|(a,b)| {
        (match b {
        "Y" => 3,
        "Z" => 6,
        _ => 0
        }) + match a {
            "A" => if b == "X" { 3 } else if b == "Y" { 1 } else { 2 },
            "B" => if b == "X" { 1 } else if b == "Y" { 2 } else { 3 },
            "C" => if b == "X" { 2 } else if b == "Y" { 3 } else { 1 },
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
