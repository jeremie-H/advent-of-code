use std::{error::Error, collections::HashSet};

//ULDR
const DIR:[(i32,i32);4]=[(1,0),(0,-1),(-1,0),(0,1)];

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let moves = read_moves(input);
    let mut positions = HashSet::new();
    positions.insert((0,0));
    let mut head: (i32,i32)=(0,0);
    let mut tail: (i32,i32)=(0,0);

    moves.iter().for_each(|(d,n)|{
        for _ in 0..*n {
            head = (head.0+DIR[*d].0, head.1+DIR[*d].1);
            tail = calcul_new_tail(head,tail);
            positions.insert(tail);
        }
    });

    Ok(positions.len() as i64)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let moves = read_moves(input);

    let mut positions = HashSet::new();
    positions.insert((0,0));//always at least this position
    let mut alaqueueleuleu = [(0,0);10];

    moves.iter().for_each(|(d,n)|{
        for _ in 0..*n {
            alaqueueleuleu[0] = (alaqueueleuleu[0].0+DIR[*d].0, alaqueueleuleu[0].1+DIR[*d].1);
            for i in 1..alaqueueleuleu.len() {
                alaqueueleuleu[i] = calcul_new_tail(alaqueueleuleu[i-1], alaqueueleuleu[i]);
            }
            positions.insert(alaqueueleuleu[alaqueueleuleu.len()-1]);
        }
    });

    Ok(positions.len() as i64)
}

/**
 * évalue toutes les différentes positions possibles
 */
fn calcul_new_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    match (head.0-tail.0,head.1-tail.1) {
        (0,0) | (1,0) | (0,1) | (1,1) | (-1,0) | (0,-1) | (-1,-1) | (1,-1) | (-1,1) => tail,
        (0,2) => (tail.0,tail.1+1),
        (2,0) => (tail.0+1,tail.1),
        (0,-2) => (tail.0,tail.1-1),
        (-2,0) => (tail.0-1,tail.1),
        (2,1) | (1,2) => (tail.0+1,tail.1+1),
        (-2,1) | (-1,2) => (tail.0-1,tail.1+1),
        (1,-2) | (2,-1) => (tail.0+1,tail.1-1),
        (-1,-2) | (-2,-1) => (tail.0-1,tail.1-1),
        (2,2) => (tail.0+1,tail.1+1),
        (2,-2) => (tail.0+1,tail.1-1),
        (-2,-2) => (tail.0-1,tail.1-1),
        (-2,2) => (tail.0-1,tail.1+1),
        (a,b) => panic!("pas possible avec ({},{})",a,b)
    }
}

fn read_moves(input: &str) -> Vec<(usize, i32)> {
    input.lines()
    .map(|l|l.split_once(' ').unwrap())
    .map(|(d,n)| (d.chars().next().unwrap(), n.parse::<i32>().unwrap()))
    .map(|(d,n)| (match d {//on map les directions pour que ce soit facile de taper sur les indices de DIR
        'U' => 0,
        'L' => 1,
        'D' => 2,
        'R' => 3,
        _ => panic!("hey !")
    },n))
    .collect::<Vec<(usize,i32)>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 1);
        assert_eq!(part2("R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20").unwrap(), 36);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input09.txt")).unwrap(), 6406);
        assert_eq!(part2(include_str!("../inputs/input09.txt")).unwrap(), 2643);
    }
}
