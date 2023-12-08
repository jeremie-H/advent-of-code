use std::{error::Error, collections::HashMap};

use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};

fn load_data(input: &str) -> (&str, HashMap<&str, (&str,&str)>) {
    let (directions, map)= input.split_once("\n\n").unwrap();
    let map = map.lines()
    .fold(HashMap::<&str, (&str,&str)>::with_capacity(800), |mut map, l|{
        map.insert(&l[..3], (&l[7..10], &l[12..15]));
        map
    });
    (directions, map)
}

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let (directions, map) = load_data(input);
    let r = directions.chars()
    .cycle()
    .fold_while((1, "AAA"), |(i, mut branche), direction| {
        branche = match direction {
            'R' => map.get(branche).unwrap().1,
            'L' => map.get(branche).unwrap().0,
            _ => panic!("direction inconnue")
        };
        if branche.eq("ZZZ") {
            Done((i, branche))
        }
        else {
            Continue((i + 1, branche))
        }
    }).into_inner().0;
    // println!("map : {:?}", map);
    // println!("r : {:?}", r);
    Ok(r as i64)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let (directions, map) = load_data(input);
    let debuts = map.keys().filter(|s|s.as_bytes()[2] == b'A').collect_vec();
    let r = directions.chars()
    .cycle()
    .fold_while((1, debuts), |(i, mut tableau), direction| {
        let mut fin = true;
        println!("tab : {:?}", tableau);
        fin = match direction {
            'R' => {
                for i in 0..tableau.len() {
                    
                    tableau[i] = &map.get(tableau[i]).unwrap().1;
                    fin &= tableau[i].as_bytes()[2] == b'Z';
                    
                }
                fin
            },
            'L' => {
                for i in 0..tableau.len() {
                    
                    tableau[i] = &map.get(tableau[i]).unwrap().0;
                    fin &= tableau[i].as_bytes()[2] == b'Z';
                }
                fin
            }
            _ => panic!("direction inconnue")
        };
        if fin {
            Done((i, tableau))
        }
        else {
            Continue((i + 1, tableau))
        }
    }).into_inner().0;
    // println!("map : {:?}", map);
    // println!("r : {:?}", r);
    Ok(r as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)").unwrap(), 2);
        assert_eq!(part1("LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)").unwrap(), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)").unwrap(), 6);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input08.txt")).unwrap(), 21883);
        // assert_eq!(part2(include_str!("../inputs/input08.txt")).unwrap(), 0);
    }
}
