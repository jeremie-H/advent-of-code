use std::error::Error;

use itertools::Itertools;


#[derive(Debug, Clone, Copy)]
struct Galaxie {
    x: i64,
    y: i64
}

impl Galaxie {
    fn distance(&self, other: &Galaxie) -> i64 {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        dx + dy
    }

    fn s_ecarte_de(&self, b: Galaxie, lignes_vides: &[i64], colonnes_vides: &[i64], multiple_vide: i64) -> i64 {
        let distance = self.distance(&b);
        let mut vides = lignes_vides.iter()
        .filter(|&&ligne| ligne > self.y.min(b.y) && ligne < self.y.max(b.y))
        .count() as i64;
        
        vides += colonnes_vides.iter()
        .filter(|&&colonne| colonne > self.x.min(b.x) && colonne < self.x.max(b.x))
        .count() as i64;
        
        distance + vides * (multiple_vide-1)
    }   
}

/**
 * Load les data
 * param input
 * return (galaxies, lignes_vides, colonnes_vides)
 * 
 */
fn load_data(input: &str) -> (Vec<Galaxie>, Vec<i64>, Vec<i64>) {
    let mut galaxies = Vec::<Galaxie>::new();
    let taille = input.split_once('\n').unwrap().0.len();
    let mut vides = vec![0; taille];
    input.lines().enumerate().for_each(|(j, ligne)| {
        ligne
            .chars()
            .enumerate()
            .for_each(|(i, c)| {
                if c == '#' {
                    galaxies.push(Galaxie{x: i as i64, y: j as i64});
                    vides[j] |= 2;
                    vides[i] |= 1;
                }
            });
    });

    let mut lignes: Vec<i64> = Vec::new();
    let mut colonnes: Vec<i64> = Vec::new();
    
    vides.iter().enumerate()
    .for_each(|(i, v)| {
        if *v & 2 == 0 {
            lignes.push(i as i64);
        }
        if *v & 1 == 0 {
            colonnes.push(i as i64);
        }
    });

    (galaxies, lignes, colonnes)
}

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let (galaxies, lignes_vides, colonnes_vides) = load_data(input);
    let somme = galaxies.iter().tuple_combinations::<(_, _)>()
    .map(|(&a, &b)| a.s_ecarte_de(b, &lignes_vides, &colonnes_vides, 2))
    .sum::<i64>();
    Ok(somme)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let (galaxies, lignes_vides, colonnes_vides) = load_data(input);
    Ok(galaxies.iter().tuple_combinations::<(_, _)>()
    .map(|(&a, &b)| a.s_ecarte_de(b, &lignes_vides, &colonnes_vides, 1_000_000))
    .sum::<i64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&ÉNONCÉ).unwrap(), 374);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&ÉNONCÉ).unwrap(), 82000210);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input11.txt")).unwrap(), 9545480);
        assert_eq!(part2(include_str!("../inputs/input11.txt")).unwrap(), 406725732046);
    }
}
