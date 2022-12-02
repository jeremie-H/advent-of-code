use std::error::Error;

use itertools::Itertools;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut iter = input.chars();
    iter.coalesce(|c1,c2| {
        if c1 ==  c2 {

        }
    });
    // let mut count = 1;
    // let mut previous = '-';
    // (1..40).for_each(|_|{
    // for c in iter.next()  {
    //     println!("{}", c);
    //     if previous == c {
    //         count+=1;
            
    //     }else {
    //         count = 1;
    //         break;
    //     }
    //     previous= c;
    // }});
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
        assert_eq!(part1("1").unwrap(), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("").unwrap(), 0);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        // assert_eq!(part1(include_str!("../inputs/input10.txt")).unwrap(), 0);
        // assert_eq!(part2(include_str!("../inputs/input10.txt")).unwrap(), 0);
    }
}
