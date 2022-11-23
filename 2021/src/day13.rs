use colored::*;
use std::error::Error;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let folds = retrieve_all_folds(input);
    let taille_y = (folds.iter().find(|f| f.0 == 'y').unwrap().1) as i32 * 2;
    let taille_x = (folds.iter().find(|f| f.0 == 'x').unwrap().1) as i32 * 2;
    let map = generate_map(input, taille_x, taille_y);

    //on ne prend que le premier pliage
    let iter = folds.iter().take(1);
    let repli = pliage(map, iter);

    let result = repli.iter().fold(0, |acc, ligne| {
        acc + ligne.iter().fold(0, |acc, c| if *c == '#' { acc + 1 } else { acc })
    });
    Ok(result)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let folds = retrieve_all_folds(input);
    let taille_y = (folds.iter().find(|f| f.0 == 'y').unwrap().1) as i32 * 2;
    let taille_x = (folds.iter().find(|f| f.0 == 'x').unwrap().1) as i32 * 2;
    let map = generate_map(input, taille_x, taille_y);

    //on prend tous les pliages désormais
    let iter = folds.iter();
    let repli = pliage(map, iter);

    display(&repli);
   
    Ok(1)
// le résultat s'affiche à l'écran uniquement
}

fn generate_map(input: &str, taille_x: i32, taille_y: i32) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = vec![vec!['.'; (taille_x + 1) as usize]; (taille_y + 1) as usize];
    input.split_once("\n\n").unwrap().0.lines().for_each(|l| {
        let (x, y) = l.split_once(',').unwrap();
        map[y.parse::<usize>().unwrap()][x.parse::<usize>().unwrap()] = '#';
    });
    map
}

/**
 * ma "jolie" fonction de pliage de papier
 */
fn pliage<'a>(map: Vec<Vec<char>>, iter: impl Iterator<Item = &'a (char, i32)>) -> Vec<Vec<char>> {
    let mut repli = map;
    iter.for_each(|p| {
        match p.0 {
            'x' => {
                let taillex_fold = p.1 as usize;
                let mut prepa = vec![vec!['.'; (taillex_fold) as usize]; repli.len() as usize];
                for i in 0..repli.len() {
                    for j in 0..repli[0].len() {
                        if j == taillex_fold {
                            continue;
                        }
                        let newj = match j {
                            j if j < taillex_fold => j,
                            j if j > taillex_fold => (repli[0].len() - j) - 1,
                            j => j,
                        };
                        //println!("({},{}) => ({},{})", i, j, i, newj);
                        if prepa[i][newj] != '#' {
                            prepa[i][newj] = repli[i][j];
                        }
                    }
                }
                repli = prepa;
            }
            'y' => {
                let tailley_fold = p.1 as usize;
                let mut prepa = vec![vec!['.'; (repli[0].len()) as usize]; tailley_fold as usize];
                for i in 0..repli.len() {
                    if i == tailley_fold {
                        continue;
                    }
                    for j in 0..repli[0].len() {
                        let newi = match i {
                            i if i < tailley_fold => i,
                            i if i > tailley_fold => (repli.len() - i) - 1,
                            i => i,
                        };
                        // println!("({},{}) => ({},{})", i, j, newi, j);
                        if prepa[newi][j] != '#' {
                            prepa[newi][j] = repli[i][j];
                        }
                    }
                }
                repli = prepa;
            }
            _ => panic!("non prévu"),
        }
    });
    repli
}

/**
 * display array
 */
fn display(map: &[Vec<char>]) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '#' {
                print!("{}", "█".color(Color::BrightBlue));
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

/**
 * récupère les pliages
 */
fn retrieve_all_folds(input: &str) -> Vec<(char, i32)> {
    let folds = input.split_once("\n\n").unwrap().1.lines().fold(Vec::new(), |mut acc, f| {
        let gauchedroite = f.split_ascii_whitespace().last().unwrap().split_once('=').unwrap();
        acc.push((gauchedroite.0.chars().next().unwrap(), gauchedroite.1.parse::<i32>().unwrap()));
        acc
    });
    folds
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_part1() {
        // avec les données d'essai de la partie 1
        assert_eq!(part1(ÉNONCÉ).unwrap(), 17);
    }

    #[test]
    fn test_part2() {
        // avec les données d'essai de la partie 2
        assert_eq!(part2(ÉNONCÉ).unwrap(), 1);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input13.txt")).unwrap(), 785);
        assert_eq!(part2(include_str!("../inputs/input13.txt")).unwrap(), 1);
    }
}
