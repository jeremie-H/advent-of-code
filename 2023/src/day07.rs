use std::{error::Error, cmp::Ordering};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Strength {
    Rien = 0,
    Paire = 1,
    DoublePaire = 2,
    Brelan = 3,
    Full = 4,
    Carré = 5,
    Five = 6
}

#[derive(Debug, Clone, PartialEq, Eq, Ord)]
struct Hand {
    game: Vec<u8>,
    bid: i64,
    strength: Strength
}

impl Hand{
    fn new(game: &[u8], bid: i64, part2: bool) -> Hand {
        let mut allcards = [0u8; 15];
        let mut gamecopy: Vec<u8> = Vec::new();
        let mut nombre_de_joker = 0;
        for i in 0..5 {
            let card: usize = match game[i] {
                b'A' => 14,
                b'K' => 13,
                b'Q' => 12,
                b'J' => {
                    if part2 { nombre_de_joker += 1; 1 }
                    else { 11 }
                },
                b'T' => 10,
                chiffre => (chiffre - b'0') as usize,
            };
            gamecopy.push(card as u8);
            allcards[card] += 1;
        }
        allcards[1] = 0;
        allcards.sort_by(|a, b| b.cmp(a));
        allcards[0] += nombre_de_joker;

        let strength = match allcards[0] {
            5 => Strength::Five,
            4 => Strength::Carré,
            3 if allcards[1] == 2 => Strength::Full,
            3 if allcards[1] < 2 => Strength::Brelan,
            2 if allcards[1] == 2 => Strength::DoublePaire,
            2 if allcards[1] < 2 => Strength::Paire,
            _ => Strength::Rien
        };
        Hand { game: gamecopy, bid, strength }
    }
}

impl PartialOrd for Hand {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.strength == other.strength {
            let o = Ordering::Equal;
            for i in 0..5 {
                if self.game[i] != other.game[i] {
                    return self.game[i].partial_cmp(&other.game[i]);
                }
            }
            return Some(o);
        }
        else {
            return Some(self.strength.cmp(&other.strength));
        }
    }
    
}

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    Ok(input.lines()
    .map(|s| s.split_once(" ").unwrap())
    .map(|(a,b)| Hand::new( a.as_bytes(), b.parse::<i64>().unwrap(), false))
    .sorted()
    .enumerate()
    //.inspect(|(i,h)| println!("i:{},h:{:?}",i+1,h))
    .map(|(i, hand)| (i as i64+1)*hand.bid as i64)
    .sum())
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    Ok(input.lines()
    .map(|s| s.split_once(" ").unwrap())
    .map(|(a,b)| Hand::new( a.as_bytes(), b.parse::<i64>().unwrap(), true))
    .sorted()
    .enumerate()
    // .inspect(|(i,h)| println!("i:{},h:{:?}",i+1,h))
    .map(|(i, hand)| (i as i64+1)*hand.bid as i64)
    .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&ÉNONCÉ).unwrap(), 6440);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&ÉNONCÉ).unwrap(), 5905);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input07.txt")).unwrap(), 248453531);
        assert_eq!(part2(include_str!("../inputs/input07.txt")).unwrap(), 248781813);//249101958 & 249050660
    }
}
