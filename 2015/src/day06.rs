use std::error::Error;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    input.lines().for_each(|l| {
        println!("l : {}",l);
    });
    Ok(0)
}

/**
 * Part 2
 */
pub fn part2(_input: &str) -> Result<i64, Box<dyn Error>> { Ok(0) }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input =r"turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500";
        assert_eq!(part1(input).unwrap(), 998996);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("").unwrap(), 0);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        // assert_eq!(part1(include_str!("../inputs/input06.txt")).unwrap(), 0);
        // assert_eq!(part2(include_str!("../inputs/input06.txt")).unwrap(), 0);
    }
}
