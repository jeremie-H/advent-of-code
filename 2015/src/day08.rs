use std::error::Error;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let r = input.lines().map(|l| l.len() as i64 - char_en_memoire(l)).sum();
    Ok(r)
}

fn char_en_memoire(l: &str) -> i64 {
    let mut iterateur = l.chars();
    let mut compteur = 0;
    while let Some(c) = iterateur.next() {
        if c == '\\' {
            if let Some(c) = iterateur.next() {
                if c == 'x' {
                    iterateur.next();
                    iterateur.next();
                }
            }
        }
        compteur += 1;
    }
    compteur - 2
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    Ok(input
        .lines()
        .map(|l| (l.chars().flat_map(|c| c.escape_default()).count() + 2) as i64 - l.len() as i64)
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = r#"""
"abc"
"aaa\"aaa"
"\x27""#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 12);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 19);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input08.txt")).unwrap(), 1350);
        assert_eq!(part2(include_str!("../inputs/input08.txt")).unwrap(), 2085);
    }
}
