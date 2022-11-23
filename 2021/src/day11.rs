use colored::*;
use std::{collections::HashSet, error::Error, thread, time::Duration};

const DIRECTIONS: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

/**
 * Part 1
 */
#[allow(clippy::needless_range_loop)]
#[rustfmt::skip]
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut mignons_poulpes = input.lines().fold(Vec::new(), |mut tabl, ligne| {
        tabl.push(ligne.chars().fold(Vec::new(), |mut tabc, caract| {
            tabc.push(caract as isize - '0' as isize);
            tabc
        }));
        tabl
    });

    let hauteur = mignons_poulpes.len();
    let largeur = mignons_poulpes[0].len();
    let mut nbflashs = 0;
    for step in 0..100 {
        // display(&mignons_poulpes);

        //increase all with 1
        for i in 0..hauteur {
            for j in 0..largeur {
                mignons_poulpes[i][j] += 1;
            }
        }

        let mut reflash: HashSet<(isize, isize)> = HashSet::new();
        // flash arround
        for i in 0..hauteur {
            for j in 0..largeur {
                if mignons_poulpes[i][j] > 9 {
                    for direction in DIRECTIONS {
                        let deplacement_x = i as isize + direction.0;
                        let deplacement_y = j as isize + direction.1;
                        if deplacement_x >= 0 && deplacement_x < largeur as isize &&
                           deplacement_y >= 0 && deplacement_y < hauteur as isize
                        {
                            mignons_poulpes[deplacement_x as usize][deplacement_y as usize] += 1;
                            if mignons_poulpes[deplacement_x as usize][deplacement_y as usize] == 10
                                && (direction.0 == -1 || (direction.1 == -1 && direction.0 == 0))
                            {
                                reflash.insert((deplacement_x, deplacement_y));
                            }
                        }
                    }
                }
            }
        }
        // reflash neighbours
        let mut first_time = true;
        while !reflash.is_empty() || first_time {
            let mut reflash2 = HashSet::new();
            first_time = false;
            //println!("reflash : {:?}", reflash);
            reflash.iter().for_each(|(depx, depy)| {
                for direction in DIRECTIONS {
                    let deplacement_x = *depx as isize + direction.0;
                    let deplacement_y = *depy as isize + direction.1;
                    if deplacement_x >= 0 && deplacement_x < largeur as isize &&
                       deplacement_y >= 0 && deplacement_y < hauteur as isize {
                        mignons_poulpes[deplacement_x as usize][deplacement_y as usize] += 1;
                        if mignons_poulpes[deplacement_x as usize][deplacement_y as usize] == 10 {
                            reflash2.insert((deplacement_x, deplacement_y));
                        }
                    }
                }
            });
            reflash = reflash2;
        }
        // display(&mignons_poulpes);

        // set to 0 all flashed
        for i in 0..mignons_poulpes.len() {
            for j in 0..mignons_poulpes[0].len() {
                if mignons_poulpes[i][j] > 9 {
                    nbflashs += 1;
                    mignons_poulpes[i][j] = 0;
                }
            }
        }

        if mignons_poulpes.iter().all(|ligne| ligne.iter().all(|poulpe| *poulpe == 0_isize)) {
            println!("step = {}", step);
            break;
        }
        //display(&mignons_poulpes);

        
    }
    Ok(nbflashs)
}

/**
 * Part 2
 */
