use std::error::Error;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let calcul = |fuel, position: i32, tentative: i32| fuel + (position - tentative).abs();
    Ok(loop_crabes(input, calcul, false))
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let calcul = |fuel, position: i32, tentative: i32| {
        let n = (position - tentative).abs();
        fuel + n * (n + 1) / 2
    };
    Ok(loop_crabes(input, calcul, true))
}

/**
 * loop on each position
 */
fn loop_crabes<F: Fn(i32, i32, i32) -> i32>(input: &str, closure_calcul: F, mean: bool) -> i64 {
    let mut positions = input.split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut minfuel: i32 = i32::MAX;

    let median_or_mid = match mean {
        //moyenne
        true => positions.iter().sum::<i32>() / positions.len() as i32,
        //mediane
        false => {
            let mid = positions.len() / 2;
            *positions.select_nth_unstable(mid).1
        }
    };

    for tentative in median_or_mid..median_or_mid + 100 {
        let fuel = positions
            .iter()
            .fold(0, |fuel, &position| closure_calcul(fuel, position, tentative as i32));
        minfuel = minfuel.min(fuel);
    }
    //println!("fuel dépensé par les 🦀 = {}", minfuel);

    minfuel as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("16,1,2,0,4,2,7,1,2,14").unwrap(), 37);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("16,1,2,0,4,2,7,1,2,14").unwrap(), 168);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input07.txt")).unwrap(), 352997);
        assert_eq!(part2(include_str!("../inputs/input07.txt")).unwrap(), 101571302);
    }
}
