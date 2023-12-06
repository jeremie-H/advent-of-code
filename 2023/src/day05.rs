use std::{error::Error, cmp::Ordering};

use itertools::Itertools;

const MILIEUX: [&str; 7] = ["soil", "fertilizer", "water", "light", "temperature", "humidity", "location"];

/**
 * Load les data
 */
fn load_data(input: &str) -> (Vec<i64>, Vec<Vec<[i64; 3]>>) {
    let (seeds, game)= input.split_once("\n\n").unwrap();
    let seeds: Vec<i64> = seeds
    .split_whitespace().skip(1)
    //.inspect(|f| println!("{}", f))
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
    (seeds, elements)
}

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let (seeds, elements) = load_data(input);
    let mut locations: Vec<i64> = Vec::with_capacity(seeds.len());
    for seed in seeds {
        // let mut elemnt_i = 0;
        let mut transform = seed;
        for element in &elements {
            for ligne in element {
                if transform >= ligne[1] && transform <= ligne[1]+ligne[2] {
                    transform = transform - ligne[1] + ligne[0];
                    //println!("{} is in {:?} : {} = {}", transform, ligne, MILIEUX[elemnt_i], transform);
                    break;
                }
            }
            // elemnt_i += 1;
        }
        locations.push(transform);
        //println!("location seed {} : {}", seed, transform);
    }
    Ok(*locations.iter().min().unwrap())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    min: i64,
    max: i64,
    origin: i64,
}
impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        self.min.cmp(&other.min)
    }
}
impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Range {
    
    //renvoie un tableau de range résultant de l'intersection entre self et other
    fn intersection(&self, other: &Range) -> (Option<Range>,Vec<Range>)  {
        let mut ranges_restants: Vec<Range> = Vec::new();
        let intersection: Option<Range>;
        //pas d'intersection
        if self.max < other.min || self.min > other.max {
            ranges_restants.push(*self);
            intersection = None;
        } else {
            //calcul du range "intersection" décalé par rapport à l'origine
            intersection = Some(Range {
                min: (self.min.max(other.min) - other.min) + other.origin,
                max: (self.max.min(other.max) - other.min) + other.origin,
                origin: other.origin,
            });

            // le reste "devant"
            if self.min < other.min {
                ranges_restants.push(Range {
                    min: self.min,
                    max: other.min,
                    origin: self.origin,
                });
            }
            // le reste "derrière"
            if other.max < self.max {
                ranges_restants.push(Range {
                    min: other.max,
                    max: self.max,
                    origin: self.origin,
                });
            }
        }
        (intersection,ranges_restants)
    }
    
}
    

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let (seeds, elements) = load_data(input);
    //on transforme les seeds en range
    let mut seeds_ranges: Vec<Range> = seeds.iter().tuples().map(|(&a,&b)| {
        let r = Range{min: a, max: b+a-1, origin: 0};
        //println!("Range {:?}", r);
        r
    }).collect::<Vec<Range>>();
    let mut gros_tableaux_des_ranges: Vec<Range> = Vec::new();
    let mut transform_avant_element: Vec<Range> = Vec::new();
    let mut transform_après_element: Vec<Range> = Vec::new();
    let mut transform_encours: Vec<Range> = Vec::new();
    let mut transform_encours2: Vec<Range> = Vec::new();

    //for seed_range in seeds_ranges {
        let mut elemnt_i = 0;
       
        transform_avant_element.append(&mut seeds_ranges);

        for element in &elements {
            
            

            for elt in &transform_avant_element {
                transform_encours.clear();
                transform_encours.push(*elt);
                let mut une_inter = false;
                for ligne in element {
                    transform_encours2.clear();
                    for tr_encours in &transform_encours {

                        let (inter, mut reste) = tr_encours.intersection(&Range{min: ligne[1], max: ligne[1]+ligne[2], origin: ligne[0]});
                        //println!("intersection {:?} avec {:?} : {:?}, reste : {:?}", tr_encours, ligne, inter, reste);
                        if let Some(r) = inter {
                            une_inter = true;
                            transform_après_element.push(r);
                            transform_encours2.append(&mut reste);
                        }
                        else {
                            transform_encours2.push(*tr_encours);
                        }
                    }
                    transform_encours.clear();
                    transform_encours.append(& mut transform_encours2);
                }
                transform_après_element.append(&mut transform_encours);
                 
                //println!("transform_encours : {:?}", transform_encours);

                // while !transform_encours.is_empty() {
                //     let elt = transform_encours.pop().unwrap();
                //         for ligne in element {
                //         let (inter, reste) = elt.intersection(&Range{min: ligne[1], max: ligne[1]+ligne[2], origin: ligne[0]});
                //         if let Some(r) = inter {
                //             transform_après_element.push(r);
                //         }
                //     }
                // }
                // transform_après_element.append(&mut transform_encours);
                // println!("fin element : {:?}", transform_après_element);
            }

            elemnt_i += 1;
            transform_avant_element.clear();
            transform_avant_element.append(&mut transform_après_element);
            //println!("transform_avant_element {:?}", transform_avant_element);
      //  }
    }
    

    let min = transform_avant_element.iter().min().unwrap();
    Ok(min.min)
}



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
        assert_eq!(part2(&ÉNONCÉ).unwrap(), 46);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("../inputs/input05.txt")).unwrap(), 111627841);
        assert_eq!(part2(include_str!("../inputs/input05.txt")).unwrap(), 69323688);
    }
}
