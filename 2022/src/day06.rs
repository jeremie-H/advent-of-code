use std::error::Error;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    Ok(input.as_bytes().windows(4)
    .position(|s| s[0]!= s[1] && s[1] != s[2] && s[2] != s[3] && s[0] != s[2] && s[0]!= s[3] && s[1] != s[3])
    .unwrap() as i64 + 4)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    Ok(input.as_bytes().windows(14)
    .position(|s| {
        for i in 0..s.len()-1 {
            for j in i+1..s.len() {
                if s[i]==s[j] { return false; }
            }
        };
        true
    })
    .unwrap() as i64 + 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap(), 7);
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap(), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(), 26);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input06.txt")).unwrap(), 1850);
        assert_eq!(part2(include_str!("../inputs/input06.txt")).unwrap(), 2823);
    }
}
