use crate::day06grid::Grid;
use std::error::Error;

/**
 * Part 1
 */
#[allow(clippy::needless_range_loop)] //les boucles proposées par clippy sont illisibles
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let instructions = read_instructions(input);
    let instructions = instructions.iter().map(|(i, c1, c2)| Grid::from_str(i, c1, c2)).collect::<Vec<Grid>>();

    let mut grille: [[bool; 1000]; 1000] = [[false; 1000]; 1000];
    for k in &instructions {
        for i in k.corner_a.0..=k.corner_b.0 {
            for j in k.corner_a.1..=k.corner_b.1 {
                grille[i][j] = match k.state {
                    0 => false,
                    1 => true,
                    2 => !grille[i][j],
                    _ => panic!("unknown state"),
                }
            }
        }
    }
    let count = grille
        .iter()
        .fold(0, |compteur, ligne| compteur + ligne.iter().fold(0, |acc, state| if *state { acc + 1 } else { acc }));

    Ok(count)
}

/**
 * décompose une instruction en tuple de la forme
 * ("turn on", "499,499", "500,500")
 * ("toggle", "0,499", "100,400")
 */
fn read_instructions(input: &str) -> Vec<(&str, &str, &str)> {
    input
        .lines()
        .map(|l| l.split_once(" through ").unwrap())
        .map(|(l1, l2)| (l1.rsplit_once(' ').unwrap(), l2))
        .map(|((s1, s2), s3)| (s1, s2, s3))
        .collect::<Vec<(&str, &str, &str)>>()
}

/**
 * Part 2
 */
#[allow(clippy::needless_range_loop)] //les boucles proposées par clippy sont illisibles
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let instructions = read_instructions(input);
    let instructions = instructions.iter().map(|(i, c1, c2)| Grid::from_str(i, c1, c2)).collect::<Vec<Grid>>();

    let mut grille: [[i8; 1000]; 1000] = [[0i8; 1000]; 1000];
    for k in &instructions {
        for i in k.corner_a.0..=k.corner_b.0 {
            for j in k.corner_a.1..=k.corner_b.1 {
                grille[i][j] = match k.state {
                    0 => {
                        if grille[i][j] <= 1 {
                            0
                        } else {
                            grille[i][j] - 1
                        }
                    }
                    1 => grille[i][j] + 1,
                    2 => grille[i][j] + 2,
                    _ => panic!("unknown state"),
                }
            }
        }
    }
    let count = grille.iter().fold(0, |compteur, ligne| compteur + ligne.iter().fold(0, |acc, state| acc + (*state as i64)));

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500";
        assert_eq!(part1(input).unwrap(), 998996);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("turn on 0,0 through 0,0\ntoggle 0,0 through 999,999").unwrap(), 2000001);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input06.txt")).unwrap(), 377891);
        assert_eq!(part2(include_str!("../inputs/input06.txt")).unwrap(), 14110788);
    }
}
