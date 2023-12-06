use std::error::Error;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let input = input.split_once("\n").unwrap();
    let races = input.0.split_whitespace().into_iter()
    .filter_map(|s| s.parse::<i64>().ok())
    .zip(input.1.split_whitespace().into_iter().filter_map(|s| s.parse::<i64>().ok()))
    .collect::<Vec<(i64,i64)>>();
    
    let winning_ways: i64 = races.iter().map(|&(time, distance)| {
        let mut winning_way = 0;
        for t in 1..time {
            if  (time-t)*t > distance {
                winning_way += 1;
            }
        }
        winning_way
    }).product();

    Ok(winning_ways)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let input = input.split_once("\n").unwrap();
    let time = input.0.split_once(":").unwrap().1.replace(" ", "").parse::<i64>().unwrap();
    let distance = input.1.split_once(":").unwrap().1.replace(" ", "").parse::<i64>().unwrap();
    
    let mut winning_way: i64 = 0;
    for t in 1..time {
        if  (time-t)*t > distance {
            winning_way += 1;
        }
    }
    
    Ok(winning_way)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("Time:      7  15   30\nDistance:  9  40  200").unwrap(), 288);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("Time:      7  15   30\nDistance:  9  40  200").unwrap(), 71503);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input06.txt")).unwrap(), 2269432);
        assert_eq!(part2(include_str!("../inputs/input06.txt")).unwrap(), 35865985);
    }
}
