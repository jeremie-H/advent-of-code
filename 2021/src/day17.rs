use std::{error::Error, ops::RangeInclusive};


//const CIBLE: [(i32, i32); 2] = [(143, 177), (-106, -71)];
// pour arriver au target x=143 minimum, il faut commencer au moins à 17 en vélocité x
//const RANGEX: RangeInclusive<i32> = 17..=CIBLE[0].1;
//const RANGEY: RangeInclusive<i32> = (CIBLE[1].0)..=-CIBLE[1].0;

/**
 * Part 1
 *
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let cible = read_cible(input);
    let mut compteur = 0;
    for i in (cible[1].0+1) as i64..0i64 {
        compteur += i.abs();
    }
    Ok(compteur)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let cible = read_cible(input);
    let range_x : RangeInclusive<i32> = 1..=cible[0].1;
    let range_y : RangeInclusive<i32> = (cible[1].0)..=-cible[1].0;
    let mut compteur = 0;
    for i in range_x {
        for j in range_y.clone().rev() {
            let velocity = (i, j);
            let position = (0, 0);
            let mut tab = [velocity, position];
            let mut check = -2;
            while check <= 0 {
                tab = nextstep(tab[0], tab[1]);
                check = check_target(&cible, tab[1]);
                if check == 0 {
                    //println!("Ok : {:?}", velocity);
                    compteur += 1;
                    break;
                }
                //println!("next position : {:?}", tab);
            }
        }
    }
    Ok(compteur)
}

/**
 * read input like "target area: x=20..30, y=-10..-5" => [(20,30),(-10,-5)]
 */
fn read_cible(input: &str) -> Vec<(i32,i32)> {
    input.strip_prefix("target area: x=").unwrap()
    .split(", y=")
    .map(|f| {
        let (a,b) = f.split_once("..").unwrap();
        (a.parse::<i32>().unwrap(),b.parse::<i32>().unwrap())
    })
    .collect::<Vec<(i32,i32)>>()
}

fn nextstep(velocity: (i32, i32), pos: (i32, i32)) -> [(i32, i32); 2] {
    let next_pos = (pos.0 + velocity.0, pos.1 + velocity.1);
    let velocity = (
        match velocity.0 {
            v if v == 0 => 0,
            v if v > 0 => v - 1,
            v => v + 1,
        },
        velocity.1 - 1,
    );
    [velocity, next_pos]
}

fn check_target(cible: &[(i32,i32)], pos: (i32, i32)) -> i32 {
    match pos {
        pos if pos.1 < cible[1].0 => 1, // tombé en dessous de la target
        pos if pos.0 >= cible[0].0 && pos.0 <= cible[0].1 && pos.1 >= cible[1].0 && pos.1 <= cible[1].1 => 0,
        _ => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 45);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 112);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input17.txt")).unwrap(), 5565);
        assert_eq!(part2(include_str!("../inputs/input17.txt")).unwrap(), 2118);
    }
}
