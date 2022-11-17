use std::error::Error;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let result: i64 = input.chars()
    .fold(0, |acc: i64, caractere| {
        acc + match caractere {
            '(' => 1,
            ')' => -1,
            _ => 0
        }
    });
    Ok(result)
}


/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut floor:i64 = 0;
    let result = 0;
    for (i, step) in input.chars()
    .map(| step| match step {
        '(' => 1,
        ')' => -1,
        _ => 0
    })
    .enumerate() {
        floor += step;
        if floor == -1 {
            return Ok(i as i64 +1);
        }
    };
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // avec les données d'essai
        assert_eq!(part1("(()(()(").unwrap(), 3);
        assert_eq!(part1("))(((((").unwrap(), 3);
        assert_eq!(part1(")())())").unwrap(), -3);
    }

    #[test]
    fn test_part2() {
        // avec les données d'essai
        assert_eq!(part2(")").unwrap(), 1);
        assert_eq!(part2("()())").unwrap(), 5);
    }
    

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("input.txt")).unwrap(), 138);
        assert_eq!(part2(include_str!("input.txt")).unwrap(), 1771);
    }
}
