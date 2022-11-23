use itertools::Itertools;
use std::error::Error;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let result = input.lines().fold(0, |papier, cadeau: &str| {
        papier
            + cadeau
                .split('x')
                .map(|f| f.parse::<i64>().unwrap())
                .tuple_combinations::<(i64, i64)>()
                //.inspect(|f| println!("{:?}",f))
                .fold(0, |acc, (x, y)| acc + 2 * x * y)
            + cadeau
                .split('x')
                .map(|f| f.parse::<i64>().unwrap())
                .tuple_combinations::<(i64, i64)>()
                //.inspect(|f| println!("{:?}",f))
                .map(|(x, y)| x * y)
                .min()
                .unwrap()
    });
    Ok(result)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let result = input.lines().fold(0, |ruban, cadeau: &str| {
        ruban
            + cadeau
                .split('x')
                .map(|f| f.parse::<i64>().unwrap())
                .sorted()
                .take(2)
                //.inspect(|f| println!("{:?}",f))
                .fold(0, |acc, x| acc + 2 * x)
            + cadeau.split('x').map(|f| f.parse::<i64>().unwrap()).product::<i64>()
    });
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("2x3x4\n1x1x10").unwrap(), 101);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("2x3x4\n1x1x10").unwrap(), 14 + 34);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input02.txt")).unwrap(), 1586300);
        assert_eq!(part2(include_str!("../inputs/input02.txt")).unwrap(), 3737498);
    }
}
