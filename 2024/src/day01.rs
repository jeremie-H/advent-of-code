use itertools::Itertools;
use std::error::Error;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let (mut gauche, mut droite): (Vec<i64>, Vec<i64>) = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
        .unzip();
    gauche.sort();
    droite.sort();
    Ok(gauche.into_iter().zip(droite).map(|(a, b)| (a - b).abs()).sum())
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let (gauche, droite): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("   ").unwrap();
            (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
        })
        .unzip();

    let counts = droite.iter().copied().counts();

    Ok(gauche.into_iter().map(|a| a * *counts.get(&a).unwrap_or(&0) as i64).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 31);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input01.txt")).unwrap(), 1530215);
        assert_eq!(part2(include_str!("../inputs/input01.txt")).unwrap(), 26800609);
    }
}
