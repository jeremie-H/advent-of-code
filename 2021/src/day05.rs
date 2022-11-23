use std::{
    cmp::{max, min},
    error::Error,
};
/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let deplacements = load_deplacements(&input);

    //init ocean
    let mut ocean = init_ocean(&deplacements);

    //process
    deplacements
        .iter()
        //filtre uniquement sur les horizontales et verticales
        .filter(|depl| depl[0].0 == depl[1].0 || depl[0].1 == depl[1].1)
        .for_each(|depl| {
            trace_line(depl, &mut ocean);
        });

    //compute & display ocean
    Ok(score_ocean(&ocean, ocean.len() == 10))
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let deplacements = load_deplacements(&input);

    //init ocean
    let mut ocean = init_ocean(&deplacements);

    //process
    deplacements.iter().for_each(|depl| {
        trace_line(depl, &mut ocean);
    });

    //compute & display ocean
    Ok(score_ocean(&ocean, ocean.len() == 10))
}

/**
 * load deplacements from inputs
 */
#[rustfmt::skip]
fn load_deplacements(input: &str) -> Vec<Vec<(i32, i32)>> {
    input.split('\n')
        .map(|ligne| {
            ligne
                .split(" -> ")
                .map(|pos| {
                    pos.split(',').fold((-1, -1), |(x, _y), coord| {
                        let coord_entier = coord.parse().unwrap();
                        if x == -1 { (coord_entier, -1) }
                        else { (x, coord_entier) }
                    })
                })
                .fold(vec![(0, 0); 0], |mut depl, (x, y)| {
                    depl.push((x, y));
                    depl
                })
        })
        .collect::<Vec<Vec<(i32, i32)>>>()
}

/**
 * initialize the ocean from the input.txt
 */
fn init_ocean(deplacements: &[Vec<(i32, i32)>]) -> Vec<Vec<i32>> {
    let taille_ocean = match deplacements[0][0].0 {
        x if x > 10 => 1000,
        _ => 10,
    };
    vec![vec![0_i32; taille_ocean]; taille_ocean]
}

/**
 * compute the score of the resulting ocean
 */
#[rustfmt::skip]
fn score_ocean(ocean: &[Vec<i32>], display: bool) -> i64 {
    let mut compteur = 0;
    ocean.iter().for_each(|ligne| {
        ligne.iter().for_each(|col| {
            if display {
                if *col == 0 { print!(".") }
                else { print!("{}", col) }
            }
            if *col > 1 { compteur += 1 }
        });
        if display {
            println!()
        }
    });
    compteur
}

/**
 * trace the line between two points (x1,y1) and (x2,y2)
 */
#[rustfmt::skip]
fn trace_line(depl: &[(i32, i32)], ocean: &mut [Vec<i32>]) {
    let x1 = depl[0].0;
    let y1 = depl[0].1;
    let x2 = depl[1].0;
    let y2 = depl[1].1;

    //gestion horizontales et verticales
    if x1 == x2 {
        for i in min(y1, y2)..=max(y1, y2) {
            ocean[i as usize][x1 as usize] += 1;
        }
    } else if y1 == y2 {
        for j in min(x1, x2)..=max(x1, x2) {
            ocean[y1 as usize][j as usize] += 1;
        }
    }
    //gestion diagonales
    else {
        let mut j = 0;
        let mut inc: i32 = 0;
        for i in min(y1, y2)..=max(y1, y2) {
            if i == y1 {
                j = x1;
                if x1 < x2 {inc = 1} else {inc = -1}
            }
            if i == y2 {
                j = x2;
                if x1 > x2 {inc = 1} else {inc = -1}
            }
            ocean[i as usize][j as usize] += 1;
            j += inc;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 12);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input05.txt")).unwrap(), 8111);
        assert_eq!(part2(include_str!("../inputs/input05.txt")).unwrap(), 22088);
    }
}
