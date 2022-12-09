use std::{error::Error, collections::HashSet};

//ULDR
const DIR:[(i32,i32);4]=[(1,0),(0,-1),(-1,0),(0,1)];

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let moves = input.lines()
    .map(|l|l.split_once(' ').unwrap())
    .map(|(d,n)| (d.chars().next().unwrap(), n.parse::<i32>().unwrap()))
    .map(|(d,n)| (match d {
        'U' => 0,
        'L' => 1,
        'D' => 2,
        'R' => 3,
        _ => panic!("hey !")
    },n))
    .collect::<Vec<(usize,i32)>>();

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

fn calcul_new_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    match (head.0-tail.0,head.1-tail.1){
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

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let moves = read_moves(input);

    let mut positions = HashSet::new();
    positions.insert((0,0));//always at least this position
    
    let mut head: (i32,i32)=(0,0);
    let mut tail1: (i32,i32)=(0,0);
    let mut tail2: (i32,i32)=(0,0);
    let mut tail3: (i32,i32)=(0,0);
    let mut tail4: (i32,i32)=(0,0);
    let mut tail5: (i32,i32)=(0,0);
    let mut tail6: (i32,i32)=(0,0);
    let mut tail7: (i32,i32)=(0,0);
    let mut tail8: (i32,i32)=(0,0);
    let mut tail9: (i32,i32)=(0,0);

    moves.iter().for_each(|(d,n)|{
        for _ in 0..*n {
            head = (head.0+DIR[*d].0, head.1+DIR[*d].1);
            tail1 = calcul_new_tail(head,tail1);
            tail2 = calcul_new_tail(tail1,tail2);
            tail3 = calcul_new_tail(tail2,tail3);
            tail4 = calcul_new_tail(tail3,tail4);
            tail5 = calcul_new_tail(tail4,tail5);
            tail6 = calcul_new_tail(tail5,tail6);
            tail7 = calcul_new_tail(tail6,tail7);
            tail8 = calcul_new_tail(tail7,tail8);
            tail9 = calcul_new_tail(tail8,tail9);
            positions.insert(tail9);
        }
    });

    Ok(positions.len() as i64)
}

fn read_moves(input: &str) -> Vec<(usize, i32)> {
    input.lines()
    .map(|l|l.split_once(' ').unwrap())
    .map(|(d,n)| (d.chars().next().unwrap(), n.parse::<i32>().unwrap()))
    .map(|(d,n)| (match d {
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

    const ÉNONCÉ: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

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
