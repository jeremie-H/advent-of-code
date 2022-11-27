use itertools::*;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

fn load_cities(input: &str) -> (HashMap<(&str, &str), i64>, HashSet<&str>) {
    let mut distances = HashMap::new();
    let mut cities = HashSet::new();
    input.lines().map(|l| l.split(' ').collect::<Vec<&str>>()).for_each(|words| match words[..] {
        [from, "to", to, "=", dist] => {
            distances.insert((from, to), dist.parse::<i64>().unwrap());
            distances.insert((to, from), dist.parse::<i64>().unwrap());
            cities.insert(from);
            cities.insert(to);
        }
        _ => panic!("weird line {:?}", words),
    });
    (distances, cities)
}

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let (distances, cities) = load_cities(input);
    let short: i64 = cities
        .iter()
        .permutations(cities.len())
        .map(|une_permutation| une_permutation.windows(2).map(|v| distances.get(&(*v[0], *v[1])).unwrap()).sum::<i64>())
        .min()
        .unwrap();

    Ok(short)
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let (distances, cities) = load_cities(input);
    let short: i64 = cities
        .iter()
        .permutations(cities.len())
        .map(|une_permutation| une_permutation.windows(2).map(|v| distances.get(&(*v[0], *v[1])).unwrap()).sum::<i64>())
        .max()
        .unwrap();

    Ok(short)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";

    #[test]
    fn test_part1() {
        assert_eq!(part1(ÉNONCÉ).unwrap(), 605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(ÉNONCÉ).unwrap(), 982);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input09.txt")).unwrap(), 117);
        assert_eq!(part2(include_str!("../inputs/input09.txt")).unwrap(), 909);
    }
}
