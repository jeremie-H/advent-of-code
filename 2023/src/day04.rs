use std::{error::Error, collections::HashSet, ops::Div};

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    Ok(input.lines().map(|line| {
        let (card, game) = line.split_once(": ").unwrap();
        worth(card, game)
    }).sum::<i64>())
}

fn worth(card: &str, game: &str) -> i64 {
    let (winnings, game) = game.split_once(" | ").unwrap();
    let winnings = winnings.split(" ").filter(|c| !c.is_empty()).map(|w| w.trim().parse::<i64>().unwrap()).collect::<HashSet<i64>>();
    // println!("winnings {:?}", winnings);
    let game = game.split(" ").filter(|c| !c.is_empty()).map(|w| w.trim().parse::<i64>().unwrap()).collect::<HashSet<i64>>();
    // println!("game : {:?}", game);

    // println!("intersection : {:?}", game.intersection(&winnings));
    (2  as i64).pow(game.intersection(&winnings).count() as u32).div(2)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    Ok(input.lines().fold((vec![1i64;256], 0), |(mut states, somme), line| {
        let (card, game) = line.split_once(": ").unwrap();
        let card = card.split_once(" ").unwrap().1.trim().parse::<i64>().unwrap();
        let (winnings, game) = game.split_once(" | ").unwrap();

        let winnings = winnings.split(" ").filter(|c| !c.is_empty()).map(|w| w.trim().parse::<i64>().unwrap()).collect::<HashSet<i64>>();
        // println!("winnings {:?}", winnings);
        let game = game.split(" ").filter(|c| !c.is_empty()).map(|w| w.trim().parse::<i64>().unwrap()).collect::<HashSet<i64>>();
        // println!("game : {:?}", game);
        let inter = game.intersection(&winnings).count();
        for i in 1..=inter {
            states[card as usize+i] += states[card as usize];
        };
        
        let worth = (2  as i64).pow(inter as u32).div(2);
        println!("card : {:?}, intersection : {:?}, worth : {:?}, state[i]: {:?}", card, game.intersection(&winnings), worth, states[card as usize]);
        let s = states[card as usize];
        (states, somme + s )
    }).1)
}

// fn worth_with_states(card: i64, game: &str, states: &mut [i64]) -> i64 {
    
// }

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&ÉNONCÉ).unwrap(), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&ÉNONCÉ).unwrap(), 30);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input04.txt")).unwrap(), 24542);
        assert_eq!(part2(include_str!("../inputs/input04.txt")).unwrap(), 8736438);
    }
}
