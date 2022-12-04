use std::error::Error;

use itertools::Itertools;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut alphabet = [false;26*2];
    Ok(input.lines()
    .map(|ligne|ligne.split_at(ligne.len()/2))
    .map(|(gauche, droite)| {
        alphabet.fill(false);
        gauche.as_bytes().iter().for_each(|lettre| {
            alphabet[indice(lettre)]=true
        });
        indice(droite.as_bytes().iter().find(|&lettre|{
            alphabet[indice(lettre)]
        }).unwrap()) as i64 + 1
    })
    .sum())
    
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut alphabet = [0u8;26*2];
    Ok(input.lines()
    .chunks(3)
    .into_iter()
    .map(|groups| {
        alphabet.fill(0);
        groups.map(|s| s.as_bytes())
        .enumerate()
        .for_each(|(i,c)| {
            for ca in c{
                alphabet[indice(ca)]|=1<<i;
            }
        });
        (alphabet.iter().position(|&c| c==7).unwrap() as i64) + 1
    })
    .sum())
}


fn indice(lettre: &u8) -> usize {
    match lettre {
        b'a'..=b'z' => (lettre-b'a') as usize,
        b'A'..=b'Z' => (26+lettre-b'A') as usize,
        _ => panic!("lettre non reconnue")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 70);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input03.txt")).unwrap(), 7878);
        assert_eq!(part2(include_str!("../inputs/input03.txt")).unwrap(), 2760);
    }
}
