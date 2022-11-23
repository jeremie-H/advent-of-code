use cached::proc_macro::cached;
use cached::SizedCache;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::error::Error;

/**
 * THOUGHS
 * 3 lancés de dés de dirac, on a les possibilitées suivantes :
 * 1 1 1 -> 3 (1 possibilitée)
 * 1 1 2 -> 4 (3 possibilitée)
 * 1 2 1 -> 4
 * 1 2 2 -> 5 (6 possibilitée)
 * 1 1 3 -> 5
 * 1 3 1 -> 5
 * 1 3 2 -> 6 (7 possibilitée)
 * 1 2 3 -> 6
 * 1 3 3 -> 7 (6 possibilitée)
 * 2 1 1 -> 4
 * 2 1 2 -> 5
 * 2 2 1 -> 5
 * 2 2 2 -> 6
 * 2 1 3 -> 6
 * 2 3 1 -> 6
 * 2 2 3 -> 7
 * 2 3 2 -> 7
 * 2 3 3 -> 8 (3 possibilitée)
 * 3 1 1 -> 5
 * 3 1 2 -> 6
 * 3 2 1 -> 6
 * 3 2 2 -> 7
 * 3 1 3 -> 7
 * 3 2 3 -> 8
 * 3 3 1 -> 7
 * 3 3 2 -> 8
 * 3 3 3 -> 9 (1 possibilitée)
 * ================================
 * Soit
 * - 1 possibilité  de faire 3
 * - 3 possibilités de faire 4
 * - 6 possibilités de faire 5
 * - 7 possibilités de faire 6
 * - 6 possibilités de faire 7
 * - 3 possibilités de faire 8
 * - 1 possibilité  de faire 9
 * 
 * 
 *   1   2   3   4   5   6   7   8   9   10
 * -----------------------------------------
 * |   |   |   |   |   |   |   |   |   |   |
 * -----------------------------------------
 * 
 * si P1 commence en 1
 * 
 *   1   2   3   4   5   6   7   8   9   10
 * "-----------------------------------------"
 * "| x |   |   | 1 | 3 | 6 | 7 | 6 | 3 | 1 |"
 * "-----------------------------------------"
 * "            | 4 | 5 | 6 | 7 | 8 | 9 | 10|" ⬅️ le score
 * 
 * passons P2 et revenons à P1
 * 
 * je n'ai plus de positions précises de P1, mais seulement un nombre d'univers par case
 * si P1 est en 4, alors on peut réappliquer les proba de 1,3,6,7,6,3,1
 * 
 *   1   2   3   4   5   6   7   8   9   10
 * "-------------------------------------------"
 * "|   |   |   | 1 | 3 | 6 | 7 | 6 | 3 | 1   |"
 * "|   |   |   | 1 | 3 | 6 |7*1|6*3|3*6|1*7  |" on reboucle au début
 * "| 6 | 3 | 1 | 1 | 3 | 6 |7*1|6*3|3*6|1*7  |"
 * "-------------------------------------------"
 * "            | 4 | 5 | 6 |7+7|8+8|9+9|10+10|" ⬅️ le score
 * 
 */

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut positions = input
        .lines()
        .map(|l| l.split_once(':').unwrap().1.trim().parse::<i32>().unwrap() - 1)
        .collect::<Vec<i32>>();
    println!("positions : {:?}", positions);

    let scores = (1..=100)
        .cycle()
        .tuples::<(_, _, _)>()
        .enumerate()
        .fold_while((0i32, 0i32, 0i64), |mut scores, (i, roll)| {
            if i % 2 == 0 {
                positions[0] = (positions[0] + roll.0 + roll.1 + roll.2) % 10;
                scores.0 += positions[0] + 1;
            } else {
                positions[1] = (positions[1] + roll.0 + roll.1 + roll.2) % 10;
                scores.1 += positions[1] + 1;
            }
            if scores.0 >= 1000 || scores.1 >= 1000 {
                scores.2 = scores.0.min(scores.1) as i64 * ((i + 1) as i64 * 3);
                Done(scores)
            } else {
                Continue(scores)
            }
        })
        .into_inner();

    println!("score = {:?}", scores);
    Ok(scores.2)
}


//1 possibilité de faire 3
//3 possibilité de faire 4, etc...
const ROLLS: [(i64, i64); 7] = [(1, 3), (3, 4), (6, 5), (7, 6), (6, 7), (3, 8), (1, 9)];


/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut positions = input
    .lines()
    .map(|l| l.split_once(':').unwrap().1.trim().parse::<i64>().unwrap())
    .collect::<Vec<i64>>();
    println!("positions : {:?}", positions);

    let mut scores = vec![0, 0];
    let scores = calcul_coup_suivant(0, &mut positions, &mut scores);

    let result = scores.0.max(scores.1);
    Ok(result)
}


/**
 * ici l'utilisation de memoïsation fait passer la résolution du challenge de 2 secondes à 28 ms 😲
 * énorme gain en utilisant ce cache.
 * En plus on peut calculer exactement sa taille connaissant les limites de chaque paramètres
 * possibilitées = 2 (tour) * 10 (positions Player1) * 10 (positions Player1) * 21 (score1) * 21 (score2) = 88200 !
 * On cale tout ça dans un tableau d'input typé : [0i64;5]; et on copie tous les paramètres dedans (cf convert)
 */
#[cached(
    type = "SizedCache<[i64;5], (i64,i64)>",
    create = "{ SizedCache::with_size(88200) }",
    convert = r#"{ let mut v = [0i64;5]; v[0] = positions[0]; v[1] = positions[1]; v[2] = scores[0]; v[3] = scores[1]; v[4] = tour as i64; v }"#
)]
fn calcul_coup_suivant(tour: usize, positions: &mut Vec<i64>, scores: &mut Vec<i64>) -> (i64, i64) {
    let copy_scores = scores.clone();
    let copy_positions = positions.clone();
    if scores[0] >= 21 {
        return (1, 0);
    }
    if scores[1] >= 21 {
        return (0, 1);
    }
    let mut result = (0, 0);
    for roll in ROLLS {
        let ajout = roll.1;
        let univers = roll.0;
        positions[tour] = (positions[tour] + ajout - 1) % 10 + 1;
        scores[tour] += positions[tour];
        let next = calcul_coup_suivant(1 - tour, positions, scores);
        result.0 += univers * next.0;
        result.1 += univers * next.1;
        scores[tour] = copy_scores[tour];
        positions[tour] = copy_positions[tour];
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 739785);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 444356092776315);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input21.txt")).unwrap(), 906093);
        assert_eq!(part2(include_str!("../inputs/input21.txt")).unwrap(), 274291038026362);
    }
}
