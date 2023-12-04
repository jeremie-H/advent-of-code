use std::{error::Error, ops::{RangeBounds, Range}};


/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> { Ok(somme_des_nombres(input)) }

const CHIFFRES: [&str; 9] = ["one", "two","three","four","five","six","seven","eight","nine"];

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    
    Ok(input.lines()
    .map(|line| calculate_line_value(line))
    .sum::<i64>())

}

fn calculate_line_value(line: &str) -> i64 {
    let b_ligne = line.as_bytes();
    let dizaine = calcul_valeur(b_ligne, 0..b_ligne.len());
    let unité = calcul_valeur(b_ligne, (0..b_ligne.len()).rev());
    
    dizaine * 10 + unité
}

fn calcul_valeur(ligne: &[u8], range: impl Iterator<Item = usize>) -> i64 {
    for i in range {
        if ligne[i].is_ascii_digit() {
            return (ligne[i] - b'0') as i64;
        }
        for (num, name) in CHIFFRES.iter().enumerate() {
            let name_bytes = name.as_bytes();
            if ligne[i..].starts_with(name_bytes) {
                return (num + 1) as i64;
            }
        }
    }
    0
}


fn somme_des_nombres(lines: &str) -> i64 {
    lines.lines().fold(0, |acc, l| {
        let nombre = l.chars().filter(|c| c.is_numeric()).map(|c| (c as u8) - b'0').fold(Vec::new(), |mut acc, entier| {
            acc.push(entier);
            acc
        });
        let nombre = 10 * nombre[0] + nombre[nombre.len() - 1];
        acc + nombre as i64
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet").unwrap(), 142);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen").unwrap(),
            281
        );
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input01.txt")).unwrap(), 54630);
        assert_eq!(part2(include_str!("../inputs/input01.txt")).unwrap(), 54770);
    }
}