#[allow(clippy::needless_range_loop)]
#[rustfmt::skip]
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut mignons_poulpes = input.lines().fold(Vec::new(), |mut tabl, ligne| {
        tabl.push(ligne.chars().fold(Vec::new(), |mut tabc, caract| {
            tabc.push(caract as isize - '0' as isize);
            tabc
        }));
        tabl
    });

    let hauteur = mignons_poulpes.len();
    let largeur = mignons_poulpes[0].len();
    for step in 0..1000 {
        //increase all with 1
        for i in 0..hauteur {
            for j in 0..largeur {
                mignons_poulpes[i][j] += 1;
            }
        }

        let mut reflash: HashSet<(isize, isize)> = HashSet::new();
        // flash arround
        for i in 0..hauteur {
            for j in 0..largeur {
                if mignons_poulpes[i][j] > 9 {
                    for direction in DIRECTIONS {
                        let deplacement_x = i as isize + direction.0;
                        let deplacement_y = j as isize + direction.1;
                        if deplacement_x >= 0 && deplacement_x < largeur as isize &&
                           deplacement_y >= 0 && deplacement_y < hauteur as isize
                        {
                            mignons_poulpes[deplacement_x as usize][deplacement_y as usize] += 1;
                            if mignons_poulpes[deplacement_x as usize][deplacement_y as usize] == 10
                                && (direction.0 == -1 || (direction.1 == -1 && direction.0 == 0))
                            {
                                reflash.insert((deplacement_x, deplacement_y));
                            }
                        }
                    }
                }
            }
        }
        // reflash neighbours
        let mut first_time = true;
        while !reflash.is_empty() || first_time {
            let mut reflash2 = HashSet::new();
            first_time = false;

            //println!("reflash : {:?}", reflash);

            reflash.iter().for_each(|(depx, depy)| {
                for direction in DIRECTIONS {
                    let deplacement_x = *depx as isize + direction.0;
                    let deplacement_y = *depy as isize + direction.1;
                    if deplacement_x >= 0 && deplacement_x < largeur as isize &&
                       deplacement_y >= 0 && deplacement_y < hauteur as isize {
                        mignons_poulpes[deplacement_x as usize][deplacement_y as usize] += 1;
                        if mignons_poulpes[deplacement_x as usize][deplacement_y as usize] == 10 {
                            reflash2.insert((deplacement_x, deplacement_y));
                        }
                    }
                }
            });
            reflash = reflash2;
        }
        
        display_and_set_zeros(&mut mignons_poulpes, false);
        
        //check if all flash together
        if mignons_poulpes.iter().all(|ligne| ligne.iter().all(|poulpe| *poulpe == 0_isize)) {
            println!("step where all octopus flash : {}", step + 1);
            return Ok(step + 1);
        }
    }
    Ok(-1)
}

#[allow(dead_code)]
fn display(mignons_poulpes: &Vec<Vec<isize>>) {
    for i in 0..mignons_poulpes.len() {
        for j in 0..mignons_poulpes[0].len() {
            if mignons_poulpes[i][j] > 9 {
                print!("{}", ((mignons_poulpes[i][j] - 10) as u8 + b'a') as char);
            } else {
                print!("{}", mignons_poulpes[i][j]);
            }
        }
        println!();
    }
    println!("==========");
}

#[allow(dead_code)]
fn display_and_set_zeros(mignons_poulpes: &mut [Vec<isize>], display: bool) {
    if display {
        println!("\x1b[1J\x1b[A");
    }

    // set to 0 all flashed
    for i in 0..mignons_poulpes.len() {
        for j in 0..mignons_poulpes[0].len() {
            if mignons_poulpes[i][j] > 9 {
                mignons_poulpes[i][j] = 0;
                if display {
                    print!("{}", "0".color(Color::BrightCyan));
                }
            } else if display {
                print!("{}", mignons_poulpes[i][j]);
            }
        }
        if display {
            println!();
        }
    }
    if display {
        println!("==========");
        thread::sleep(Duration::from_millis(450));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_part1() {
        // avec les données d'essai de la partie 1
        assert_eq!(part1(ÉNONCÉ).unwrap(), 1656);
    }

    #[test]
    fn test_part2() {
        // avec les données d'essai de la partie 2
        assert_eq!(part2(ÉNONCÉ).unwrap(), 195);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input11.txt")).unwrap(), 1735);
        assert_eq!(part2(include_str!("../inputs/input11.txt")).unwrap(), 400);
    }
}
