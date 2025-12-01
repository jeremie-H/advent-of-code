use std::{error::Error, time::Duration};

use adventofcode::common::*;
use colored::*;
use itertools::Itertools;
use took::{Timer, Took};

const RUNS: usize = 20;

#[rustfmt::skip]
fn main() {
    println!("Benchmarking all AOC days with {} runs...", format!("{}", RUNS).green().bold());
    
    let times: Vec<_> = alldays()
        .iter()
        .enumerate()
        .map(|(i,j)| {
            display_msg_head(i+1);
            let resultat=((j.0 as fn(input: &str) -> Result<i64, Box<dyn Error>>)(j.1)).unwrap_or_default();
            let tuple_duration_result = (
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
            );
            display_msg_result(resultat);
            tuple_duration_result
        })
        .collect();

    let mut duree_totale: Duration = Duration::ZERO;
    println!("## Benches results");
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
            println!("| {} |           |           |", strday);
        }
    }

    println!("|**Total**|           |{: >10} |", format!("**{}**",Took::from_std(duree_totale)));

}

pub trait ArrayDisplay {
    fn affiche_deux_parts(&self, deuxieme: &Took, description: &str);
    fn affiche_part1(&self, description: &str);
}

impl ArrayDisplay for Took {
    fn affiche_deux_parts(&self, deuxieme: &Took, description: &str) {
        //| day01 |  83.32 μs |  97.22 μs |
        println!("| {} |{: >10} |{: >10} |", description, self, deuxieme);
    }

    fn affiche_part1(&self, description: &str) {
        //| day01 |  83.32 μs |  97.22 μs |
        println!("| {} |{: >10} |{: >10} |", description, self, "");
    }
}
