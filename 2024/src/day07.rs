use std::error::Error;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    Ok(
        input.lines()
        .filter_map(|line| {
            let (total, equation) = line.split_once(": ")?;
            let total: i64 = total.parse().ok()?;
            let nombres: Vec<i64> = equation
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            if eval_expression(&nombres, total) {
                Some(total)
            } else {
                None
            }
        })
        .sum())
}

fn eval_expression(nombres: &[i64], total: i64) -> bool {
    let mut results = vec![nombres[0]];
    for &num in &nombres[1..] {
        for i in 0..results.len() {
            results.push(results[i] + num); // Addition
            results[i] *= num;               // Multiplication en place
        }
    }
    results.contains(&total)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    Ok(input.lines()
    .filter_map(|line| {
        let (total, equation) = line.split_once(": ")?;
        let total: i64 = total.parse().ok()?;
        let numbers: Vec<i64> = equation
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        if eval_expression2(&numbers, total) {
            Some(total)
        } else {
            None
        }
    })
    .sum())
}

fn eval_expression2(nombres: &[i64], total: i64) -> bool {
    if nombres.len() == 2 {
        let first = nombres[0];
        let last = nombres[1];
        return first * last == total
            || first + last == total
            || first * 10_i64.pow(last.ilog10()+1) + last == total;
    }
    
    eval_expression2(&nombres[0..&nombres.len() - 1], total - nombres.last().unwrap())
    || {
        let concat_mul = 10_i64.pow(nombres.last().unwrap().ilog10() + 1);
        (total - nombres.last().unwrap()) % concat_mul == 0 && eval_expression2(&nombres[0..&nombres.len() - 1], (total - nombres.last().unwrap()) / concat_mul)
    }
    || total % nombres.last().unwrap() == 0 && eval_expression2(&nombres[0..&nombres.len() - 1], total / nombres.last().unwrap())
}


#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 11387);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input07.txt")).unwrap(), 4122618559853);
        assert_eq!(part2(include_str!("../inputs/input07.txt")).unwrap(), 227615740238334);
    }
}
