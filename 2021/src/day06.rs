use std::error::Error;

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> { Ok(fill_the_ocean(input, 80)) }

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> { Ok(fill_the_ocean(input, 256)) }

fn fill_the_ocean(input: &str, generation: i32) -> i64 {
    let mut banc_de_poissons = input
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .fold([0i64; 9], |mut tabl, index| {
            tabl[index as usize] += 1;
            tabl
        });
    println!("bancs de 🐟 au départ {:?}", banc_de_poissons);

    for _ in 0..generation {
        let petits = banc_de_poissons[0];
        for generation in 0..8 {
            banc_de_poissons[generation] = banc_de_poissons[generation + 1];
        }
        banc_de_poissons[6] += petits;
        banc_de_poissons[8] = petits;
    }
    println!("bancs de 🐟 à l'arrivée {:?}", banc_de_poissons);
    banc_de_poissons.iter().sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("3,4,3,1,2").unwrap(), 5934i64);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("3,4,3,1,2").unwrap(), 26984457539i64);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input06.txt")).unwrap(), 350149i64);
        assert_eq!(part2(include_str!("../inputs/input06.txt")).unwrap(), 1590327954513i64);
    }
}
