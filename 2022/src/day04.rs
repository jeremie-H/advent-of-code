use std::error::Error;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let result = input.lines().map(|l|l.split_once(',').unwrap())
    .map(|(s1,s2)|(s1.split_once('-').unwrap(), s2.split_once('-').unwrap()))
    .map(|(s1,s2)|((s1.0.parse::<i64>().unwrap(),s1.1.parse::<i64>().unwrap()),(s2.0.parse::<i64>().unwrap(),s2.1.parse::<i64>().unwrap())))
    .filter(|(elf1,elf2)|(elf1.0 >= elf2.0 && elf1.1 <= elf2.1) || (elf1.0 <= elf2.0 && elf1.1 >= elf2.1))
    .count() as i64 ;
    Ok(result)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let result = input.lines().map(|l|l.split_once(',').unwrap())
    .map(|(s1,s2)|(s1.split_once('-').unwrap(), s2.split_once('-').unwrap()))
    .map(|(s1,s2)|((s1.0.parse::<i64>().unwrap(),s1.1.parse::<i64>().unwrap()),(s2.0.parse::<i64>().unwrap(),s2.1.parse::<i64>().unwrap())))
    .filter(|(elf1,elf2)|(elf1.0 >= elf2.0 && elf1.1 <= elf2.1) || (elf1.0 <= elf2.0 && elf1.1 >= elf2.1) || (elf1.1 >= elf2.0 && elf1.0 <= elf2.0)|| (elf1.0 <= elf2.1 && elf1.1 >= elf2.1))
    .count() as i64 ;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 4);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input04.txt")).unwrap(), 562);
        assert_eq!(part2(include_str!("../inputs/input04.txt")).unwrap(), 924);
    }
}
