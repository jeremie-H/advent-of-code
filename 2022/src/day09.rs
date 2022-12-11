use std::{error::Error, collections::HashSet};

//Up / Left / Down / Right
const DIR:[(i16,i16);4]=[(1,0),(0,-1),(-1,0),(0,1)];

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let moves = read_moves(input);
    let mut positions = HashSet::new();
    positions.insert((0,0));
    let mut head = (0i16,0i16);
    let mut tail = (0i16,0i16);

    moves.for_each(|(d,n)|{
        for _ in 0..n {
            head = (head.0+DIR[d].0, head.1+DIR[d].1);
            calcul_new_tail(head,&mut tail);
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

    moves.for_each(|(d,n)|{
        for _ in 0..n {
            alaqueueleuleu[0] = (alaqueueleuleu[0].0+DIR[d].0, alaqueueleuleu[0].1+DIR[d].1);
            for i in 1..alaqueueleuleu.len() {
                calcul_new_tail(alaqueueleuleu[i-1], &mut alaqueueleuleu[i]);
            }
            positions.insert(alaqueueleuleu[alaqueueleuleu.len()-1]);
        }
    });

    Ok(positions.len() as i64)
}

/**
 * évalue toutes les différentes positions possibles
 */
#[allow(clippy::comparison_chain)]
fn calcul_new_tail(head: (i16, i16), tail: &mut (i16, i16)) {
    if tail.0.abs_diff(head.0) >= 2 || tail.1.abs_diff(head.1) >= 2 {
        if tail.0 < head.0 { tail.0 += 1;}
        else if tail.0 > head.0 { tail.0 -= 1; }

        if tail.1 < head.1 { tail.1 += 1;}
        else if tail.1 > head.1 { tail.1 -= 1; }
    }
}

fn read_moves(input: &str) -> impl Iterator<Item=(usize,i16)> + '_ {
    input.lines()
    .map(|l|l.split_once(' ').unwrap())
    .map(|(d,n)| (match d.chars().next().unwrap() {//on map les directions pour que ce soit facile de taper sur les indices de DIR
        'U' => 0,
        'L' => 1,
        'D' => 2,
        'R' => 3,
        _ => panic!("hey !")
    },n.parse::<i16>().unwrap()))
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
