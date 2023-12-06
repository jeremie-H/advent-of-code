use std::{error::Error, collections::HashSet, ops::Div};

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    Ok(input.lines().map(|line| {
        let (_card, game) = line.split_once(": ").unwrap();
        worth(game)
    }).sum::<i64>())
}

fn worth(game: &str) -> i64 {
    let (winnings, game) = game.split_once(" | ").unwrap();
    let winnings = winnings.split(" ").filter(|c| !c.is_empty()).map(|w| w.trim().parse::<i64>().unwrap()).collect::<HashSet<i64>>();
    let game = game.split(" ").filter(|c| !c.is_empty()).map(|w| w.trim().parse::<i64>().unwrap()).collect::<HashSet<i64>>();
    (2  as i64).pow(game.intersection(&winnings).count() as u32).div(2)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let (_, total_sum) = input.lines().try_fold((vec![1i64; 256], 0), |(mut states, sum), line| -> Result<_, Box<dyn Error>> {
        let (card, game) = line.split_once(":").ok_or("Invalid input format")?;
        let card = card.split_once(" ").ok_or("Invalid card format").map(|(_, val)| val.trim().parse::<i64>())??;
        let (winnings, game) = game.split_once(" | ").ok_or("Invalid game format")?;

        let winnings = winnings
            .split_whitespace()
            .filter(|c| !c.is_empty())
            .map(|w| w.trim().parse::<i64>())
            .collect::<Result<HashSet<i64>, _>>()?;
        let game = game
            .split_whitespace()
            .filter(|c| !c.is_empty())
            .map(|w| w.trim().parse::<i64>())
            .collect::<Result<HashSet<i64>, _>>()?;

        let inter = game.intersection(&winnings).count();
        for i in 1..=inter {
            states[card as usize + i] += states[card as usize];
        }

        let s = states[card as usize];
        Ok((states, sum + s))
    })?;

    Ok(total_sum)
}

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
