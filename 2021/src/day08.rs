use itertools::Itertools;
use std::{collections::HashMap, error::Error};

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let result = input.lines().fold(0, |acc, l| {
        let partie_droite = l.split_once('|').unwrap().1;
        acc + partie_droite
            .split_ascii_whitespace()
            .filter(|elt| matches!(elt.len(), 2 | 3 | 4 | 7))
            .count()
    });
    Ok(result as i64)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let digits = input
        .lines()
        .map(|ligne| {
            let g_d = ligne.split_once('|').unwrap();
            let digits_gauche = g_d.0.trim().split(' ').collect::<Vec<&str>>();
            let digits_droite = g_d.1.trim().split(' ').collect::<Vec<&str>>();
            (digits_gauche, digits_droite)
        })
        .collect::<Vec<_>>();

    let result: i64 = digits
        .iter()
        .map(|(gauche, droite)| {
            let mut tableau = vec![""; 10];
            tableau[1] = gauche.iter().find(|d| d.len() == 2).unwrap();
            tableau[4] = gauche.iter().find(|d| d.len() == 4).unwrap();
            tableau[7] = gauche.iter().find(|d| d.len() == 3).unwrap();
            tableau[8] = gauche.iter().find(|d| d.len() == 7).unwrap();
            tableau[6] = gauche.iter().find(|d| d.len() == 6 && !is_included_in(d, tableau[7])).unwrap();
            tableau[3] = gauche.iter().find(|d| d.len() == 5 && is_included_in(d, tableau[7])).unwrap();
            tableau[5] = gauche
                .iter()
                .find(|s| s.len() == 5 && !is_included_in(s, tableau[7]) && is_included_in(tableau[6], s))
                .unwrap();
            tableau[9] = gauche
                .iter()
                .find(|s| s.len() == 6 && is_included_in(s, tableau[7]) && is_included_in(s, tableau[5]))
                .unwrap();
            tableau[0] = gauche.iter().find(|s| s.len() == 6 && !tableau.contains(s)).unwrap();
            tableau[2] = gauche
                .iter()
                .find(|s| s.len() == 5 && !is_included_in(s, tableau[7]) && !is_included_in(tableau[9], s))
                .unwrap();

            let correspondances: HashMap<String, u8> = tableau.iter().enumerate().fold(HashMap::new(), |mut acc, (i, chaine)| {
                let key = chaine.chars().sorted().collect::<String>();
                acc.insert(key, i as u8);
                acc
            });
            let somme: i64 = droite.iter().fold(0, |acc, chaine| {
                let key = chaine.chars().sorted().collect::<String>();
                acc * 10 + correspondances[&key] as i64
            });
            somme
        })
        .sum();

    Ok(result)
}

/**
 * test si la chaine contient l'un des caractères de liste_char
 */
fn is_included_in(chaine: &str, liste_char: &str) -> bool {
    for ch in liste_char.chars() {
        if !chaine.contains(ch) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 61229);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input08.txt")).unwrap(), 543);
        assert_eq!(part2(include_str!("../inputs/input08.txt")).unwrap(), 994266);
    }
}
