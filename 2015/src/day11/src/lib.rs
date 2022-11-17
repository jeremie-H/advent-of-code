use std::error::Error;

/**
 * Part 1
 */
pub fn part1(_input: &str) -> Result<i64, Box<dyn Error>> {
    let result = 0;
    Ok(result)
}

/**
 * Part 2
 */
pub fn part2(_input: &str) -> Result<i64, Box<dyn Error>> {
    let result = 0;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // avec les données d'essai
        assert_eq!(part1("").unwrap(), 0);
    }

    #[test]
    fn test_part2() {
        // avec les données d'essai
        assert_eq!(part2("").unwrap(), 0);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("input.txt")).unwrap(), 0);
        assert_eq!(part2(include_str!("input.txt")).unwrap(), 0);
    }
}
