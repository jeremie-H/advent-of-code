use std::error::Error;
use itertools::Itertools;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    Ok(input
        .lines()
        .map(|line| line.split_whitespace().filter_map(|e| e.parse::<i64>().ok()).collect::<Vec<i64>>())
        .filter(|levels| is_safe(levels))
        .count() as i64)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    Ok(input
        .lines()
        .map(|line| line.split_whitespace().filter_map(|e| e.parse::<i64>().ok()).collect::<Vec<i64>>())
        .filter(|levels| {
            if is_safe(levels) { return true; }
            (0..levels.len()).any(|i| {
                let mut new_levels = levels.to_vec();
                new_levels.remove(i);
                is_safe(&new_levels)
            })
        })
        .count() as i64)
}

fn is_safe(levels: &[i64])  -> bool {
    levels.iter().tuple_windows().all(|(a, b)| b > a && b - a <= 3) ||
    levels.iter().tuple_windows().all(|(a, b)| a > b && a - b <= 3)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

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
        assert_eq!(part1(include_str!("../inputs/input02.txt")).unwrap(), 369);
        assert_eq!(part2(include_str!("../inputs/input02.txt")).unwrap(), 428);
    }
}
