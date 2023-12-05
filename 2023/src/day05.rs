use std::error::Error;

const MILIEUX: [&str; 7] = ["soil", "fertilizer", "water", "light", "temperature", "humidity", "location"];

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let (seeds, game)= input.split_once("\n\n").unwrap();
    let seeds: Vec<i64> = seeds
    .split_whitespace().skip(1)
    .inspect(|f| println!("{}", f))
    .map(|f|f.parse::<i64>().unwrap())
    .collect();

    let elements: Vec<Vec<[i64;3]>> = game.split("\n\n").fold(Vec::<Vec<[i64;3]>>::new(), |mut acc_elements, bloc|{
        let ligne: Vec<[i64;3]> = bloc.lines().skip(1).fold(Vec::<[i64;3]>::new(), |mut acc_ligne, line| {
            let rule : [i64;3] = line.split_whitespace().map(|c| c.parse::<i64>().unwrap()).collect::<Vec<i64>>().try_into().unwrap();
            acc_ligne.push(rule);
            acc_ligne
        });
        acc_elements.push(ligne);
        acc_elements
    });

    let mut locations: Vec<i64> = Vec::with_capacity(seeds.len());

    for seed in seeds {
        let mut elemnt_i = 0;
        let mut transform = seed;
        for element in &elements {

            for ligne in element {
                if transform >= ligne[1] && transform <= ligne[1]+ligne[2] {
                    transform = transform - ligne[1] + ligne[0];
                    println!("{} is in {:?} : {} = {}", transform, ligne, MILIEUX[elemnt_i], transform);
                    break;
                }
            }



            elemnt_i += 1;
        }
        locations.push(transform);
        println!("location seed {} : {}", seed, transform);
    }


    Ok(*locations.iter().min().unwrap())
}

/**
 * Part 2
 */
pub fn part2(_input: &str) -> Result<i64, Box<dyn Error>> { Ok(0) }

#[cfg(test)]
mod tests {
    use super::*;

    const ÉNONCÉ: &str ="\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    #[test]
    fn test_part1() {
        assert_eq!(part1(&ÉNONCÉ).unwrap(), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&ÉNONCÉ).unwrap(), 0);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input05.txt")).unwrap(), 111627841);
        // assert_eq!(part2(include_str!("../inputs/input05.txt")).unwrap(), 0);
    }
}
