use std::error::Error;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let input = input.split_once("\n").unwrap();
    let races = input.0.split_whitespace().into_iter()
    .filter_map(|s| s.parse::<f64>().ok())
    .zip(input.1.split_whitespace().into_iter().filter_map(|s| s.parse::<f64>().ok()))
    .collect::<Vec<(f64,f64)>>();
    
    let winning_ways: f64 = races.iter().map(|&(time, distance)| {
        calcul_winning_way(time, distance)
    }).product();

    Ok(winning_ways as i64)
}


fn calcul_winning_way(time: f64, distance: f64) -> f64 {
    let racine_carre = (time * time - 4.0 * distance).sqrt();
    let x1 = (time - racine_carre) / 2.0;
    let x2 = (time + racine_carre) / 2.0;

    let mut ways_to_win = x2.floor() - x1.floor();
    if x2.floor() == x2 {
        ways_to_win -= 1.0
    }
    ways_to_win
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let input = input.split_once("\n").unwrap();
    let time = input.0.split_once(":").unwrap().1.replace(" ", "").parse::<f64>().unwrap();
    let distance = input.1.split_once(":").unwrap().1.replace(" ", "").parse::<f64>().unwrap();
    let winning_way = calcul_winning_way(time, distance);
    Ok(winning_way as i64)
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
