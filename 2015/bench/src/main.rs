use std::{time::Duration, error::Error};

use util::common::*;

use took::{Timer, Took};
use itertools::Itertools;

const RUNS: usize = 200;

#[rustfmt::skip]
fn main() {
    println!("Benchmarking all days with {} runs...", RUNS);
    
    let runner: &[(fn(input: &str) -> _, &str)] = &[
        (day01::part1, include_str!("../../src/day01/src/input.txt")), (day01::part2, include_str!("../../src/day01/src/input.txt")),
        (day02::part1, include_str!("../../src/day02/src/input.txt")), (day02::part2, include_str!("../../src/day02/src/input.txt")),
        (day03::part1, include_str!("../../src/day03/src/input.txt")), (day03::part2, include_str!("../../src/day03/src/input.txt")),
        (day04::part1, include_str!("../../src/day04/src/input.txt")), (day04::part2, include_str!("../../src/day04/src/input.txt")),
        (day05::part1, include_str!("../../src/day05/src/input.txt")), (day05::part2, include_str!("../../src/day05/src/input.txt")),
        (day06::part1, include_str!("../../src/day06/src/input.txt")), (day06::part2, include_str!("../../src/day06/src/input.txt")),
        (day07::part1, include_str!("../../src/day07/src/input.txt")), (day07::part2, include_str!("../../src/day07/src/input.txt")),
        (day08::part1, include_str!("../../src/day08/src/input.txt")), (day08::part2, include_str!("../../src/day08/src/input.txt")),
        (day09::part1, include_str!("../../src/day09/src/input.txt")), (day09::part2, include_str!("../../src/day09/src/input.txt")),
        (day10::part1, include_str!("../../src/day10/src/input.txt")), (day10::part2, include_str!("../../src/day10/src/input.txt")),
        (day11::part1, include_str!("../../src/day11/src/input.txt")), (day11::part2, include_str!("../../src/day11/src/input.txt")),
        (day12::part1, include_str!("../../src/day12/src/input.txt")), (day12::part2, include_str!("../../src/day12/src/input.txt")),
        (day13::part1, include_str!("../../src/day13/src/input.txt")), (day13::part2, include_str!("../../src/day13/src/input.txt")),
        (day14::part1, include_str!("../../src/day14/src/input.txt")), (day14::part2, include_str!("../../src/day14/src/input.txt")),
        (day15::part1, include_str!("../../src/day15/src/input.txt")), (day15::part2, include_str!("../../src/day15/src/input.txt")),
        (day16::part1, include_str!("../../src/day16/src/input.txt")), (day16::part2, include_str!("../../src/day16/src/input.txt")),
        (day17::part1, include_str!("../../src/day17/src/input.txt")), (day17::part2, include_str!("../../src/day17/src/input.txt")),
        (day18::part1, include_str!("../../src/day18/src/input.txt")), (day18::part2, include_str!("../../src/day18/src/input.txt")),
        (day19::part1, include_str!("../../src/day19/src/input.txt")), (day19::part2, include_str!("../../src/day19/src/input.txt")),
        (day20::part1, include_str!("../../src/day20/src/input.txt")), (day20::part2, include_str!("../../src/day20/src/input.txt")),
        (day21::part1, include_str!("../../src/day21/src/input.txt")), (day21::part2, include_str!("../../src/day21/src/input.txt")),
        (day22::part1, include_str!("../../src/day22/src/input.txt")), (day22::part2, include_str!("../../src/day22/src/input.txt")),
        (day23::part1, include_str!("../../src/day23/src/input.txt")), (day23::part2, include_str!("../../src/day23/src/input.txt")),
        (day24::part1, include_str!("../../src/day24/src/input.txt")), (day24::part2, include_str!("../../src/day24/src/input.txt")),
        (day25::part1, include_str!("../../src/day25/src/input.txt")), (day25::part2, include_str!("../../src/day25/src/input.txt")),
    ];
    let times: Vec<_> = runner
        .iter()
        .enumerate()
        .map(|(i,j)| {
            display_msg_head(i+1);
            let resultat=((j.0 as fn(input: &str) -> Result<i64, Box<dyn Error>>)(j.1)).unwrap_or_default();
            display_msg_result(resultat);
            (
                (0..RUNS)
                    .map(|_| {
                        let took = Timer::new();
                        //appel de la fonction
                        match j.0(j.1) {
                            Ok(_) => (),
                            Err(err) => panic!("error in {}",err)
                        }
                        took.took().into_std()
                    })
                    .min()
                    .unwrap(),
                //retourne le résultat de la fonction, pour savoir si !=0 alors on pourra afficher le résultat
                resultat,
            )
        })
        .collect();

    let mut duree_totale: Duration = Duration::ZERO;
    println!("## Résultat des benchs");
    println!("| day   |   part 1  |   part 2  | \n|-------|----------:|----------:|");
    
    //check si le jour est terminé et si oui alors on affiche les 2 parts et on ajoute à la durée totale
    for (daynumber,(p1, p2)) in times.iter().tuples().enumerate() {
        let strday = format!("day{:0>2}",daynumber+1);
        if p1.1 != 0 && p2.1 != 0 {
            Took::from_std(p1.0).affiche_deux_parts(&Took::from_std(p2.0), &strday);
            duree_totale += p1.0 + p2.0;
        } else if p1.1 != 0 && p2.1 == 0 {
            Took::from_std(p1.0).affiche_part1(&strday);
            duree_totale += p1.0;
        } else {
            println!("|       |           |           |");
        }
    }

    println!("| total |           |{: >10}|", format!("**{}**",Took::from_std(duree_totale)));

}

pub trait ArrayDisplay {
    fn affiche_deux_parts(&self, deuxieme: &Took, description: &str);
    fn affiche_part1(&self, description: &str);
}

impl ArrayDisplay for Took {
    
    fn affiche_deux_parts(&self, deuxieme: &Took, description: &str) {
        //| day01 |  83.32 μs |  97.22 μs |
        println!("| {} |{: >10} |{: >10} |", description, self, deuxieme );
    }

    fn affiche_part1(&self, description: &str) {
        //| day01 |  83.32 μs |  97.22 μs |
        println!("| {} |{: >10} |{: >10} |", description, self, "");
    }
}