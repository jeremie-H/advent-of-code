use std::{error::Error, collections::HashSet};

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let (_, x, y) : (i8,i64,i64) = input.split(", ")
        .fold((0,0,0), |(d,x,y), mouvement| {
            let depl = mouvement.split_at(1).1.parse::<i64>().unwrap();
            let d = (d + match mouvement.as_bytes()[0]  {
                b'L' => -1,
                b'R' => 1,
                _ => 0
            } ).rem_euclid(4);
            match d {
                0 => (d, x, y + depl),
                1 => (d, x + depl, y),
                2 => (d, x, y - depl),
                3 => (d, x - depl, y),
                _ => panic!("non pas ici {:?}",(d,x,y))
            }
        } );
    Ok(x.abs()+y.abs())
    
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut visited = HashSet::<(i64,i64)>::new();
    let mut d: i8 = 0;
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut found = false;
    visited.insert((0,0));
    input.split(", ")
        .find(|mouvement| {
            let depl = mouvement.split_at(1).1.parse::<i64>().unwrap();
            d = (d + match mouvement.as_bytes()[0]  {
                b'L' => -1,
                b'R' => 1,
                _ => 0
            } ).rem_euclid(4);
            for _ in 0..depl {
                match d {
                    0 => y += 1,
                    1 => x += 1,
                    2 => y -= 1,
                    3 => x -= 1,
                    _ => panic!("non pas ici {:?}",(x,y))
                };
                if visited.contains(&(x, y)) {
                    found=true;
                    break;
                }
                visited.insert((x, y));
            }
            found
        });
    Ok(x.abs()+y.abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("R2, L3").unwrap(), 5);
        assert_eq!(part1("R2, R2, R2").unwrap(), 2);
        assert_eq!(part1("R5, L5, R5, R3").unwrap(), 12);
        
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("R8, R4, R4, R8").unwrap(), 4);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input01.txt")).unwrap(), 181);
        assert_eq!(part2(include_str!("../inputs/input01.txt")).unwrap(), 140);
    }
}
