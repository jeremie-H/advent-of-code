use colored::*;
use std::{cmp, error::Error};

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let lava = input.split('\n').fold(Vec::new(), |mut acc, ligne| {
        acc.push(ligne);
        acc
    });
    let mut compteur = 0;
    for i in 0..lava.len() {
        for j in 0..lava[i].len() {
            let audessus = if i == 0 { i + 1 } else { i - 1 };
            let audessous = if i == lava.len() - 1 { lava.len() - 2 } else { i + 1 };
            let value = lava[i].as_bytes()[j] - b'0';
            if value < cmp::min(lava[audessus].as_bytes()[j] - b'0', lava[audessous].as_bytes()[j] - b'0') {
                let agauche = if j == 0 { 1 } else { j - 1 };
                let adroite = if j == lava[i].len() - 1 { lava[i].len() - 2 } else { j + 1 };

                if value < cmp::min(lava[i].as_bytes()[agauche] - b'0', lava[i].as_bytes()[adroite] - b'0') {
                    compteur += (lava[i].as_bytes()[j] - b'0') as i32 + 1;
                }
            }
        }
    }
    Ok(compteur as i64)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut lac_de_lave = input.lines().fold(Vec::new(), |mut tabl, ligne| {
        tabl.push(ligne.chars().fold(Vec::new(), |mut tabc, caract| {
            tabc.push(caract as isize - '0' as isize);
            tabc
        }));
        tabl
    });

    let hauteur = lac_de_lave.len();
    let largeur = lac_de_lave[0].len();

    // search basin
    let mut basin_inc = 10;
    for i in 0..hauteur {
        for j in 0..largeur {
            if lac_de_lave[i][j] < 9 {
                lac_de_lave[i][j] = basin_inc;

                dig_dans_la_lave(&mut lac_de_lave, i, j, largeur, hauteur, basin_inc);

                basin_inc += 1;
            }
        }
    }

    let mut lesbassins = Vec::new();
    for increment_teste in 10..=basin_inc {
        lesbassins.push(lac_de_lave.iter().fold(0, |acc, ligne| {
            acc + ligne.iter().fold(0, |mut acc, point| {
                if *point == increment_teste {
                    acc += 1;
                }
                acc
            })
        }));
    }

    // println!("après flash");
    display(&lac_de_lave);
    lesbassins.sort_by(|a, b| b.cmp(a));
    let result = lesbassins.iter().take(3).product::<i64>();
    println!("les bassins les plus gros {:?}", result);
    Ok(result)
}

const COULEURS: [colored::Color; 14] = [
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Blue,
    Color::Magenta,
    Color::Cyan,
    Color::BrightBlack,
    Color::BrightRed,
    Color::BrightGreen,
    Color::BrightYellow,
    Color::BrightBlue,
    Color::BrightMagenta,
    Color::BrightCyan,
    Color::BrightWhite,
];

fn dig_dans_la_lave(lac_de_lave: &mut Vec<Vec<isize>>, i: usize, j: usize, largeur: usize, hauteur: usize, basin_inc: isize) -> () {
    DIRECTIONS.iter().for_each(|(x, y)| {
        let deplacement_x = i as isize + x;
        let deplacement_y = j as isize + y;
        if deplacement_x >= 0 && deplacement_x < hauteur as isize && deplacement_y >= 0 && deplacement_y < largeur as isize {
            if lac_de_lave[deplacement_x as usize][deplacement_y as usize] < 9 {
                lac_de_lave[deplacement_x as usize][deplacement_y as usize] = basin_inc;
                dig_dans_la_lave(
                    lac_de_lave,
                    deplacement_x as usize,
                    deplacement_y as usize,
                    largeur,
                    hauteur,
                    basin_inc,
                );
            }
        }
    });
}

#[allow(dead_code)]
fn display(tableau: &Vec<Vec<isize>>) {
    for i in 0..tableau.len() {
        for j in 0..tableau[0].len() {
            if tableau[i][j] > 9 {
                let index = (tableau[i][j] as usize) % COULEURS.len();
                print!("{}", format!("{}", '█').color(COULEURS[index]));
            } else {
                print!("{}", format!("{}", '█').color(Color::Black));
            }
        }
        println!();
    }
    println!("==========");
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 1134);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input09.txt")).unwrap(), 594);
        assert_eq!(part2(include_str!("../inputs/input09.txt")).unwrap(), 858494);
    }
}
