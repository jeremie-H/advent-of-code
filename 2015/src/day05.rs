use std::error::Error;

use itertools::Itertools;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    Ok(input.lines().fold(0, |acc, l| {
        let mut previous_letter = '-';
        let mut double_letter = false;
        let mut voyelles = 0;
        for c in l.chars() {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => voyelles += 1,
                'b' if previous_letter == 'a' => return acc,
                'd' if previous_letter == 'c' => return acc,
                'q' if previous_letter == 'p' => return acc,
                'y' if previous_letter == 'x' => return acc,
                _ => {}
            };
            match c {
                single if previous_letter == single => double_letter = true,
                _ => {}
            };
            previous_letter = c;
        }
        if voyelles >= 3 && double_letter {
            acc + 1
        } else {
            acc
        }
    }))
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    Ok(input.lines().fold(0, |acc, l| {
        let mut pair = false;
        let mut atleast_oneletter = false;
        let mut previous = '-';
        let mut s = String::with_capacity(2);
        for (c1, c2) in l.chars().tuple_windows() {
            s.push(c1);
            s.push(c2);
            pair |= l.match_indices(&s).count() > 1;
            s.clear();
            //println!("({},{}) pair = {}, cc={:?}",c1,c2,pair,cc);
            if c2 == previous {
                atleast_oneletter = true;
            }
            if pair && atleast_oneletter {
                return acc + 1;
            }
            previous = c1;
        }
        acc
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("ugknbfddgicrmopn\naaa\njchzalrnumimnmhp\nhaegwjzuvuyypxyu\ndvszwmarrgswjxmb").unwrap(), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("qjhvhtzxzqqjkmpb\nxxyxx\nuurcxstgmygtbstg\nieodomkazucvgmuy").unwrap(), 2);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input05.txt")).unwrap(), 236);
        assert_eq!(part2(include_str!("../inputs/input05.txt")).unwrap(), 51);
    }
}
